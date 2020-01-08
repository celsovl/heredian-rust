pub mod events;
pub mod display;
pub mod audio;
pub mod audiocodecs;
pub mod color;
pub mod graphics;
pub mod file;
pub mod fullscreen;
pub mod keyboard;
pub mod font;
pub mod image;
pub mod threads;
pub mod time;
pub mod timer;
pub mod transformations;
pub mod utf8;
pub mod primitives;

use std::ffi::c_void;
use std::os::raw::c_char;

use time::AlTimeout;

pub const ALLEGRO_WINDOWED: u32 = 1 << 0;
pub const ALLEGRO_FULLSCREEN: u32 = 1 << 1;

pub const ALLEGRO_VERSION: u32 = 5;
pub const ALLEGRO_SUB_VERSION: u32 = 2;
pub const ALLEGRO_WIP_VERSION: u32 = 0;
pub const ALLEGRO_RELEASE_NUMBER: u32 = 0;
pub const ALLEGRO_VERSION_INT: u32 = ALLEGRO_VERSION<<24 | ALLEGRO_SUB_VERSION<<16 | ALLEGRO_WIP_VERSION<<8 | ALLEGRO_RELEASE_NUMBER;

pub type AlJoystick = c_void;
pub type AlMouse = c_void;
pub type AlTouchInput = c_void;

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_install_system(version: u32, atexit: unsafe extern fn(extern fn()) -> i32) -> bool;
    pub fn al_init_primitives_addon() -> bool;
    pub fn al_get_errno() -> i32;
}

pub fn al_init() -> bool {
    unsafe {
        return al_install_system(ALLEGRO_VERSION_INT, libc::atexit)
    }
}

