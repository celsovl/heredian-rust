use std::cmp::{Ord, Ordering};
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::time::{Duration, Instant};
use std::thread;

use heredian_lib::*;
use heredian_lib::net::{Client};
use heredian_lib::allegro_safe::*;
use crate::heredian::structs::*;

pub struct GameScreen {
    client: Option<Client<PacketCharInfo>>
}

impl GameScreen {

    pub fn new() -> GameScreen {
        GameScreen {
            client: None
        }
    }

    fn init(&mut self, state: &mut GameState) {
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 34000));
        let mut client = Client::<PacketCharInfo>::connect(&address);
        client.start();

        let now = Instant::now();

        let char_info =
            loop {
                if now.elapsed() < Duration::from_secs(30) {
                    match client.try_recv() {
                        Ok(info) if info.numchar == 0 => break info,
                        Ok(_) | Err(_) => ()
                    }
                } else {
                    panic!("Character creation timed out.");
                }

                thread::yield_now();
            };

        let ambient = Scene::load(state.opmap, state.width, state.height);
        state.local_char_id = char_info.idchar as usize;
        
        println!("New ID: {:?}", char_info);

        let mut local_char = Char::load(state.opchar.unwrap() as i32);
        local_char.idmap = state.opmap;
        local_char.obj.id = char_info.idchar as i32;
        local_char.obj.idchar = char_info.idchar as i32;
        local_char.obj.x = ambient.ex as f32;
        local_char.obj.y = ambient.ey as f32;
        
        state.list_chars.push(local_char);
        state.ambient = Some(ambient);

        self.client = Some(client);
    }

    fn close(&mut self, _state: &mut GameState) {
        /*
        al_destroy_font(self.fonte);
        al_destroy_bitmap(self.image);
        al_destroy_sample(self.musicsel);
        al_destroy_sample(self.musicconfirm);
        */
    }

    fn draw(&mut self, state: &mut GameState) {
        // clear screen
        al_clear_to_color(al_map_rgb(0, 0, 0));

        self.scale_camera(state);

        let ambient = state.ambient.as_ref().unwrap();
        ambient.draw();

        self.draw_objects(state);

        self.reset_camera();

        self.draw_info(state);

        al_flip_display();
    }

    fn draw_objects(&self, state: &mut GameState) {
        enum CharOrLifeless<'a> {
            Char(&'a mut Char),
            Lifeless(&'a mut Lifeless)
        }

        impl<'a> PartialEq for CharOrLifeless<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl<'a> PartialOrd for CharOrLifeless<'a> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<'a> Eq for CharOrLifeless<'a> {}

        impl<'a> Ord for CharOrLifeless<'a> {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    (CharOrLifeless::Char(c1), CharOrLifeless::Char(c2)) => c1.obj.id.cmp(&c2.obj.id),
                    (CharOrLifeless::Char(_), CharOrLifeless::Lifeless(_)) => Ordering::Less,
                    (CharOrLifeless::Lifeless(_), CharOrLifeless::Char(_)) => Ordering::Greater,
                    (CharOrLifeless::Lifeless(l1), CharOrLifeless::Lifeless(l2)) => l1.obj.id.cmp(&l2.obj.id),
                }
            }
        }

        // it creates vec to sort objects in order of their bottom position
        // objects with a lower bottom position (i.e. up on the display) are drawn first
        let mut objects = Vec::with_capacity(state.list_chars.len() + state.list_lifeless.len());

        let idmap = state.ambient.as_ref().unwrap().id;

        objects.extend(state.list_chars.iter_mut().filter(|c| c.idmap == idmap).map(|c: &mut Char| ((c.obj.y + c.obj.h) as i32, CharOrLifeless::Char(c))));
        objects.extend(state.list_lifeless.iter_mut().filter(|l| l.idmap == idmap).map(|l: &mut Lifeless| ((l.obj.y + l.obj.h) as i32, CharOrLifeless::Lifeless(l))));
        objects.sort();

        for (_, object) in objects {
            match object {
                CharOrLifeless::Char(c) => c.draw(),
                CharOrLifeless::Lifeless(l) => l.draw()
            }
        }
    }

    fn draw_info(&self, state: &mut GameState) {
        let ambient = state.ambient.as_ref().unwrap();
        let fonte = ambient.info.fonte;

        // draw char info (health, stamina, ...)
        for (i, c) in state.list_chars.iter().filter(|c| c.obj.r#type <= 4).enumerate() {
            c.draw_info(state, i as i32);
        }
        
        let fps = state.avg_fps();

        // writes the FPS
        al_draw_text(
            fonte,
            al_map_rgb(255,255,255),
            630.0,
            5.0,
            0,
            &format!("FPS: {:.3}", fps));
    }

    fn reset_camera(&self) {
        let mut camera = AlTransform::default();
        al_identity_transform(&mut camera);
        al_use_transform(&camera);
    }

    fn scale_camera(&self, state: &mut GameState) {
        let mut camera = AlTransform::default();
        al_identity_transform(&mut camera);

        let scale = state.scale;
        let local_char = state.get_localchar().unwrap();
        let ambient = state.ambient.as_ref().unwrap();
        let (x, y, w, h) = (local_char.obj.x, local_char.obj.y, local_char.obj.wd, local_char.obj.hd);

        let mut scaled_x = -((x * scale) - (ambient.wd as f32/2.0) + w/2.0);
        let mut scaled_y = -((y * scale) - (ambient.hd as f32/2.0) + h/2.0);

        scaled_x = scaled_x.min(0.0).max(-((scale-1.0)*(ambient.wd as f32)));
        scaled_y = scaled_y.min(ambient.info.h as f32).max(-((scale-1.0)*(ambient.hd as f32)));

        al_build_transform(&mut camera, scaled_x, scaled_y, scale, scale, 0.0);
        al_use_transform(&camera);
    }

    fn run_loop(&mut self, state: &mut GameState) {
        let mut evento = AlEvent::default();

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_KEY_DOWN => {
                    ();
                },
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if !al_is_event_queue_empty(state.event_queue) {
                        continue;
                    }

                    //self.recv_once(state);
                    self.update(state);
                    self.draw(state);
                },
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => break,
                _ => ()
            }
        }
    }

    fn update_scale(&self, state: &mut GameState) {
        let mut kb_state = AlKeyboardState::default();
        al_get_keyboard_state(&mut kb_state);

        if al_key_down(&kb_state, ALLEGRO_KEY_2) {
            state.scale = (state.scale+0.05).min(2.0);
        }

        if al_key_down(&kb_state, ALLEGRO_KEY_1) {
            state.scale = (state.scale-0.05).max(1.0);
        }
    }

    fn update(&mut self, state: &mut GameState) {
        self.update_scale(state);

        match self.client.as_ref() {
            Some(client) => {
                while let Ok(char_info) = client.try_recv() {
                    //println!("id: {:#?}", &char_info);
                    state.update_char(char_info);
                }

                let mut should_send = state.update_local_char(client);
                should_send |= self.try_ambient_change(state);

                if should_send {
                    let local_char = state.get_localchar_mut().expect("Cannot find local char.");
                    local_char.send(client);
                }
            },
            None => panic!("No channel available for propagation of local char's changes.")
        }
    }

    pub fn try_ambient_change(&self, state: &mut GameState) -> bool {
        state.try_change_ambient()
    }

    pub fn show(&mut self, state: &mut GameState) {
        self.init(state);
        self.run_loop(state);
        self.close(state);
    }
}