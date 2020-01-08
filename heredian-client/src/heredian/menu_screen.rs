use std::ptr;

use super::allegro_safe::*;
use super::structs::*;

pub struct MenuScreen;

impl MenuScreen {

    pub fn show(&self, state: &GameState) -> OpcaoMenu {
        let fonte = al_load_font("assets/Fonts/font_menu.ttf", 100, 0);
        let image = al_load_bitmap("assets/Images/Menu.png");
        let music = al_load_sample("assets/Songs/Menu/music.ogg");
        al_play_sample(music, 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_LOOP, ptr::null_mut());

        let musicsel = al_load_sample("assets/Songs/Menu/musicsel.ogg");
        let musicconfirm = al_load_sample("assets/Songs/Menu/musicconfirm.ogg");

        let textos = vec!["Novo Jogo", "Sair"];
        let confirm = ">";

        let (mut nopcao, nposicao, nespaco) = (0, 290, 80);

        let mut evento = AlEvent::default();

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if !al_is_event_queue_empty(state.event_queue) {
                        continue;
                    }

                    al_clear_to_color(al_map_rgb(0, 0, 0));

                    // imagem de fundo
                    al_draw_scaled_bitmap(image,
                        0f32, 0f32,
                        al_get_bitmap_width(image) as f32,
                        al_get_bitmap_height(image) as f32,
                        0 as f32,
                        0 as f32,
                        state.width as f32,
                        state.height as f32,
                        0);

                    // menu
                    for (i, linha) in textos.iter().enumerate() {
                        al_draw_text(
                            fonte, 
                            al_map_rgb(0, 0, 0), 
                            (state.width / 2) as f32,
                            (nposicao + nespaco * i) as f32,
                            ALLEGRO_ALIGN_CENTRE, 
                            linha);
                    }

                    // selecao
                    al_draw_text(
                        fonte, 
                        al_map_rgb(0, 0, 0), 
                        ((state.width / 2)-230) as f32,
                        (nposicao + nopcao * nespaco) as f32,
                        ALLEGRO_ALIGN_LEFT, 
                        confirm);

                    al_flip_display();
                },
                AlEventType::ALLEGRO_EVENT_KEY_DOWN => {
                    match evento.get_keyboard().keycode {
                        ALLEGRO_KEY_DOWN => {
                            al_play_sample_b(musicsel, 2.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());
                            if nopcao < textos.len()-1 {
                                nopcao += 1;
                            }
                        },
                        ALLEGRO_KEY_UP => {
                            al_play_sample_b(musicsel, 2.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());
                            if nopcao > 0  {
                                nopcao -= 1;
                            }
                        },
                        ALLEGRO_KEY_ENTER => {
                            al_play_sample(musicconfirm, 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());
                            al_rest(1.0);
                            break;
                        },
                        _ => ()
                    };
                },
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => {
                    nopcao = textos.len();
                    break;
                },
                _ => ()
            }
        }

        al_destroy_font(fonte);
        al_destroy_bitmap(image);
        al_destroy_sample(music);
        al_destroy_sample(musicsel);
        al_destroy_sample(musicconfirm);

        match nopcao {
            0 => OpcaoMenu::NovoJogo,
            1 => OpcaoMenu::Sair,
            _ => OpcaoMenu::Fechar
        }
    }
}