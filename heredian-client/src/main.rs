extern crate libc;

use crate::heredian::game_lib::*;
use crate::heredian::structs::*;
use crate::heredian::splash_screen::*;
use crate::heredian::intro_screen::*;
use crate::heredian::menu_screen::*;
use crate::heredian::select_char_screen::*;
use crate::heredian::loading_screen::*;
use crate::heredian::game_screen::*;

pub mod heredian;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
static TITLE: &str = "Heredian";

fn main() {
    let mut state = gdp_init();
    state.init();

    SplashScreen::show(&state);
    IntroScreen::show(&state);
    let opcao = MenuScreen::show(&state);

    state.opmenu = Some(opcao);

    if Some(OpcaoMenu::NovoJogo) == state.opmenu {
        let opcao = SelectCharScreen::show(&state);

        if opcao.is_some() {
            state.opchar = opcao;

            LoadingScreen::show(&state);

            let game = GameScreen {};
            //game.show(&state);
        }
    }

    /*
    // inicia o allegro
    gdp_init();
    // inicia os eventos
    gdp_initevents();
    // inicia o timer
    gdp_timer();
    // exibe splash
    gdp_splash();
    // intro
    gdp_intro();

    // se fechou a tela não faz nada
    if nclose_game == 0 {
        // exibe o menu
        gdp_menu();
        // opção 1 inicia o jogo
        if opmenu == 1 {
            if nclose_game == 0 {
                //tela de loaging
                gdp_loaging();
	            gdp_game();
            }
        }
    }

    gdp_close();
    */
}
