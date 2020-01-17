use heredian_lib::allegro_safe::*;
use super::structs::*;

pub struct SplashScreen;

impl SplashScreen {

    fn fade_in(alfa: u8, image: *const AlBitmap, state: &GameState) {
        Self::fade(alfa, image, state);
    }

    fn fade_out(alfa: u8, image: *const AlBitmap, state: &GameState) {
        Self::fade(255-alfa, image, state);
    }

    fn fade(alfa: u8, image: *const AlBitmap, state: &GameState) {
        al_clear_to_color(al_map_rgb(0, 0, 0));
        
		// imagem de fundo
		al_draw_tinted_scaled_bitmap(
			image,
			al_map_rgba(alfa, alfa, alfa, alfa),
			0f32, 0f32,
			al_get_bitmap_width(image) as f32,
			al_get_bitmap_height(image) as f32,
			0f32,
			0f32,
			state.width as f32,
			state.height as f32,
			0
        );
        
		al_flip_display();
    }

    pub fn show(state: &GameState) {
        let image = al_load_bitmap("assets/Images/logo0.png");
        let timer = al_create_timer(0.005);

        al_register_event_source(state.event_queue, al_get_timer_event_source(timer));
        al_start_timer(timer);

        enum Phase {
            FadeIn,
            FadeOut,
            Middle
        }

        let mut phase = Phase::FadeIn;
        let mut evento = AlEvent::default();
        let mut ciclos = 0;

        loop {
            al_wait_for_event(state.event_queue, &mut evento);

            match evento.get_type() {
                AlEventType::ALLEGRO_EVENT_TIMER => {
                    if evento.get_timer().source == timer {

                        match phase {
                            Phase::FadeIn => {
                                Self::fade_in(ciclos, image, state);
                                if ciclos == 255 {
                                    phase = Phase::Middle;
                                    ciclos = 0;
                                }
                            },
                            Phase::Middle => {
                                if ciclos == 255 {
                                    phase = Phase::FadeOut;
                                    ciclos = 0;
                                }
                            },
                            Phase::FadeOut => {
                                Self::fade_out(ciclos, image, state);
                                if ciclos == 255 {
                                    break;
                                }
                            },
                        }

                        ciclos += 1;
                    }
                },
                AlEventType::ALLEGRO_EVENT_KEY_DOWN => break,
                AlEventType::ALLEGRO_EVENT_DISPLAY_CLOSE => break,
                _ => ()
            }
        }

        al_destroy_timer(timer);
        al_destroy_bitmap(image);
    }
}