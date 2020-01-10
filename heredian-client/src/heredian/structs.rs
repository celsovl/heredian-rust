use std::ptr;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};

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
#[derive(Copy, Clone)]
pub enum OpcaoMenu {
    NovoJogo,
    Sair,
    Fechar,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OpcaoChar {
    James = 1,
    Julios,
    Japa,
    Gauss,
}

pub type Chan<T> = (Sender<T>, Receiver<T>);

pub trait Connection : Default {
    fn connect(&mut self) -> Chan<PacketCharInfo>;
    fn close(&mut self);
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

    pub fn get_localchar(&self) -> Option<&Char> {
        let fn_find = |v: &&Char| v.obj.id as usize == self.local_char_id;
        self.list_chars.iter().find(fn_find)
    }

    pub fn get_localchar_mut(&mut self) -> Option<&mut Char> {
        let local_char_id = self.local_char_id;
        let fn_find = |v: &&mut Char| v.obj.id as usize == local_char_id;
        self.list_chars.iter_mut().find(fn_find)
    }

    pub fn update_local_char(&mut self, chan: &Chan<PacketCharInfo>) {
        let (opchar, opmap) = (self.opchar.unwrap_or(OpcaoChar::James), self.opmap);
        let local_char = self.get_localchar_mut().expect("Cannot find local char.");

        if local_char.update_local() {
            local_char.send(opchar, opmap, chan);
        }
    }

    pub fn update_char(&mut self, char_info: PacketCharInfo) {
        let fn_find = |v: &&mut Char| v.obj.id == char_info.idchar as i32;
        match self.list_chars.iter_mut().find(fn_find) {
            Some(c) => {
                if c.dead || char_info.idchar < 4 {
                    let new_char = Char::load(char_info.idchar as i32);
                    *c = new_char;
                }

                c.update(char_info);
            },
            None => {
                let new_char = Char::load(char_info.idchar as i32);
                self.list_chars.push(new_char);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Sprite {
    pub w: i32,
    pub h: i32,
    pub ix: i32,
    pub iy: i32,
    pub last: i32,
    pub first: i32,
}

#[derive(Default, Debug)]
pub struct Object {
    pub id: i32,
    pub idchar: i32,
    pub x: f32,
    pub y: f32,
    pub w: f64,
    pub h: f64,
    pub wd: f32,
    pub hd: f32,
    pub d: i32,
    pub d2: i32,
    pub a: i32,
    pub lock: i32,
    pub a2: u32,
    pub r#type: i32,
}

#[derive(Debug)]
pub struct Action {
    pub id: i32,
    pub fps: f64,
    pub directions: VecDeque<Sprite>,
    pub image: *const AlBitmap,
    pub fila_timer: *const AlEventQueue,
    pub sound: Option<*const AlSample>,
    pub stepx: i32,
    pub stepy: i32,
    pub damage: i32,
    pub lock: i32,
    pub lifelessid: Option<i32>,
    pub charge: Option<i32>,
    pub rebatex: Option<i32>,
    pub rebatey: Option<i32>,
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
    pub obj: Object,
    pub actions: Vec<Action>,
    pub idmap: i32,
    pub dead: bool,
}

#[derive(Default, Debug)]
pub struct InfoChar {
    pub name: String,
    pub healtfull: i32,
    pub staminafull: i32,
    pub healt: i32,
    pub stamina: i32,
}

#[derive(Default, Debug)]
pub struct Gate {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub ambient_id: i32,
    pub ex: i32,
    pub ey: i32,
}

#[derive(Default, Debug)]
pub struct Char {
    pub obj: Object,
    pub act: Vec<Action>,
    pub info: InfoChar,
    pub idmap: i32,
    pub dead: bool,
    pub totlifeless: i32,
    pub listlifeless: Vec<Lifeless>,
}

#[derive(Debug)]
pub struct Info {
    pub image: Option<*const AlBitmap>,
    pub w: i32,
    pub h: i32,
    pub fonte: *const AlFont,
    pub color: AlColor,
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
    pub id: i32,
    pub info: Info,
    pub image: *const AlBitmap,
    pub model: *const AlBitmap,
    pub musicback: *const AlSample,
    pub w: i32,
    pub h: i32,
    pub wd: i32,
    pub hd: i32,
    pub ex: i32,
    pub ey: i32,
    pub gates: Vec<Gate>,
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
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub d: i16,
    pub damage: i16,
}

#[derive(Default, Debug)]
pub struct PacketCharInfo {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub a: i16,
    pub d: i16,
    pub dhit: i16,
    pub numchar: i16,
    pub idchar: i16,
    pub totchar: i16,
    pub totenemies: i16,
    pub exit: bool,
    pub healt: i16,
    pub stamina: i16,
    pub damage: i16,
    pub idmap: i16,
    pub totlifeless: i16,
	pub step: i16,
    pub vision: i16,
    pub listlifeless: [Option<PacketLifelessInfo>; MAXCHARLIFELESS],
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
    pub r#type: LayeredObjectType,
    pub arr_idx: i32,
    pub y: i32,
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
            
            hd: 0f32,
            wd: 0f32,
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
            dead: false,
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
            dead: false,
            totlifeless: 0,
            act: acoes,
            obj: obj,
            info: info,
            listlifeless: Vec::with_capacity(10),
        }
    }

    pub fn update(&mut self, char_info: PacketCharInfo) {
        match char_info.dhit as i32 {
            GDPUP => self.obj.y -= 1.0,
            GDPDOWN => self.obj.y += 1.0,
            GDPLEFT => self.obj.x -= 1.0,
            GDPRIGHT => self.obj.x += 1.0,
            other => panic!("invalid PacketCharInfo.dhit value {}", other)
        }

        if !char_info.exit {
            if self.obj.a != 4 {
                self.dead          = false;
                self.obj.id        = char_info.idchar as i32;
                self.obj.idchar    = char_info.idchar as i32;
                self.obj.a         = char_info.a as i32;
                self.obj.d         = char_info.d as i32;
                self.obj.x         = char_info.x as f32;
                self.obj.y         = char_info.y as f32;
                self.info.stamina  = char_info.stamina as i32;
                self.idmap         = char_info.idmap as i32;
            }
        } else {
            self.dead = true;
        }
    }

    pub fn update_local(&mut self) -> bool {
        let mut state = AlKeyboardState::default();
        al_get_keyboard_state(&mut state);

        let old = (self.obj.d, self.obj.d2, self.obj.a, self.obj.a2);

        self.obj.d2 = 0;

        if al_key_down(&mut state, ALLEGRO_KEY_UP) {
            self.obj.d2 |= GDPUP;
            self.obj.d = GDPUP;
        }

        if al_key_down(&mut state, ALLEGRO_KEY_DOWN) {
            self.obj.d2 |= GDPDOWN;
            self.obj.d = GDPDOWN;
        }

        if al_key_down(&mut state, ALLEGRO_KEY_LEFT) {
            self.obj.d2 |= GDPLEFT;
            self.obj.d = GDPLEFT;
        }

        if al_key_down(&mut state, ALLEGRO_KEY_RIGHT) {
            self.obj.d2 |= GDPRIGHT;
            self.obj.d = GDPRIGHT;
        }

        if al_key_down(&mut state, ALLEGRO_KEY_D) {
            self.obj.a2 |= 1;
        } else {
            self.obj.a2 &= !1;
        }

        if al_key_down(&mut state, ALLEGRO_KEY_F) {
            self.obj.a2 |= 2;
        } else {
            self.obj.a2 &= !2;
        }

        if self.obj.a2 & 2 != 0 {
            self.obj.a = 3;
        } else if self.obj.d2 != 0 {
            self.obj.a = 0;
        } else if self.obj.a2 & 1 != 0 {
            self.obj.a = 2;
        } else {
            self.obj.a = 1;
        }

        old != (self.obj.d, self.obj.d2, self.obj.a, self.obj.a2)
    }

    pub fn send(&self, opchar: OpcaoChar, opmap: i32, chan: &Chan<PacketCharInfo>) {
        let mut char_info = PacketCharInfo {
            numchar:        opchar as i16,
            idchar:         self.obj.id as i16,
            a:              self.obj.a as i16,
            d:              self.obj.d as i16,
            x:              (self.obj.x as i32  + self.act[self.obj.a as usize].rebatex.unwrap_or(0)) as i16,
            w:              (self.obj.wd as i32 - self.act[self.obj.a as usize].rebatex.unwrap_or(0)) as i16,
            y:              (self.obj.y as i32  + self.act[self.obj.a as usize].rebatey.unwrap_or(0)) as i16,
            h:              (self.obj.hd as i32 - self.act[self.obj.a as usize].rebatey.unwrap_or(0)) as i16,
            healt:          self.info.healt as i16,
            stamina:        self.info.stamina as i16,
            damage:         self.act[self.obj.a as usize].damage as i16,
            exit:           false,
            idmap:          opmap as i16,
            totlifeless:    self.totlifeless as i16,
            listlifeless:   Default::default(),

            dhit: 0i16,
            step: 0i16,
            totchar: 0i16,
            totenemies: 0i16,
            vision: 0i16,
        };

        for (i, lifeless) in self.listlifeless.iter().enumerate() {
            if !lifeless.dead {
                char_info.listlifeless[i] = Some(PacketLifelessInfo {
                    x: lifeless.obj.x as i16,
                    y: lifeless.obj.y as i16,
                    w: lifeless.obj.w as i16,
                    h: lifeless.obj.h as i16,
                    d: lifeless.obj.d as i16,
                    damage: lifeless.actions[lifeless.obj.a as usize].damage as i16,
                });
            }
        }
        
        chan.0.send(char_info).unwrap();
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