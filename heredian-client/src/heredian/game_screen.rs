use std::ptr;

use crate::heredian::allegro_safe::*;
use crate::heredian::structs::*;

pub struct GameScreen {

}

impl GameScreen {
    fn init(&mut self, state: &GameState) {
        /*
        self.fonte = al_load_font("assets/Fonts/font_menu.ttf", 100, 0);
        self.image = al_load_bitmap("assets/Images/select.png");

        self.musicsel = al_load_sample("assets/Songs/Menu/musicsel.ogg");
        self.musicconfirm = al_load_sample("assets/Songs/Menu/musicconfirm.ogg");

        self.textos = vec!["James", "Julios", "Japa", "Gauss"];
        self.confirm = ">";
        */
    }

    fn close(&mut self, state: &GameState) {
        /*
        al_destroy_font(self.fonte);
        al_destroy_bitmap(self.image);
        al_destroy_sample(self.musicsel);
        al_destroy_sample(self.musicconfirm);
        */
    }

    fn update(&mut self, state: &GameState, evento: &AlKeyboardEvent) {

    }

    fn draw(&mut self, state: &GameState) {

    }

    fn run_loop(&mut self, state: &GameState) {
        let mut evento = AlEvent::default();

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_KEY_DOWN => {
                    self.update(state, evento.get_keyboard());
                },
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if !al_is_event_queue_empty(state.event_queue) {
                        continue;
                    }

                    self.draw(state);
                },
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => break,
                _ => ()
            }
        }
    }

    pub fn show(&mut self, state: &GameState) {
        self.init(state);
        self.run_loop(state);
        self.close(state);
    }
}