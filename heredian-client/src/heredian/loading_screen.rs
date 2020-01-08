use super::allegro_safe::*;
use super::structs::*;

pub struct LoadingScreen;

impl LoadingScreen {
    pub fn show(state: &GameState) {
        let fonte = al_load_font("assets/Fonts/font_menu.ttf", 30, 0);
    
        al_clear_to_color(al_map_rgb(0, 0, 0));
        al_draw_text(
            fonte, 
            al_map_rgb(255, 255, 255), 
            (state.width-120) as f32,
            (state.height-50) as f32,
            ALLEGRO_ALIGN_LEFT, 
            "Carregando...");
            
        al_flip_display();
    
        al_destroy_font(fonte);
    }
}