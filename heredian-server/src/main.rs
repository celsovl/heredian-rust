use std::net::{SocketAddr};
use std::path::{Path};
use std::time::{Duration};
use std::thread;

use heredian_lib::*;
use heredian_lib::file_manager::*;
use heredian_lib::allegro_safe::{
    al_load_bitmap, al_init, al_init_image_addon, 
    al_map_rgb, al_get_pixel, AlBitmap,
    al_get_bitmap_width, al_get_bitmap_height
};
use heredian_lib::net::*;

struct Ambients {
    width: i16,
    height: i16,
    boss_num: i16,
    models: Vec<*const AlBitmap>,
    enemies: Vec<PacketCharInfo>,
    clients: Vec<PacketCharInfo>,
    clients_addrs: Vec<SocketAddr>,
    last_id: i16,
}

impl Ambients {
    fn load() -> Ambients {
        let path = Path::new("assets/Configs/Config.txt");
        let config_file = ConfigFile::load(path);

        let width = config_file.get("width").expect("width not found.");
        let height = config_file.get("height").expect("height not found.");
        let boss_num = config_file.get("boss_num").expect("boss_num not found.");

        let path = Path::new("assets/Configs/Ambients.txt");
        let ambient_config_file = ConfigFile::load(path);

        let qt_maps = ambient_config_file.get("qt_maps").expect("qt_maps not found.");
        let mut models = Vec::with_capacity(qt_maps);
        
        for i in 1..=qt_maps {
            let key = format!("map{}", i);
            let model_path = ambient_config_file.get_string(&key).expect(&(key + " not found."));
            models.push(al_load_bitmap(model_path));
        }

        let enemies = Self::load_enemies();
        let last_id = enemies.len() as i16;

        Ambients {
            width: width,
            height: height,
            boss_num: boss_num,
            models: models,
            enemies: enemies,
            clients: Vec::with_capacity(4),
            clients_addrs: Vec::with_capacity(4),
            last_id: last_id,
        }
    }

    fn load_enemies() -> Vec<PacketCharInfo> {
        let path = Path::new("assets/Configs/Enemies.txt");
        let config_file = ConfigFile::load(path);

        let path = Path::new("assets/Configs/EnemiesConf.txt");
        let enemies_config_file = ConfigFile::load(path);

        let qt_inimigos = config_file.get("qt_inimigos").expect("qt_inimigos not found.");
        let mut enemies = Vec::with_capacity(qt_inimigos);

        for i in 1..=qt_inimigos {
            enemies.push(PacketCharInfo {
                x: config_file.get(&format!("{}_x", i)).expect("x not found"),
                y: config_file.get(&format!("{}_y", i)).expect("y not found"),
                a: config_file.get(&format!("{}_a", i)).expect("a not found"),
                d: config_file.get(&format!("{}_d", i)).expect("d not found"),
                numchar: config_file.get(&format!("{}_num", i)).expect("num not found"),
                vision: enemies_config_file.get(&format!("{}vision", i)).unwrap_or(0),
                step: enemies_config_file.get(&format!("{}step", i)).unwrap_or(0),
                damage: enemies_config_file.get(&format!("{}damage", i)).unwrap_or(0),
                idchar: i as i16,
                exit: false,
                healt: config_file.get(&format!("{}_helt", i)).expect("helt not found"),
                stamina: config_file.get(&format!("{}_stamina", i)).expect("stamina not found"),
                idmap: config_file.get(&format!("{}_idmap", i)).expect("idmap not found"),
                w: config_file.get(&format!("{}_width", i)).expect("width not found"),
                h: config_file.get(&format!("{}_height", i)).expect("height not found"),
                ..PacketCharInfo::default()
            });
        }

        enemies
    }

    fn send_direct_enemies(&mut self, server: &Server<PacketCharInfo>, addr: SocketAddr) {
        let len_enemies = self.enemies.len();

        for enemy in self.enemies.iter_mut() {
            enemy.totchar = self.clients.len() as i16;
            enemy.totenemies = len_enemies as i16;
            server.send(Message::Direct(enemy.clone(), addr));
        }
    }

    fn broadcast_enemies(&mut self, server: &Server<PacketCharInfo>) {
        let len_enemies = self.enemies.len();

        for enemy in self.enemies.iter_mut() {
            enemy.totchar = self.clients.len() as i16;
            enemy.totenemies = len_enemies as i16;
            server.send(Message::Broadcast(enemy.clone()));
        }
    }
}

fn hit(packet: &mut PacketCharInfo, tx: i16, ty: i16, td: i16, damage: i16) {
    let (x1, y1, x2, y2) = (packet.x, packet.y, packet.x + packet.w, packet.y + packet.h);

    if (x1..=x2).contains(&tx) && (y1..=y2).contains(&ty) {
        packet.healt -= damage;
        packet.dhit = td;

        match td as i32 {
            GDPUP => packet.y -= 3,
            GDPDOWN => packet.y += 3,
            GDPLEFT => packet.x -= 3,
            GDPRIGHT => packet.y += 3,
            _ => unreachable!()
        }
    }
}

fn disconnect_client(ambients: &mut Ambients, addr: SocketAddr, server: &Server<PacketCharInfo>) {
    let idx = ambients.clients_addrs.iter().position(|a| *a == addr);

    if let Some(idx) = idx {
        ambients.clients_addrs.remove(idx);
        ambients.clients.remove(idx);
    }
}

fn connect_client(ambients: &mut Ambients, addr: SocketAddr, server: &Server<PacketCharInfo>) {
    ambients.last_id += 1;

    let packet = PacketCharInfo {
        idchar: ambients.last_id,
        totchar: (ambients.clients.len() + 1) as i16,
        ..PacketCharInfo::default()
    };

    server.send(Message::Direct(packet.clone(), addr));
    ambients.clients_addrs.push(addr);
    ambients.clients.push(packet);
    ambients.send_direct_enemies(server, addr);
}

fn on_message(ambients: &mut Ambients, packet: PacketCharInfo, addr: SocketAddr, server: &Server<PacketCharInfo>) {
    let len_clients = ambients.clients.len();

    let pos_char = ambients.clients.iter().position(|c| c.idchar == packet.idchar).expect("idchar not found - not connected.");
    
    // put this char at the front of the list
    ambients.clients.swap(0, pos_char);
    ambients.clients_addrs.swap(0, pos_char);

    // split the list 
    let (this_char, others_chars) = ambients.clients.split_first_mut().unwrap();

    println!("Char {} went from map {} to {}", this_char.idchar, packet.idmap, this_char.idmap);
    
    this_char.totchar = len_clients as i16;
    this_char.totenemies = ambients.enemies.len() as i16;
    this_char.idmap = packet.idmap;

    if this_char.healt == 0 {
        this_char.healt = packet.healt;
        this_char.x = packet.x;
        this_char.y = packet.y;
        this_char.numchar = packet.numchar;
    }

    damage_char(this_char, others_chars);
    damage_char(this_char, ambients.enemies.as_mut_slice());

    let mut lifeless_char = PacketCharInfo::default();

    for lifeless in this_char.listlifeless.iter() {
        if let Some(lifeless) = lifeless {
            lifeless_char.x = lifeless.x;
            lifeless_char.y = lifeless.y;
            lifeless_char.w = lifeless.w;
            lifeless_char.h = lifeless.h;
            lifeless_char.d = lifeless.d;
            lifeless_char.damage = lifeless.damage;

            damage_char(&lifeless_char, others_chars);
            damage_char(&lifeless_char, ambients.enemies.as_mut_slice());        
        }
    }

    this_char.healt = this_char.healt.max(0);
    
    server.send(Message::Broadcast(this_char.clone()));
}

fn dir_damage(this_char: &PacketCharInfo, other_char: &mut PacketCharInfo) {
    if rand::random::<i16>() % 10 + 1 < 5 {
        return;
    }

    let (x1, y1, x2, y2) = (this_char.x, this_char.y, this_char.x + this_char.w, this_char.y + this_char.h);
    let (xm, ym) = ((x1+x2)/2, (y1+y2)/2);

    match this_char.d as i32 {
        GDPUP => hit(other_char, xm, y1, this_char.d, this_char.damage),
        GDPDOWN => hit(other_char, xm, y2, this_char.d, this_char.damage),
        GDPLEFT => hit(other_char, x1, ym, this_char.d, this_char.damage),
        GDPRIGHT => hit(other_char, x2, ym, this_char.d, this_char.damage),
        _ => unreachable!()
    }
}

fn damage_char(this_char: &PacketCharInfo, others_chars: &mut [PacketCharInfo]) {
    if this_char.damage > 0 {
        for other in others_chars {
            if this_char.idmap == other.idmap {
                dir_damage(this_char, other);
            }
        }
    }
}

fn recv_once(ambients: &mut Ambients, server: &Server<PacketCharInfo>) {
    while let Ok(msg) = server.try_recv() {
        match msg {
            Message::Accepted(addr) => connect_client(ambients, addr, server),
            Message::Disconnected(addr) => disconnect_client(ambients, addr, server),
            Message::Direct(packet, addr) => on_message(ambients, packet, addr, server),
            _ => unreachable!()
        }
    }
}

fn distance(u: &PacketCharInfo, v: &PacketCharInfo) -> f32 {
    ((u.x - v.x) as f32).hypot((u.y - v.y) as f32)
}

fn game_loop(ambients: &mut Ambients, server: &Server<PacketCharInfo>) {
    let mut lock = 0;
    let (width, height) = (ambients.width, ambients.height);
    
    loop {
        // remove dead enemies
        ambients.enemies.retain(|e| !e.exit);

        let len_chars = ambients.clients.len() as i16;
        let len_enemies = ambients.enemies.len() as i16;

        // update dead enemies' info
        for enemy in ambients.enemies.iter_mut() {
            let mut should_send = true;
            let ambient_data = (width, height, ambients.models[(enemy.idmap-1) as usize]);

            if enemy.healt <= 0 {
                enemy.exit = true;
            } else {
                // detect nearest char to attack
                let nearest_client = ambients
                                        .clients
                                        .iter_mut()
                                        .filter(|c| c.idmap == enemy.idmap)
                                        .map(|c| {
                                            let dist = distance(c, enemy);
                                            (c, dist)
                                        })
                                        .max_by(|c1, c2| c1.1.partial_cmp(&c2.1).unwrap());
                
                if let Some((client, dist)) = nearest_client {
                    if dist <= enemy.vision as f32 {
                        enemy.a = 1;

                        let dx = ((client.x - enemy.x) as f32).abs() as i16;
                        let dy = ((client.y - enemy.y) as f32).abs() as i16;

                        // check if this enemy hit this client
                        let x2 = enemy.x + enemy.w;
                        let y2 = enemy.y + enemy.h;

                        let c_x2 = client.x + client.w;
                        let c_y2 = client.y + client.h;

                        if enemy.x <= c_x2 && client.x <= x2 &&
                            enemy.y <= c_y2 && client.y <= y2 {
                            dir_damage(enemy, client);
                        }

                        // boss moves differently
                        if enemy.numchar == ambients.boss_num {
                            if dx < enemy.step + enemy.w && enemy.x < client.x ||
                                dy < enemy.step + enemy.h && enemy.y < client.y {
                                
                                if lock > 0 {
                                    lock -= 1;
                                    should_send = false;
                                } else {
                                    lock = 20;
                                    enemy.a = 3;

                                    if dx >= dy {
                                        if enemy.x < client.x {
                                            enemy.d = GDPRIGHT as i16;
                                        } else {
                                            enemy.d = GDPLEFT as i16;
                                        }
                                    } else {
                                        if enemy.y < client.y {
                                            enemy.d = GDPDOWN as i16;
                                        } else {
                                            enemy.d = GDPUP as i16;
                                        }
                                    }
                                }
                            } else {
                                if lock > 0 {
                                    lock -= 1;
                                    should_send = false;
                                } else {
                                    lock = 20;
                                    enemy.a = 3;

                                    if dx >= dy {
                                        if enemy.x < client.x {
                                            enemy.x += enemy.step;
                                            enemy.d = GDPRIGHT as i16;
                                        } else {
                                            enemy.x -= enemy.step;
                                            enemy.d = GDPLEFT as i16;
                                        }
                                    } else {
                                        if enemy.y < client.y {
                                            enemy.y += enemy.step;
                                            enemy.d = GDPDOWN as i16;
                                        } else {
                                            enemy.y -= enemy.step;
                                            enemy.d = GDPUP as i16;
                                        }
                                    }
                                }
                            }
                        } else {
                            // checks move over x-axis
                            if enemy.x < client.x {
                                enemy.x += enemy.step;
                                enemy.d = GDPRIGHT as i16;

                                if enemy_collided(enemy, ambient_data) {
                                    enemy.x -= enemy.step-1;
                                }

                                if dx > dy {
                                    enemy.d = GDPLEFT as i16;
                                } else {
                                    if enemy.y < client.y {
                                        enemy.d = GDPDOWN as i16;
                                    } else {
                                        enemy.d = GDPUP as i16;
                                    }
                                }
                            } else {
                                if dx > enemy.step * 2 {
                                    enemy.x -= enemy.step;
                                }
                                
                                enemy.d = GDPLEFT as i16;

                                if enemy_collided(enemy, ambient_data) {
                                    enemy.x += enemy.step * 2;
                                }

                                if dx > dy {
                                    enemy.d = GDPLEFT as i16;
                                } else {
                                    if enemy.y < client.y {
                                        enemy.d = GDPDOWN as i16;
                                    } else {
                                        enemy.d = GDPUP as i16;
                                    }
                                }
                            }

                            // checks move over y-axis
                            if enemy.y < client.y {
                                enemy.y += enemy.step;
                                enemy.d = GDPDOWN as i16;

                                if enemy_collided(enemy, ambient_data) {
                                    enemy.y -= enemy.step * 2;
                                }
        
                                if dy > dx {
                                    enemy.d = GDPDOWN as i16;
                                } else {
                                    if enemy.x < client.x {
                                        enemy.d = GDPRIGHT as i16;
                                    } else {
                                        enemy.d = GDPLEFT as i16;
                                    }
                                }
                            } else {
                                if dy > enemy.step * 2 {
                                    enemy.y -= enemy.step;
                                }
        
                                enemy.d = GDPUP as i16;

                                if enemy_collided(enemy, ambient_data) {
                                    enemy.y += enemy.step * 2;
                                }
        
                                if dy > dx {
                                    enemy.d = GDPUP as i16;
                                } else {
                                    if enemy.x < client.x {
                                        enemy.d = GDPRIGHT as i16;
                                    } else {
                                        enemy.d = GDPLEFT as i16;
                                    }
                                }
                            }
                        }
                    } else {
                        enemy.a = 0;
                    }
                } else {
                    should_send = false;
                }
            }

            if should_send {
                enemy.totchar = len_chars;
                enemy.totenemies = len_enemies;
                server.send(Message::Broadcast(enemy.clone()));
            }
        }

        recv_once(ambients, server);

        thread::sleep(Duration::from_millis(80));
    }
}

fn enemy_collided(enemy: &PacketCharInfo, ambients_data: (i16, i16, *const AlBitmap)) -> bool {
    let (width, height, model) = ambients_data;

    if enemy.y < 0 || enemy.x < 0 {
        return true;
    }

    if (enemy.y + enemy.h) > height as i16 {
        return true;
    }

    let colorwall = al_map_rgb(0, 0, 0);

    let sx = al_get_bitmap_width(model) as f32 / width as f32;
    let sy = al_get_bitmap_height(model) as f32 / height as f32;

    let we = width as f32 * sx;
    let he = height as f32 * sy;

    let xup   = (enemy.x + enemy.w) as f32 * sx;
    let yup   = enemy.y as f32 * sy;

    let xdown = enemy.x as f32 * sx;
    let ydown = (enemy.y + enemy.h) as f32 * sy;

    if xdown >= 0.0 && ydown >= 0.0 && xup <= we && yup <= he {
        if enemy.d != GDPRIGHT as i16 {
            let color = al_get_pixel(model, xdown as i32, ydown as i32);
            if colorwall == color {
                return true;
            }
        }

        if enemy.d != GDPLEFT as i16 {
            let color = al_get_pixel(model, xup as i32, ydown as i32);
            if colorwall == color {
                return true;
            }
        }
    } else {
        return true;
    }

    return false;
}

fn main() {
    al_init();
    al_init_image_addon();

    let mut server = Server::new();
    server.listen(34000);

    let mut ambients = Ambients::load();
    println!("Heredian Server");
    game_loop(&mut ambients, &server);
}
