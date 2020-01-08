use std::ptr;

use super::allegro_safe::*;
use super::file_manager::*;
use super::structs::*;

pub struct IntroScreen;

impl IntroScreen {

    fn carregar_texto<'a>(&self, config_file: &'a ConfigFile) -> Vec<&'a String> {
        let num_lines = config_file.get("num_lines").expect("num_lines não encontrado.");
        let mut texto = Vec::with_capacity(num_lines);

        for i in 1..=num_lines {
            texto.push(
                config_file
                    .get_string(&format!("lin{}", i))
                    .expect(&format!("lin{} não encontrado.", i)));
        }

        texto
    }

    pub fn show(&self, state: &GameState) {
        let config_file = ConfigFile::load("assets/Configs/Intro.txt");
        let fonte = al_load_font("assets/Fonts/font_intro.TTF", 20, 0);

        let music = al_load_sample("assets/Songs/Intro/intro_music.ogg");
        al_play_sample(music, 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_LOOP, ptr::null_mut());

        let narrative = al_load_sample("assets/Songs/Intro/intro_narrative.ogg");
        al_play_sample(narrative, 2.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());

        let texto = self.carregar_texto(&config_file);
        let speed: f32 = config_file.get("speed_text").expect("speed_text não encontrado");

        let space = 45f32;
        let mut posicao = 0f32;
        let limite = -state.height as f32 - (space * texto.len() as f32);

        let mut evento = AlEvent::default();

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if !al_is_event_queue_empty(state.event_queue) {
                        continue;
                    }

                    al_clear_to_color(al_map_rgb(0, 0, 0));

                    for (i, linha) in texto.iter().enumerate() {
                        al_draw_text(
                            fonte, 
                            al_map_rgb(255, 255, 255), 
                            state.width as f32 / 2.0,
                            state.height as f32 + posicao + (space * i as f32),
                            ALLEGRO_ALIGN_CENTRE,
                            linha);
                    }

                    al_flip_display();

                    posicao -= speed;
                    if posicao < limite {
                        break;
                    }
                },
                AlEventType::ALLEGRO_EVENT_KEY_DOWN => break,
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => break,
                _ => ()
            }
        }

        al_destroy_font(fonte);
        al_destroy_sample(music);
        al_destroy_sample(narrative);
    }
}