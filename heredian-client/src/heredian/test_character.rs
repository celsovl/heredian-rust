use heredian_lib::allegro_safe::*;
use crate::heredian::structs::{GameState, Char};

#[derive(Default, Debug)]
pub struct TestCharacterScreen {
    current_char: Option<Char>,
}

impl TestCharacterScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, state: &mut GameState) {
        let mut camera = AlTransform::default();
        al_identity_transform(&mut camera);
        al_build_transform(&mut camera, 0.0, 0.0, 3.0, 3.0, 0.0);
        al_use_transform(&camera);

        let mut evento = AlEvent::default();

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if !al_is_event_queue_empty(state.event_queue) {
                        continue;
                    }

                    if self.check_char() {
                        self.update();
                        self.draw();
                    }
                },
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => break,
                _ => ()
            }
        }
    }

    fn check_char(&mut self) -> bool {
        let mut kstate = AlKeyboardState::default();
        al_get_keyboard_state(&mut kstate);

        let char_id =
            if al_key_down(&kstate, ALLEGRO_KEY_1) {
                1
            } else if al_key_down(&kstate, ALLEGRO_KEY_2) {
                2
            } else if al_key_down(&kstate, ALLEGRO_KEY_3) {
                3
            } else if al_key_down(&kstate, ALLEGRO_KEY_4) {
                4
            } else {
                0
            };

        if char_id > 0 && self.current_char.as_ref().map(|c| c.obj.r#type != char_id).unwrap_or(true) {
            let mut ch = Char::load(char_id);
            println!("Loaded char {}", ch.info.name);
            ch.obj.x = 100.0;
            ch.obj.y = 75.0;
            self.current_char = Some(ch);
        }

        self.current_char.is_some()
    }

    fn update(&mut self) {
        let current_char = self.current_char.as_mut().unwrap();
        current_char.update_local((0,0,0,0,0,std::ptr::null()));
    }

    fn draw(&mut self) {
        al_clear_to_color(al_map_rgb(0, 0, 0));

        let current_char = self.current_char.as_mut().unwrap();
        current_char.draw();

        al_flip_display();
    }
}