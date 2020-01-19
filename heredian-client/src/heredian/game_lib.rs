use std::ptr;

use heredian_lib::{LIFELESS, CHARS};
use heredian_lib::file_manager::*;
use heredian_lib::allegro_safe::*;
use super::structs::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
static TITLE: &str = "Heredian";

pub fn gdp_init() -> GameState {
    let config_file = ConfigFile::load("assets/Configs/server.txt");

    let mut state = GameState {
        ambient: None,
        list_lifeless: Vec::with_capacity(LIFELESS),
        list_chars: Vec::with_capacity(CHARS),
        ncanaisaudio: 4,
        connect_erro: false,
        total_lifeless: 0,
        total_chars: 4,
        total_enemies: 0,
        scale: 1.5,
        boss_char_id: config_file.get("boss_char_id").expect("boss_char_id não encontrado."),
        screen: ptr::null(),
        event_queue: ptr::null(),
        timer: ptr::null(),

        fps: 0.0,
        last_time: 0.0,

        width: WIDTH,
        height: HEIGHT,
        title: TITLE,

        local_char_id: 0,
        nclose_game: 0,
        
        opchar: None,
        opmenu: None,
        opmap: 1,
    };

    // Inicializa a Allegro
	al_init();

	// Inicializa o add-on para utilização de imagens
	al_init_image_addon();

	// Inicializa o add-on para utilização de teclado
	al_install_keyboard();

	// Inicialização do add-on para uso de fontes
	al_init_font_addon();
	al_init_ttf_addon();

	// Inicialização do add-on para uso de sons
	al_install_audio();
	al_init_acodec_addon();
	al_reserve_samples(state.ncanaisaudio);

    //inicia addons de primitivas
    al_init_primitives_addon();

    state.create_display();
    
    state
}