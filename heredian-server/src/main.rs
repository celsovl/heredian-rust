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
            let numchar = config_file.get(&format!("{}_num", i)).expect("num not found");
            enemies.push(PacketCharInfo {
                x: config_file.get(&format!("{}_x", i)).expect("x not found"),
                y: config_file.get(&format!("{}_y", i)).expect("y not found"),
                a: config_file.get(&format!("{}_a", i)).expect("a not found"),
                d: config_file.get(&format!("{}_d", i)).expect("d not found"),
                numchar: numchar,
                vision: enemies_config_file.get(&format!("{}vision", numchar)).unwrap_or(0),
                step: enemies_config_file.get(&format!("{}step", numchar)).unwrap_or(0),
                damage: enemies_config_file.get(&format!("{}damage", numchar)).unwrap_or(0),
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
}

fn hit(packet: &mut PacketCharInfo, tx: i16, ty: i16, td: i16, damage: i16, ambient_data: (i16, i16, *const AlBitmap)) -> bool {
    const DISPLACEMENT: i16 = 3;

    let (x1, y1, x2, y2) = (packet.x, packet.y, packet.x + packet.w, packet.y + packet.h);

    if (x1..=x2).contains(&tx) && (y1..=y2).contains(&ty) {
        packet.healt -= damage;
        packet.dhit = td;

        packet.healt = packet.healt.max(0);
        packet.exit = packet.healt == 0;

        let mov = match td as i32 {
                    GDPUP => (0, -DISPLACEMENT),
                    GDPDOWN => (0, DISPLACEMENT),
                    GDPLEFT => (-DISPLACEMENT, 0),
                    GDPRIGHT => (DISPLACEMENT, 0),
                    _ => unreachable!()
                };

        println!("id {} d {} mov {:?}", packet.idchar, td, mov);

        if mov.0 != 0 {
            packet.x += mov.0;
            if collided(packet, ambient_data) {
                packet.x -= mov.0;
            }
        } else {
            packet.y += mov.1;
            if collided(packet, ambient_data) {
                packet.y -= mov.1;
            }
        }

        true
    } else {
        false
    }
}

fn disconnect_client(ambients: &mut Ambients, addr: SocketAddr, _server: &Server<PacketCharInfo>) {
    let idx = ambients.clients_addrs.iter().position(|a| *a == addr);

    if let Some(idx) = idx {
        ambients.clients_addrs.remove(idx);
        ambients.clients.remove(idx);
    }
}

fn connect_client(ambients: &mut Ambients, addr: SocketAddr, server: &Server<PacketCharInfo>) {
    ambients.last_id += 1;

    let mut packet = PacketCharInfo {
        idchar: ambients.last_id,
        totchar: (ambients.clients.len() + 1) as i16,
        ..PacketCharInfo::default()
    };

    server.send(Message::Direct(packet.clone(), addr));
    ambients.clients_addrs.push(addr);

    packet.x = -1;
    packet.y = -1;

    ambients.clients.push(packet);
    ambients.send_direct_enemies(server, addr);
}

fn on_message(ambients: &mut Ambients, packet: PacketCharInfo, _addr: SocketAddr, server: &Server<PacketCharInfo>) {
    let len_clients = ambients.clients.len();
    let ambient_data = (ambients.width, ambients.height, ambients.models[packet.idmap as usize]);

    let pos_char = ambients.clients.iter().position(|c| c.idchar == packet.idchar).expect("idchar not found - not connected.");
    
    // put this char at the front of the list
    ambients.clients.swap(0, pos_char);
    ambients.clients_addrs.swap(0, pos_char);

    // split the list 
    let (this_char, _) = ambients.clients.split_first_mut().unwrap();

    this_char.totchar = len_clients as i16;
    this_char.totenemies = ambients.enemies.len() as i16;
    this_char.idmap = packet.idmap;

    if this_char.healt == 0 {
        this_char.healt = packet.healt;
        this_char.numchar = packet.numchar;
    }

    if this_char.x == -1 && this_char.y == -1 {
        this_char.x = packet.x;
        this_char.y = packet.y;
    }

    this_char.w = packet.w;
    this_char.h = packet.h;
    this_char.d = packet.d;
    this_char.a = packet.a;
    this_char.dhit = packet.dhit;
    this_char.damage = packet.damage;
    this_char.step = packet.step;

    //damage_char(this_char, others_chars, ambient_data);
    damage_char(this_char, ambients.enemies.as_mut_slice(), ambient_data, server);

    let mut lifeless_char = PacketCharInfo::default();

    for lifeless in this_char.listlifeless.iter() {
        if let Some(lifeless) = lifeless {
            lifeless_char.x = lifeless.x;
            lifeless_char.y = lifeless.y;
            lifeless_char.w = lifeless.w;
            lifeless_char.h = lifeless.h;
            lifeless_char.d = lifeless.d;
            lifeless_char.damage = lifeless.damage;

            //damage_char(&lifeless_char, others_chars, ambient_data, server);
            damage_char(&lifeless_char, ambients.enemies.as_mut_slice(), ambient_data, server);        
        }
    }

    this_char.healt = this_char.healt.max(0);

    server.send(Message::Broadcast(this_char.clone()));
}

fn dir_damage_chance(this_char: &PacketCharInfo, other_char: &mut PacketCharInfo, odds: f32, ambient_data: (i16, i16, *const AlBitmap)) -> bool {
    let res = 
        if rand::random::<f32>() <= odds {
            dir_damage(this_char, other_char, ambient_data)
        } else {
            false
        };

    res
}

fn dir_damage(this_char: &PacketCharInfo, other_char: &mut PacketCharInfo, ambient_data: (i16, i16, *const AlBitmap)) -> bool {
    let (x1, y1, x2, y2) = (this_char.x, this_char.y, this_char.x + this_char.w, this_char.y + this_char.h);
    let (xm, ym) = ((x1+x2)/2, (y1+y2)/2);

    match this_char.d as i32 {
        GDPUP => hit(other_char, xm, y1, this_char.d, this_char.damage, ambient_data),
        GDPDOWN => hit(other_char, xm, y2, this_char.d, this_char.damage, ambient_data),
        GDPLEFT => hit(other_char, x1, ym, this_char.d, this_char.damage, ambient_data),
        GDPRIGHT => hit(other_char, x2, ym, this_char.d, this_char.damage, ambient_data),
        _ => unreachable!()
    }
}

fn damage_char(this_char: &PacketCharInfo, others_chars: &mut [PacketCharInfo], ambient_data: (i16, i16, *const AlBitmap), server: &Server<PacketCharInfo>) {
    if this_char.damage > 0 {
        for other in others_chars {
            if this_char.idmap == other.idmap {
                if dir_damage_chance(this_char, other, 1.0, ambient_data) {
                    server.send(Message::Broadcast(other.clone()));
                }
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
    let uc = (u.x as f32 + u.w as f32 / 2.0, u.y as f32 + u.h as f32 / 2.0);
    let vc = (v.x  as f32 + v.w as f32 / 2.0, v.y as f32 + v.h as f32 / 2.0);

    (uc.0 - vc.0).hypot(uc.1 - vc.1)
}

fn move_enemy(enemy: &mut PacketCharInfo, client: &PacketCharInfo, ambient_data: (i16, i16, *const AlBitmap)) {
    let dx = (client.x + client.w) as f32/2.0 - (enemy.x + enemy.w) as f32/2.0;
    let dy = (client.y + client.h) as f32/2.0 - (enemy.y + enemy.h) as f32/2.0;

    //println!("client ({}): {}, {}; enemy ({}): {}, {}", client.idchar, client.x, client.y, enemy.idchar, enemy.x, enemy.y);

    let theta = (dy/dx).atan();
    let theta =
        match (dx, dy) {
            (x, y) if x >= 0.0 && y >= 0.0 => theta,
            (x, _) if x < 0.0 => std::f32::consts::PI + theta,
            (x, y) if x >= 0.0 && y < 0.0 => 2.0 * std::f32::consts::PI + theta,
            _ => unreachable!(),
        }.sin_cos();

    let (mov_x, mov_y) = ((enemy.step as f32 * theta.1), (enemy.step as f32 * theta.0));

    let final_d = if mov_x.abs() > mov_y.abs() {
        if mov_x >= 0.0 { GDPRIGHT } else { GDPLEFT }
    } else {
        if mov_y >= 0.0 { GDPDOWN } else { GDPUP }
    } as i16;

    let (mov_x, mov_y) = (mov_x.round() as i16, mov_y.round() as i16);

    enemy.x += mov_x;
    enemy.d = if mov_x >= 0 {
                GDPRIGHT
            } else {
                GDPLEFT
            } as i16;

    if collided(enemy, ambient_data) {
        enemy.x -= mov_x;
    }

    enemy.y += mov_y;
    enemy.d = if mov_y >= 0 {
                GDPDOWN
            } else {
                GDPUP
            } as i16;

    if collided(enemy, ambient_data) {
        enemy.y -= mov_y;
    }

    enemy.d = final_d;
}

fn move_boss(boss: &mut PacketCharInfo, client: &PacketCharInfo, lock: &mut i32) -> bool {
    let mut should_send = true;

    let dx = ((boss.x - client.x) as f32).abs() as i16;
    let dy = ((boss.y - client.y) as f32).abs() as i16;

    if dx < boss.step + boss.w && boss.x < client.x ||
        dy < boss.step + boss.h && boss.y < client.y {
        
        if *lock > 0 {
            *lock -= 1;
            should_send = false;
        } else {
            *lock = 20;
            boss.a = 3;

            if dx >= dy {
                if boss.x < client.x {
                    boss.d = GDPRIGHT as i16;
                } else {
                    boss.d = GDPLEFT as i16;
                }
            } else {
                if boss.y < client.y {
                    boss.d = GDPDOWN as i16;
                } else {
                    boss.d = GDPUP as i16;
                }
            }
        }
    } else {
        if *lock > 0 {
            *lock -= 1;
            should_send = false;
        } else {
            *lock = 20;
            boss.a = 3;

            if dx >= dy {
                if boss.x < client.x {
                    boss.x += boss.step;
                    boss.d = GDPRIGHT as i16;
                } else {
                    boss.x -= boss.step;
                    boss.d = GDPLEFT as i16;
                }
            } else {
                if boss.y < client.y {
                    boss.y += boss.step;
                    boss.d = GDPDOWN as i16;
                } else {
                    boss.y -= boss.step;
                    boss.d = GDPUP as i16;
                }
            }
        }
    }

    should_send
}

fn intersected(p1: &PacketCharInfo, p2: &PacketCharInfo) -> bool {
    let p1_x2 = p1.x + p1.w;
    let p1_y2 = p1.y + p1.h;

    let p2_x2 = p2.x + p2.w;
    let p2_y2 = p2.y + p2.h;

    // p1.x --- p2.x --- p1.x2 --- p2.x2
    p1.x <= p2_x2 && p2.x <= p1_x2 && p1.y <= p2_y2 && p2.y <= p1_y2
}

fn game_loop(ambients: &mut Ambients, server: &Server<PacketCharInfo>) {
    let mut lock = 0;
    let (width, height) = (ambients.width, ambients.height);
    
    loop {
        let len_chars = ambients.clients.len() as i16;
        let len_enemies = ambients.enemies.len() as i16;

        for enemy in ambients.enemies.iter_mut() {
            let mut should_send = true;
            let ambient_data = (width, height, ambients.models[(enemy.idmap-1) as usize]);

            if enemy.healt <= 0 {
                // update dead enemies' info
                enemy.exit = true;
            } else {
                // detect nearest char to attack
                let nearest_client = ambients
                                        .clients
                                        .iter_mut()
                                        .filter(|c| c.idmap == enemy.idmap && !c.exit)
                                        .map(|c| {
                                            let dist = distance(c, enemy);
                                            (c, dist)
                                        })
                                        .max_by(|c1, c2| c1.1.partial_cmp(&c2.1).unwrap());

                if let Some((client, dist)) = nearest_client {
                    if dist <= enemy.vision as f32 {
                        enemy.a = 1;

                        // check if this enemy hit this client
                        if intersected(enemy, client) {
                            if dir_damage_chance(enemy, client, 0.5, ambient_data) {
                                server.send(Message::Broadcast(client.clone()));
                            }
                        }

                        // boss moves differently
                        if enemy.numchar == ambients.boss_num {
                            should_send = move_boss(enemy, client, &mut lock);
                        } else {
                            move_enemy(enemy, client, ambient_data);
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

        for _ in 0..5 {
            recv_once(ambients, server);
            move_chars(ambients, server);
            thread::sleep(Duration::from_millis(16));
        }
    }
}

fn move_chars(ambients: &mut Ambients, server: &Server<PacketCharInfo>) {
    for this_char in ambients.clients.iter_mut() {
        if this_char.a == 1 || this_char.a == 2 {
            match this_char.d as i32 {
                GDPLEFT => this_char.x -= this_char.step,
                GDPRIGHT => this_char.x += this_char.step,
                GDPUP => this_char.y -= this_char.step,
                GDPDOWN => this_char.y += this_char.step,
                _ => unreachable!()
            }

            server.send(Message::Broadcast(this_char.clone()));
        }
    }
}

fn collided(enemy: &PacketCharInfo, ambient_data: (i16, i16, *const AlBitmap)) -> bool {
    let (width, height, model) = ambient_data;

    if enemy.y < 0 || enemy.x < 0 {
        return true;
    }

    if (enemy.y + enemy.h) > height as i16 {
        return true;
    }

    let colorwall = al_map_rgb(0, 0, 0);

    let we = al_get_bitmap_width(model) as f32;
    let he = al_get_bitmap_height(model) as f32;

    let sx = we / width as f32;
    let sy = he / height as f32;

    let xup   = (enemy.x + enemy.w) as f32 * sx;
    let yup   = enemy.y as f32 * sy;

    let xdown = enemy.x as f32 * sx;
    let ydown = (enemy.y + enemy.h) as f32 * sy;

    if xdown >= 0.0 && ydown >= 0.0 && xup <= we && yup <= he {
        let color = al_get_pixel(model, xdown as i32, ydown as i32);
        if colorwall == color {
            return true;
        }

        let color = al_get_pixel(model, xup as i32, ydown as i32);
        if colorwall == color {
            return true;
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        let (mut p1, mut p2) = (PacketCharInfo::default(), PacketCharInfo::default());
        p1.x = 5;
        p1.y = 5;
        p1.w = 1;
        p1.h = p1.w;

        p2.x = 4;
        p2.y = 4;
        p2.w = 1;
        p2.h = p1.w;

        let dist = distance(&p1, &p2);
        assert_eq!(dist, 2.0f32.sqrt());

        let dist = distance(&p2, &p1);
        assert_eq!(dist, 2.0f32.sqrt());

        let dist = distance(&p2, &p2);
        assert_eq!(dist, 0.0);

        let dist = distance(&p1, &p1);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_dir_damage() {
        //let (mut p1, mut p2) = (PacketCharInfo::default(), PacketCharInfo::default());
        //dir_damage(&p1, &mut p2);
    }
}