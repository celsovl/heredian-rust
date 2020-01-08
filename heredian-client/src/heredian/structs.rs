use std::ptr;
use std::collections::VecDeque;
use std::path::Path;

use crate::heredian::allegro_safe::*;

use super::file_manager::ConfigFile;

pub const GDPLEFT: i32 = 1;
pub const GDPRIGHT: i32 = 2;
pub const GDPUP: i32 = 4;
pub const GDPDOWN: i32 = 8;
pub const DIRECTIONS: usize = 4;
pub const CHARS: usize =  30;
pub const LIFELESS: usize =  20;
pub const MAXCHARLIFELESS: usize =  5;

pub const FPS: f64 = 60.0;

#[derive(PartialEq)]
pub enum OpcaoMenu {
    NovoJogo,
    Sair,
    Fechar,
}

pub enum OpcaoChar {
    James,
    Julios,
    Japa,
    Gauss,
}

pub struct GameState {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
    pub ambient: Option<Scene>,
    pub total_chars: i32,
    pub total_enemies: i32,
    pub total_lifeless: i32,
    pub connect_erro: bool,
    pub scale: f32,

    pub boss_char_id: usize,
    pub local_char_id: usize,

    pub list_chars: Vec<Char>,
    pub list_lifeless: Vec<Char>,

    pub opmenu: Option<OpcaoMenu>,
    pub opchar: Option<OpcaoChar>,
    pub opmap: i32,

    pub ncanaisaudio: i32,
    pub nclose_game: i32,

    pub screen: *const AlDisplay,
    pub event_queue: *const AlEventQueue,
    pub timer: *const AlTimer,
}

impl GameState {
    pub fn create_display(&mut self) {
        assert!(self.screen.is_null());

        // Configura a janela
        //al_set_new_display_flags(ALLEGRO_FULLSCREEN_WINDOW|ALLEGRO_FULLSCREEN);

        self.screen = al_create_display(self.width, self.height);
        // define o titulo
        al_set_window_title(self.screen, &self.title);
    }

    pub fn init(&mut self) {
        assert!(self.event_queue.is_null());
        assert!(self.timer.is_null());

        self.event_queue = al_create_event_queue();
        al_register_event_source(self.event_queue, al_get_keyboard_event_source());
        al_register_event_source(self.event_queue, al_get_display_event_source(self.screen));    

        let nfsp = 1.0 / FPS;

        self.timer = al_create_timer(nfsp);
        al_register_event_source(self.event_queue, al_get_timer_event_source(self.timer));

        al_start_timer(self.timer);
    }
}

#[derive(Default, Debug)]
pub struct Sprite {
    w: i32,
    h: i32,
    ix: i32,
    iy: i32,
    last: i32,
    first: i32,
}

#[derive(Default, Debug)]
pub struct Object {
    id: i32,
    idchar: i32,
    x: i32,
    y: i32,
    w: f64,
    h: f64,
    wd: i32,
    hd: i32,
    d: i32,
    d2: u32,
    a: i32,
    lock: i32,
    a2: u32,
    r#type: i32,
}

#[derive(Debug)]
pub struct Action {
    id: i32,
    fps: f64,
    directions: VecDeque<Sprite>,
    image: *const AlBitmap,
    fila_timer: *const AlEventQueue,
    sound: Option<*const AlSample>,
    stepx: i32,
    stepy: i32,
    damage: i32,
    lock: i32,
    lifelessid: Option<i32>,
    charge: Option<i32>,
    rebatex: Option<i32>,
    rebatey: Option<i32>,
}

impl Drop for Action {
    fn drop(&mut self) {
        if !self.image.is_null() {
            al_destroy_bitmap(self.image);
        }

        if !self.fila_timer.is_null() {
            al_destroy_event_queue(self.fila_timer);
        }

        if let Some(sound) = self.sound {
            if !sound.is_null() {
                al_destroy_sample(sound);
            }
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action {
            id: Default::default(),
            fps: Default::default(),
            directions: Default::default(),
            image: ptr::null(),
            fila_timer: ptr::null(),
            sound: Default::default(),
            stepx: Default::default(),
            stepy: Default::default(),
            charge: Default::default(),
            damage: Default::default(),
            lock: Default::default(),
            lifelessid: Default::default(),
            rebatex: Default::default(),
            rebatey: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Lifeless {
    obj: Object,
    actions: Vec<Action>,
    idmap: i32,
    dead: i32,
}

#[derive(Default, Debug)]
pub struct InfoChar {
    name: String,
    healtfull: i32,
    staminafull: i32,
    healt: i32,
    stamina: i32,
}

#[derive(Default, Debug)]
pub struct Gate {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    ambient_id: i32,
    ex: i32,
    ey: i32,
}

#[derive(Default, Debug)]
pub struct Char {
    pub obj: Object,
    pub act: Vec<Action>,
    pub info: InfoChar,
    pub idmap: i32,
    pub dead: i32,
    pub totlifeless: i32,
    pub listlifeless: Vec<Lifeless>,
}

#[derive(Debug)]
pub struct Info {
    image: Option<*const AlBitmap>,
    w: i32,
    h: i32,
    fonte: *const AlFont,
    color: AlColor,
}

impl Drop for Info {
    fn drop(&mut self) {
        if let Some(image) = self.image {
            if !image.is_null() {
                al_destroy_bitmap(image);
            }
        }

        if !self.fonte.is_null() {
            al_destroy_font(self.fonte);
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    id: i32,
    info: Info,
    image: *const AlBitmap,
    model: *const AlBitmap,
    musicback: *const AlSample,
    w: i32,
    h: i32,
    wd: i32,
    hd: i32,
    ex: i32,
    ey: i32,
    gates: Vec<Gate>,
}

impl Drop for Scene {
    fn drop(&mut self) {
        if !self.image.is_null() {
            al_destroy_bitmap(self.image);
        }

        if !self.model.is_null() {
            al_destroy_bitmap(self.model);
        }

        if !self.musicback.is_null() {
            al_destroy_sample(self.musicback);
        }
    }
}


#[derive(Default, Debug)]
pub struct PacketLifelessInfo {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
    d: i16,
    damage: i16,
}

#[derive(Default, Debug)]
pub struct PacketCharInfo {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
    a: i16,
    d: i16,
    dhit: i16,
    numchar: i16,
    idchar: i16,
    totchar: i16,
    totenemies: i16,
    exit: i16,
    healt: i16,
    stamina: i16,
    damage: i16,
    idmap: i16,
    totlifeless: i16,
	step: i16,
    vision: i16,
    listlifeless: [PacketLifelessInfo; MAXCHARLIFELESS],
}

#[derive(Debug)]
pub enum LayeredObjectType {
    None = 0,
    EnemyOrChar = 1,
    Lifeless = 2
}

impl Default for LayeredObjectType {
    fn default() -> Self {
        LayeredObjectType::None
    }
}

#[derive(Default, Debug)]
pub struct LayeredObject {
    r#type: LayeredObjectType,
    arr_idx: i32,
    y: i32,
}

impl Gate {
    pub fn from_config(config_file: &ConfigFile) -> Vec<Gate> {
        let qt_gates = config_file.get("num_gates").expect("num_gates não encontrado.");
        let mut gates = Vec::new();

        for i in 1..=qt_gates {
            gates.push(Gate {
                x1: config_file.get(&format!("gate{}_x1", i)).expect("gate_x1 não encontrado."),
                y1: config_file.get(&format!("gate{}_y1", i)).expect("gate_y1 não encontrado."),
                x2: config_file.get(&format!("gate{}_x2", i)).expect("gate_x2 não encontrado."),
                y2: config_file.get(&format!("gate{}_y2", i)).expect("gate_y2 não encontrado."),
                ambient_id: config_file.get(&format!("gate{}_map", i)).expect("gate_map não encontrado."),
                ex: config_file.get(&format!("gate{}_ex", i)).expect("gate_ex não encontrado."),
                ey: config_file.get(&format!("gate{}_ey", i)).expect("gate_ey não encontrado."),
            });
        }

        gates
    }
}

impl Info {
    pub fn from_config(width: i32) -> Info {
        Info {
            fonte: al_load_font("assets/Fonts/font_info.TTF", 12, 0),
            h: 100,
            w: width,
            color: al_map_rgb(255,255,255),
            image: None,
        }
    }
}

impl Sprite {
    pub fn from_config(config_file: &ConfigFile) -> VecDeque<Sprite> {
        let qt_sprites = config_file.get("qt_sprites").expect("qt_sprites não encontrado");

        let ntamx = config_file.get("size_x").expect("size_x não encontrado");
        let ntamy = config_file.get("size_y").expect("size_y não encontrado");

        let mut sprites = VecDeque::new();

        for i in 0..4 {
            if qt_sprites == -1 {
                sprites.push_front(Sprite {
                    ix: 0,
                    iy: 0,
                    first: 1,
                    last: 1,
                    h: 0,
                    w: 0,
                });
            }

            for j in (0..qt_sprites).rev() {
                let mut sprite = Sprite {
                    ix: j,
                    iy: i,
                    w: ntamx,
                    h: ntamy,
    
                    last: 0,
                    first: 0,
                };

                if j == qt_sprites-1 {
                    sprite.last = 1;
                } else if j == 0 {
                    sprite.first = 1;
                }

                sprites.push_front(sprite);
            }
        }

        sprites
    }
}

impl Object {
    pub fn from_config(config_file: &ConfigFile, r#type: i32) -> Object {
        Object {
            r#type: r#type,
            id: config_file.get("id").expect("ID não encontrado."),
            w: config_file.get("scale_w").expect("scale_w não encontrado."),
            h: config_file.get("scale_h").expect("scale_h não encontrado."),
            x: config_file.get("posX").expect("posX não encontrado."),
            y: config_file.get("posY").expect("posY não encontrado."),
            d: config_file.get("direction").expect("direction não encontrado."),
            a: config_file.get("ini_act").expect("ini_act não encontrado."),
            lock: 0,
            a2: 0,
            d2: 0,
            
            hd: 0,
            wd: 0,
            idchar: 0,
        }
    }
}

impl InfoChar {
    pub fn from_config(config_file: &ConfigFile) -> InfoChar {
        let mut info = InfoChar {
            name: config_file.get_string("name").expect("name não encontrado.").to_string(),
            healtfull: config_file.get("healtfull").expect("healtfull não encontrado."),
            staminafull: config_file.get("staminafull").expect("staminafull não encontrado."),
            healt: 0,
            stamina: 0,
        };

        info.healt = info.healtfull;
        info.stamina = info.staminafull;

        info
    }
}

impl Action {
    pub fn from_config(config_file: &ConfigFile) -> Vec<Action> {
        let numacao = config_file.get("action_number").expect("Número de ações não informado");

        let mut acoes = Vec::with_capacity(numacao);
        for i in 0..(numacao as i32) {
            acoes.push(Action::from_config_and_id(&config_file, i));
        }

        acoes
    }

    pub fn from_config_and_id(config_file: &ConfigFile, id: i32) -> Action {
        let action_file = config_file
                            .get_string(&format!("act_{}", id))
                            .expect(&format!("Ação de ID {} não encontrada.", id));

        let action_config_file = ConfigFile::load(action_file);

        let nacao = action_config_file.get("id").expect("id não encontrado.");
        
        let image_path = action_config_file.get_string("image").expect("image não encontrado.");
        let pri = al_load_bitmap(image_path);

        let sound_path = action_config_file.get_string("sound");
        let sound = sound_path.map(|s| al_load_sample(s));

        let fps = action_config_file.get("fps").expect("fps não encontrado.");
        let nfsp = 1.0 / fps;
        let chartimer = al_create_timer(nfsp);
        let fila_timer = al_create_event_queue();
        al_register_event_source(fila_timer, al_get_timer_event_source(chartimer));
        al_start_timer(chartimer);

        let directions = Sprite::from_config(&action_config_file);

        Action {
            id: nacao,
            image: pri,
            stepx: action_config_file.get("stepx").expect("stepx não encontrado."),
            stepy: action_config_file.get("stepy").expect("stepy não encontrado."),
            fps: fps,
            lifelessid: action_config_file.get("lifelessid"),
            rebatex: action_config_file.get("rebatex"),
            rebatey: action_config_file.get("rebatey"),
            fila_timer: fila_timer,

            sound: sound,

            charge: action_config_file.get("charge"),
            damage: action_config_file.get("damage").expect("damage não encontrado."),
            lock: action_config_file.get("lock").expect("lock não encontrado."),

            directions: directions,
        }
    }
}

impl Lifeless {
    pub fn load(id: i32) -> Lifeless {
        let path = Path::new("assets/Configs/Lifeless.txt");
        let lifeless_config_file = ConfigFile::load(path);
        let lifeless_path = lifeless_config_file.get_string(&id.to_string()).expect("Lifeless não encontrado.");
        let config_file = ConfigFile::load(lifeless_path);

        let obj = Object::from_config(&config_file, 2);
        let acoes = Action::from_config(&config_file);

        Lifeless {
            idmap: -1,
            dead: 0,
            obj: obj,
            actions: acoes,
        }
    }
}
impl Char {
    pub fn load(id: i32) -> Char {
        let path = Path::new("assets/Configs/Chars.txt");
        let chars_config_file = ConfigFile::load(path);

        let char_path = chars_config_file.get_string(&id.to_string()).expect("Char não encontrado.");
        let config_file = ConfigFile::load(char_path);

        let obj = Object::from_config(&config_file, 1);
        let info = InfoChar::from_config(&config_file);
        let acoes = Action::from_config(&config_file);

        Char {
            idmap: -1,
            dead: 0,
            totlifeless: 0,
            act: acoes,
            obj: obj,
            info: info,
            listlifeless: Vec::with_capacity(10),
        }
    }
}

impl Scene {
    pub fn load(id: i32, width: i32, height: i32) -> Scene {
        let path = Path::new("assets/Configs/Ambients.txt");
        let ambient_config_file = ConfigFile::load(path);

        let ambient_path = ambient_config_file.get_string(&format!("map{}", id)).expect("Ambient {} não encontrado.");
        let config_file = ConfigFile::load(ambient_path);

        let image_path = config_file.get_string("image").expect("image não encontrado.");
        let image = al_load_bitmap(image_path);
        al_lock_bitmap(image, al_get_bitmap_format(image), ALLEGRO_LOCK_READONLY);

        let model_path = config_file.get_string("model").expect("model não encontrado.");
        let flags = al_get_new_bitmap_flags();
        al_add_new_bitmap_flag(ALLEGRO_MEMORY_BITMAP);
        let model  = al_load_bitmap(model_path);
        al_set_new_bitmap_flags(flags);
    
        let sound_path = config_file.get_string("sound").expect("sound não encontrado.");
        let sound = al_load_sample(sound_path);

        let info = Info::from_config(width);
        let gates = Gate::from_config(&config_file);

        al_play_sample(sound, 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_LOOP, ptr::null_mut());

        Scene {
            ex: 0,
            ey: 0,
            w: al_get_bitmap_width(image),
            h: al_get_bitmap_height(image),
            wd: width,
            hd: height,
            id: id,
            image: image,
            info: info,
            model: model,
            musicback: sound,
            gates: gates,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INIT_ONCE: std::sync::Once = std::sync::Once::new();

    fn setup() {
        INIT_ONCE.call_once(|| {
            al_init();
            al_init_image_addon();
            al_install_audio();
            al_init_acodec_addon();
            al_init_font_addon();
            al_init_ttf_addon();
        });
    }

    #[test]
    fn load_char() {
        setup();

        let my_char = Char::load(1);
        assert_eq!(my_char.act.len(), 5);
        let my_char = Char::load(2);
        assert_eq!(my_char.act.len(), 5);
    }

    #[test]
    fn load_lifeless() {
        setup();

        let my_lifeless = Lifeless::load(1);
        assert_eq!(my_lifeless.actions.len(), 1);
        let my_lifeless = Lifeless::load(2);
        assert_eq!(my_lifeless.actions.len(), 1);
    }

    #[test]
    fn load_ambient() {
        setup();
        al_reserve_samples(3);

        let ambient = Scene::load(1, 640, 480);
        println!("{:#?}", ambient);
    }
}