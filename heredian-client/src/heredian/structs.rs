use std::ptr;
use std::collections::VecDeque;
use std::path::Path;

use heredian_lib::*;
use heredian_lib::allegro_safe::*;
use heredian_lib::file_manager::ConfigFile;
use heredian_lib::net::{Client};

pub const VOLUME: f32 = 0.2;
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

#[derive(Default, Debug)]
pub struct Rect(i32, i32, i32, i32);

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
    
    pub fps: f64,
    pub last_time: f64,

    pub boss_char_id: usize,
    pub local_char_id: usize,

    pub list_chars: Vec<Char>,
    pub list_lifeless: Vec<Lifeless>,

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

    pub fn avg_fps(&mut self) -> f64 {
        const ALPHA: f64 = 0.01;

        let cur_time = al_get_time();
        let fps = 1.0 / (cur_time - self.last_time);

        self.fps = if self.fps > 1.0 {
                        ALPHA*fps + (1.0-ALPHA)*self.fps
                    } else {
                        fps
                    };

        self.last_time = cur_time;
        self.fps
    }

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

    pub fn update_local_char(&mut self, client: &Client<PacketCharInfo>) {
        let ambient = self.ambient.as_ref().unwrap();

        let state_data = (
            self.height,
            ambient.w,
            ambient.h,
            ambient.wd,
            ambient.hd,
            ambient.model
        );

        let local_char = self.get_localchar_mut().expect("Cannot find local char.");

        if local_char.update_local(state_data) {
            local_char.send(client);
        }
    }

    pub fn update_char(&mut self, char_info: PacketCharInfo) {
        let fn_find = |v: &&mut Char| v.obj.id == char_info.idchar as i32;
        match self.list_chars.iter_mut().find(fn_find) {
            Some(c) => {
                if c.dead && char_info.numchar <= 4 {
                    let new_char = Char::load(char_info.numchar as i32);
                    *c = new_char;
                }

                c.update(char_info);
            },
            None => {
                let new_char = Char::load(char_info.numchar as i32);
                self.list_chars.push(new_char);
            }
        }
    }

    pub fn try_change_ambient(&mut self) {
        let ambient = self.ambient.as_ref().unwrap();
        let local_char = self.get_localchar().unwrap();

        let gate = ambient.crossed_gate(&local_char.obj);
        
        if let Some(gate) = gate {
            let gate_info = (gate.ex, gate.ey, gate.ambient_id);
            let local_char = self.get_localchar_mut().unwrap();

            local_char.obj.x = gate_info.0 as f32;
            local_char.obj.y = gate_info.1 as f32;
            local_char.idmap = gate_info.2;

            let new_ambient = Scene::load(gate_info.2, self.width, self.height);
            self.ambient = Some(new_ambient);
        }
    }
}

#[derive(Default, Debug)]
pub struct Sprite {
    pub w: i32,
    pub h: i32,
    pub ix: i32,
    pub iy: i32,
    pub last: bool,
    pub first: bool,
    pub rect: Rect,
}

#[derive(Default, Debug)]
pub struct Object {
    pub id: i32,
    pub idchar: i32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub wd: f32,
    pub hd: f32,
    pub d: i32,
    pub d2: i32,
    pub a: i32,
    pub lock: bool,
    pub a2: u32,
    pub r#type: i32,
}

#[derive(Debug)]
pub struct Action {
    pub id: i32,
    pub fps: f64,
    pub directions: [VecDeque<Sprite>; 4],
    pub image: *const AlBitmap,
    pub fila_timer: *const AlEventQueue,
    pub sound: Option<*const AlSample>,
    pub stepx: i32,
    pub stepy: i32,
    pub damage: i32,
    pub lock: bool,
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
    pub list_lifeless: Vec<Lifeless>,
}

#[derive(Debug)]
pub struct Info {
    pub image: *const AlBitmap,
    pub w: i32,
    pub h: i32,
    pub fonte: *const AlFont,
    pub color: AlColor,
}

impl Drop for Info {
    fn drop(&mut self) {
        if !self.image.is_null() {
            al_destroy_bitmap(self.image);
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

    pub fn crossed(&self, obj: &Object) -> bool {
        obj.x + obj.wd >= self.x1 as f32 &&
        obj.x <= self.x2 as f32 &&
        obj.y + obj.hd >= self.y1 as f32 &&
        obj.y + obj.hd <= self.y2 as f32
    }
}

impl Info {
    pub fn from_config(width: i32) -> Info {
        Info {
            fonte: al_load_font("assets/Fonts/font_info.TTF", 12, 0),
            h: 100,
            w: width,
            color: al_map_rgb(255,255,255),
            image: al_load_bitmap("assets/Images/info.png"),
        }
    }
}

impl Sprite {
    pub fn from_config(config_file: &ConfigFile) -> [VecDeque<Sprite>; 4] {
        let qt_sprites = config_file.get("qt_sprites").expect("qt_sprites não encontrado");

        let ntamx = config_file.get("size_x").expect("size_x não encontrado");
        let ntamy = config_file.get("size_y").expect("size_y não encontrado");

        let mut all_sprites = [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()];

        for i in 0..4 {
            let sprites = &mut all_sprites[i];

            if qt_sprites == -1 {
                sprites.push_front(Sprite {
                    ix: 0,
                    iy: 0,
                    first: true,
                    last: true,
                    h: 0,
                    w: 0,
                    rect: Default::default(),
                });
            }

            for j in (0..qt_sprites).rev() {
                let mut sprite = Sprite {
                    ix: j,
                    iy: i as i32,
                    w: ntamx,
                    h: ntamy,
    
                    last: false,
                    first: false,
                    rect: Default::default(),
                };

                if j == qt_sprites-1 {
                    sprite.last = true;
                } else if j == 0 {
                    sprite.first = true;
                }

                sprites.push_front(sprite);
            }
        }

        all_sprites
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
            lock: false,
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

        let mut directions = Sprite::from_config(&action_config_file);
        Self::adjust_rects(pri, &mut directions);

        Action {
            id: nacao,
            image: pri,
            stepx: action_config_file.get("stepx").expect("stepx não encontrado."),
            stepy: action_config_file.get("stepy").expect("stepy não encontrado."),
            fps: fps,
            lifelessid: action_config_file.get::<i32>("lifelessid").and_then(|v| if v > 0 { Some(v) } else { None } ),
            rebatex: action_config_file.get("rebatex"),
            rebatey: action_config_file.get("rebatey"),
            fila_timer: fila_timer,

            sound: sound,

            charge: action_config_file.get("charge"),
            damage: action_config_file.get("damage").expect("damage não encontrado."),
            lock: action_config_file.get::<i32>("lock").expect("lock não encontrado.") != 0,

            directions: directions,
        }
    }

    pub fn adjust_rects(image: *const AlBitmap, directions: &mut [VecDeque<Sprite>]) {

        al_lock_bitmap(
            image,
            al_get_bitmap_format(image), 
            ALLEGRO_LOCK_READONLY);

        for direction in directions {
            for sprite in direction {
                let mut rect = Rect(std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN);

                let frame = al_create_sub_bitmap(
                    image,
                    sprite.w*sprite.ix,
                    sprite.h*sprite.iy,
                    sprite.w,
                    sprite.h);

                for x in 0..sprite.w {
                    for y in 0..sprite.h {
                        let color = al_get_pixel(frame, x, y);
                        let is_transparent = color.a == 0.0;
                        if !is_transparent {
                            rect.0 = rect.0.min(x);
                            rect.1 = rect.1.min(y);
                            rect.2 = rect.2.max(x);
                            rect.3 = rect.3.max(y);
                        }
                    }
                }

                al_destroy_bitmap(frame);
                sprite.rect = rect;
            }
        }

        al_unlock_bitmap(image);
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

    pub fn draw(&mut self) {
        let d = match self.obj.d {
            GDPLEFT => 1,
            GDPRIGHT => 2,
            GDPUP => 3,
            GDPDOWN => 0,
            _ => 2
        };

        let act = &mut self.actions[self.obj.a as usize];
        let sprites = &act.directions[d];
        let sprite = sprites.front().unwrap();

        // se existir, reproduz o som
        if let Some(sound) = act.sound {
            al_play_sample(sound, VOLUME * 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());
        }

        let frame = al_create_sub_bitmap(
            act.image,
            sprite.w * sprite.ix,
            sprite.h * sprite.iy,
            sprite.w,
            sprite.h);

        // desenha o sprite
        al_draw_scaled_bitmap(
            frame,
            0.0,
            0.0,
            sprite.w as f32,
            sprite.h as f32,
            self.obj.x,
            self.obj.y,
            self.obj.wd,
            self.obj.hd,
            0);
        
        al_destroy_bitmap(frame);

        if !al_is_event_queue_empty(act.fila_timer) {
            let mut event = AlEvent::default();
            al_wait_for_event(act.fila_timer, &mut event);

            if event.get_type() == AlEventType::ALLEGRO_EVENT_TIMER {
                let sprites = &mut act.directions[d];
                sprites.rotate_left(1);

                al_flush_event_queue(act.fila_timer);
            }
        }
  }

    pub fn update(&mut self, state_data: (i32, i32, i32, i32, i32, *const AlBitmap)) {
        let d = match self.obj.d {
            GDPLEFT => 1,
            GDPRIGHT => 2,
            GDPUP => 3,
            GDPDOWN => 0,
            _ => 2
        };

        let act = &self.actions[self.obj.a as usize];
        let sprites = &act.directions[d];
        let sprite = sprites.front().unwrap();

        let (old_x, old_y) = (self.obj.x, self.obj.y);

        self.obj.wd = self.obj.w * sprite.w as f32;
        self.obj.hd = self.obj.h * sprite.h as f32;

        if self.obj.d == GDPUP {
            self.obj.y -= act.stepy as f32;
        }

        if self.obj.d == GDPDOWN {
            self.obj.y += act.stepy as f32;
        }

        if self.obj.d == GDPLEFT {
            self.obj.x -= act.stepx as f32;
        }

        if self.obj.d == GDPRIGHT {
            self.obj.x += act.stepx as f32;
        }

        // move back if collided
        if self.collided(state_data) {
            if self.obj.d == GDPUP {
                self.obj.y += act.stepy as f32;
            }

            if self.obj.d == GDPDOWN {
                self.obj.y -= act.stepy as f32;
            }

            if self.obj.d == GDPLEFT {
                self.obj.x += act.stepx as f32;
            }

            if self.obj.d == GDPRIGHT {
                self.obj.x -= act.stepx as f32;
            }
        }

        // collided, because it wasn't able to move - so destroy it
        if (old_x, old_y) == (self.obj.x, self.obj.y) {
            self.dead = true;
        }
    }

    pub fn collided(&self, state_data: (i32, i32, i32, i32, i32, *const AlBitmap)) -> bool {
        let (
            height, 
            amb_w, 
            amb_h, 
            amb_wd, 
            amb_hd, 
            amb_model) = state_data;

        if self.obj.y < 0.0 || self.obj.x < 0.0 {
            return true;
        }

        if (self.obj.y + self.obj.hd) > height as f32 {
            return true;
        }

        let colorwall = al_map_rgb(0, 0, 0);

        let sx = amb_w as f32 / amb_wd as f32;
        let sy = amb_h as f32 / amb_hd as f32;

        let we = amb_wd as f32 * sx;
        let he = amb_hd as f32 * sy;

        let xup   = (self.obj.x + self.obj.wd) * sx;
        let yup   = self.obj.y * sy;

        let xdown = self.obj.x * sx;
        let ydown = (self.obj.y + self.obj.hd) * sy;

        if xdown >= 0.0 && ydown >= 0.0 && xup <= we && yup <= he {
            if self.obj.d != GDPRIGHT {
                let color = al_get_pixel(amb_model, xdown as i32, ydown as i32);
                if colorwall == color {
                    return true;
                }
            }

            if self.obj.d != GDPLEFT {
                let color = al_get_pixel(amb_model, xup as i32, ydown as i32);
                if colorwall == color {
                    return true;
                }
            }
        } else {
            return true;
        }

        return false;
    }
}

impl Char {
    pub fn load(id: i32) -> Char {
        let path = Path::new("assets/Configs/Chars.txt");
        let chars_config_file = ConfigFile::load(path);

        let char_path = chars_config_file.get_string(&id.to_string()).expect("Char não encontrado.");
        let config_file = ConfigFile::load(char_path);

        let obj = Object::from_config(&config_file, id);
        let info = InfoChar::from_config(&config_file);
        let acoes = Action::from_config(&config_file);

        Char {
            idmap: -1,
            dead: false,
            act: acoes,
            obj: obj,
            info: info,
            list_lifeless: Vec::with_capacity(10),
        }
    }

    pub fn update(&mut self, char_info: PacketCharInfo) {
        match char_info.dhit as i32 {
            GDPUP => self.obj.y -= 1.0,
            GDPDOWN => self.obj.y += 1.0,
            GDPLEFT => self.obj.x -= 1.0,
            GDPRIGHT => self.obj.x += 1.0,
            _ => ()
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

    pub fn update_local(&mut self, state_data: (i32, i32, i32, i32, i32, *const AlBitmap)) -> bool {
        let mut kb_state = AlKeyboardState::default();
        al_get_keyboard_state(&mut kb_state);

        let old = (self.obj.d, self.obj.d2, self.obj.a, self.obj.a2);

        self.obj.d2 = 0;

        if al_key_down(&mut kb_state, ALLEGRO_KEY_UP) {
            self.obj.d2 |= GDPUP;
            self.obj.d = GDPUP;
        }

        if al_key_down(&mut kb_state, ALLEGRO_KEY_DOWN) {
            self.obj.d2 |= GDPDOWN;
            self.obj.d = GDPDOWN;
        }

        if al_key_down(&mut kb_state, ALLEGRO_KEY_LEFT) {
            self.obj.d2 |= GDPLEFT;
            self.obj.d = GDPLEFT;
        }

        if al_key_down(&mut kb_state, ALLEGRO_KEY_RIGHT) {
            self.obj.d2 |= GDPRIGHT;
            self.obj.d = GDPRIGHT;
        }

        if al_key_down(&mut kb_state, ALLEGRO_KEY_D) {
            self.obj.a2 |= 1;
        } else {
            self.obj.a2 &= !1;
        }

        if al_key_down(&mut kb_state, ALLEGRO_KEY_F) {
            self.obj.a2 |= 2;
        } else if !self.obj.lock {
            self.obj.a2 &= !2;
        }

        if self.obj.a2 & 2 != 0 {
            self.obj.a = 3;
        } else if self.obj.d2 == 0 {
            self.obj.a = 0;
        } else if self.obj.a2 & 1 != 0 {
            self.obj.a = 2;
        } else {
            self.obj.a = 1;
        }

        let act = &self.act[self.obj.a as usize];

        // create magic if needed
        let d = match self.obj.d {
            GDPLEFT => 1,
            GDPRIGHT => 2,
            GDPUP => 3,
            GDPDOWN => 0,
            _ => 2
        };

        let sprites = &act.directions[d];
        let sprite = sprites.front().unwrap();

        if !self.obj.lock && sprite.first && self.list_lifeless.len() < MAXCHARLIFELESS {
            if let Some(id) = act.lifelessid {
                let mut lifeless = Lifeless::load(id);
                lifeless.idmap = self.idmap;
                lifeless.obj.idchar = self.obj.idchar;
                lifeless.obj.d = self.obj.d;
                lifeless.obj.d2 = self.obj.d2;
                lifeless.obj.x = self.obj.x;
                lifeless.obj.y = self.obj.y;
                self.list_lifeless.push(lifeless);
            }
        }

        if self.obj.d2 & GDPUP != 0 {
            self.obj.y -= act.stepy as f32;
        }

        if self.obj.d2 & GDPDOWN != 0 {
            self.obj.y += act.stepy as f32;
        }

        if self.obj.d2 & GDPLEFT != 0 {
            self.obj.x -= act.stepx as f32;
        }

        if self.obj.d2 & GDPRIGHT != 0 {
            self.obj.x += act.stepx as f32;
        }

        if self.info.stamina <= 0 {
            self.obj.a = 0;
        }

        if self.info.healt <= 0 {
            self.obj.a = 4;
            self.info.healt = 0;
        }

        // update lifeless if needed
        for lifeless in self.list_lifeless.iter_mut() {
            lifeless.update(state_data);
        }

        // clear lifeless dead
        self.list_lifeless.retain(|l| !l.dead);

        // move back if collided
        if self.collided(state_data) {
            if self.obj.d2 & GDPUP != 0 {
                self.obj.y += act.stepy as f32;
            }

            if self.obj.d2 & GDPDOWN != 0 {
                self.obj.y -= act.stepy as f32;
            }

            if self.obj.d2 & GDPLEFT != 0 {
                self.obj.x += act.stepx as f32;
            }

            if self.obj.d2 & GDPRIGHT != 0 {
                self.obj.x -= act.stepx as f32;
            }
        }

        old != (self.obj.d, self.obj.d2, self.obj.a, self.obj.a2)
    }

    pub fn send(&self, client: &Client<PacketCharInfo>) {
        let mut char_info = PacketCharInfo {
            numchar:        self.obj.r#type as i16,
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
            idmap:          self.idmap as i16,
            totlifeless:    0i16,
            listlifeless:   Default::default(),

            dhit: 0i16,
            step: 0i16,
            totchar: 0i16,
            totenemies: 0i16,
            vision: 0i16,
        };

        for (i, lifeless) in self.list_lifeless.iter().enumerate() {
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
        
        client.send(char_info);
    }

    pub fn draw(&mut self) {
        let a = self.obj.a as usize;

        let d = match self.obj.d {
            GDPLEFT => 1,
            GDPRIGHT => 2,
            GDPUP => 3,
            GDPDOWN => 0,
            _ => 2
        };

        if let Some(sound) = self.act[a].sound {
            al_play_sample_b(sound, VOLUME * 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_ONCE, ptr::null_mut());
        }

        let sprites = &self.act[a].directions[d];
        let sprite = sprites.front().unwrap();

        if self.act[a].lock {
            self.obj.lock = true;
        }

        let sprite_size = (sprite.rect.2 - sprite.rect.0, sprite.rect.3 - sprite.rect.1);

        // adjust object width / height
        self.obj.wd = self.obj.w * (sprite.rect.2 - sprite.rect.0) as f32;
        self.obj.hd = self.obj.h * (sprite.rect.3 - sprite.rect.1) as f32;
        
        //gdp_movsprite(&tchar->obj,&oactions[acao]);

        let frame = al_create_sub_bitmap(
            self.act[a].image,
            sprite.w*sprite.ix + sprite.rect.0,
            sprite.h*sprite.iy + sprite.rect.1,
            sprite_size.0,
            sprite_size.1);

        // desenha o sprite
        al_draw_scaled_bitmap(
            frame,
            0.0,
            0.0,
            sprite_size.0 as f32,
            sprite_size.1 as f32,
            self.obj.x,
            self.obj.y,
            self.obj.wd,
            self.obj.hd,
            0);

        // draw green bounding box
        al_draw_rectangle(
            self.obj.x, 
            self.obj.y, 
            self.obj.x + self.obj.wd, 
            self.obj.y + self.obj.hd, 
            al_map_rgb(0,255,0), 
            1.0);

        let (xdown, ydown, xup) = (
            self.obj.x,
            (self.obj.y + self.obj.hd),
            (self.obj.x + self.obj.wd));

        al_draw_circle(xdown, ydown, 2.0, al_map_rgb(0, 0, 255), 1.0);
        al_draw_circle(xup, ydown, 2.0, al_map_rgb(0, 0, 255), 1.0);

        al_destroy_bitmap(frame);

        // draw lifeless if needed
        for lifeless in self.list_lifeless.iter_mut() {
            lifeless.draw();
        }


        // verifica se deu tempo para troca de sprite
        if !al_is_event_queue_empty(self.act[a].fila_timer) {
            let mut charevento = AlEvent::default();
            al_wait_for_event(self.act[a].fila_timer, &mut charevento);

            // muda o sprite
            if charevento.get_type() == AlEventType::ALLEGRO_EVENT_TIMER {
                // verica se pode libera a movimentacao
                if self.obj.lock && sprite.last {
                    self.obj.lock = false;
                    self.obj.a = 0;
                    self.obj.a2 &= !2;
                }

                // se for o ultimo sprite e o objeto não estiver travado, gasta stamina
                if sprite.last || !self.obj.lock {
                    // gasta a stamina
                    self.info.stamina += self.act[a].charge.unwrap_or(0);
                    self.info.stamina = self.info.stamina.min(self.info.staminafull).max(0);
                }

                // muda de sprite
                let sprites = &mut self.act[a].directions[d];
                sprites.rotate_left(1);

                al_flush_event_queue(self.act[a].fila_timer);
            }
        }
    }

    pub fn draw_info(&self, state: &GameState, i: i32) {
        let space = state.width/4; 
        let ambient = state.ambient.as_ref().unwrap();

        // draw info box
        al_draw_scaled_bitmap(
            ambient.info.image,
            0.0,
            0.0,
            255.0,
            147.0,
            (space*i) as f32,
            0.0,
            space as f32,
            100.0,
            0);
        
        let data = [
            (format!("Nome: {}", self.info.name), 15.0),
            (format!("Saude: {}/{}", self.info.healt, self.info.healtfull), 35.0),
            (format!("Cansaço: {}/{}", self.info.stamina, self.info.staminafull), 55.0),
        ];

        for datum in data.iter() {
            al_draw_text(
                ambient.info.fonte, 
                ambient.info.color, 
                (20+space*i) as f32, 
                datum.1, 
                ALLEGRO_ALIGN_LEFT, 
                &datum.0);
        }
    }

    pub fn collided(&self, state_data: (i32, i32, i32, i32, i32, *const AlBitmap)) -> bool {
        let (
            height, 
            amb_w, 
            amb_h, 
            amb_wd, 
            amb_hd, 
            amb_model) = state_data;

        if self.obj.y < 0.0 || self.obj.x < 0.0 {
            return true;
        }

        if (self.obj.y + self.obj.hd) > height as f32 {
            return true;
        }

        let colorwall = al_map_rgb(0, 0, 0);

        let sx = amb_w as f32 / amb_wd as f32;
        let sy = amb_h as f32 / amb_hd as f32;

        let we = amb_wd as f32 * sx;
        let he = amb_hd as f32 * sy;

        let xup   = (self.obj.x + self.obj.wd) * sx;
        let yup   = self.obj.y * sy;

        let xdown = self.obj.x * sx;
        let ydown = (self.obj.y + self.obj.hd) * sy;

        if xdown >= 0.0 && ydown >= 0.0 && xup <= we && yup <= he {
            if self.obj.d != GDPRIGHT {
                let color = al_get_pixel(amb_model, xdown as i32, ydown as i32);
                if colorwall == color {
                    return true;
                }
            }

            if self.obj.d != GDPLEFT {
                let color = al_get_pixel(amb_model, xup as i32, ydown as i32);
                if colorwall == color {
                    return true;
                }
            }
        } else {
            return true;
        }

        return false;
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

        al_play_sample_b(sound, VOLUME * 1.0, 0.0, 1.0, AlPlaymode::ALLEGRO_PLAYMODE_LOOP, ptr::null_mut());

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

    pub fn crossed_gate(&self, obj: &Object) -> Option<&Gate> {
        let mut crossed = None;

        for gate in self.gates.iter() {
            if gate.crossed(obj) {
                crossed = Some(gate);
                break;
            }
        }

        crossed
    }

    pub fn draw(&self) {
        // imagem de fundo
        al_draw_scaled_bitmap(self.image,
            0.0, 0.0,
            self.w as f32,
            self.h as f32,
            0.0,
            0.0, //ambient->info->h
            self.wd as f32,
            self.hd as f32,
            0
        );

        //mostra portoes
        for gate in self.gates.iter() {
            al_draw_filled_rectangle(
                gate.x1 as f32,
                gate.y1 as f32,
                gate.x2 as f32,
                gate.y2 as f32,
                al_map_rgba(255, 98, 100,0));
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