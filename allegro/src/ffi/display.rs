use super::*;
use super::events::{AlEventSource};
use super::graphics::AlBitmap;

pub type AlDisplay = c_void;

pub const ALLEGRO_WINDOWED: i32                    = 1 << 0;
pub const ALLEGRO_FULLSCREEN: i32                  = 1 << 1;
pub const ALLEGRO_OPENGL: i32                      = 1 << 2;
pub const ALLEGRO_DIRECT3D_INTERNAL: i32           = 1 << 3;
pub const ALLEGRO_RESIZABLE: i32                   = 1 << 4;
pub const ALLEGRO_FRAMELESS: i32                   = 1 << 5;
pub const ALLEGRO_NOFRAME: i32                     = ALLEGRO_FRAMELESS; /* older synonym */
pub const ALLEGRO_GENERATE_EXPOSE_EVENTS: i32      = 1 << 6;
pub const ALLEGRO_OPENGL_3_0: i32                  = 1 << 7;
pub const ALLEGRO_OPENGL_FORWARD_COMPATIBLE: i32   = 1 << 8;
pub const ALLEGRO_FULLSCREEN_WINDOW: i32           = 1 << 9;
pub const ALLEGRO_MINIMIZED: i32                   = 1 << 10;
pub const ALLEGRO_PROGRAMMABLE_PIPELINE: i32       = 1 << 11;
pub const ALLEGRO_GTK_TOPLEVEL_INTERNAL: i32       = 1 << 12;
pub const ALLEGRO_MAXIMIZED: i32                   = 1 << 13;
pub const ALLEGRO_OPENGL_ES_PROFILE: i32           = 1 << 14;

#[link(name="liballegro_monolith.dll")]
extern {
    // Display creation
    pub fn al_create_display(width: i32, height: i32) -> *const AlDisplay;
    pub fn al_destroy_display(display: *const AlDisplay);
    pub fn al_get_new_display_flags() -> i32;
    pub fn al_set_new_display_flags(flags: i32);
    pub fn al_get_new_display_option(option: i32, importance: *const i32) -> i32;
    pub fn al_set_new_display_option(option: i32, value: i32, importance: i32);
    pub fn al_reset_new_display_options();
    pub fn al_get_new_window_position(x: *mut i32, y: *mut i32);
    pub fn al_set_new_window_position(x: i32, y: i32);
    pub fn al_get_new_display_refresh_rate() -> i32;
    pub fn al_set_new_display_refresh_rate(refresh_rate: i32);

    // Display operation
    pub fn al_get_display_event_source(display: *const AlDisplay) -> *const AlEventSource;
    pub fn al_get_backbuffer(display: *const AlDisplay) -> *const AlBitmap;
    pub fn al_flip_display();
    pub fn al_update_display_region(x: i32, y: i32, width: i32, height: i32);
    pub fn al_wait_for_vsync() -> bool;

    // Display size and position
    pub fn al_get_display_width(display: *const AlDisplay) -> i32;
    pub fn al_get_display_height(display: *const AlDisplay) -> i32;
    pub fn al_resize_display(display: *const AlDisplay, width: i32, height: i32) -> bool;
    pub fn al_acknowledge_resize(display: *const AlDisplay) -> bool;
    pub fn al_get_window_position(display: *const AlDisplay, x: *mut i32, y: *mut i32);
    pub fn al_set_window_position(display: *const AlDisplay, x: i32, y: i32);
    pub fn al_get_window_constraints(display: *const AlDisplay, min_w: *mut i32, min_h: *mut i32, max_w: *mut i32, max_h: *mut i32) -> bool;
    pub fn al_set_window_constraints(display: *const AlDisplay, min_w: i32, min_h: i32, max_w: i32, max_h: i32) -> bool;
    pub fn al_apply_window_constraints(display: *const AlDisplay, onoff: bool);

    // Display settings
    pub fn al_get_display_flags(display: *const AlDisplay) -> i32;
    pub fn al_set_display_flag(display: *const AlDisplay, flag: i32, onoff: bool) -> bool;
    pub fn al_get_display_option(display: *const AlDisplay, option: i32) -> i32;
    pub fn al_set_display_option(display: *const AlDisplay, option: i32, value: i32);
    pub fn al_get_display_format(display: *const AlDisplay) -> i32;
    pub fn al_get_display_orientation(display: *const AlDisplay) -> i32;
    pub fn al_get_display_refresh_rate(display: *const AlDisplay) -> i32;
    pub fn al_set_window_title(display: *const AlDisplay, title: *const c_char);
    pub fn al_set_new_window_title(title: *const c_char);
    pub fn al_get_new_window_title() -> *const c_char;
    pub fn al_set_display_icon(display: *const AlDisplay, icon: *const AlBitmap);
    pub fn al_set_display_icons(display: *const AlDisplay, num_icons: i32, icons: *const *const AlBitmap);

    // Drawing halts
    pub fn al_acknowledge_drawing_halt(display: *const AlDisplay);
    pub fn al_acknowledge_drawing_resume(display: *const AlDisplay);

    // Screensaver
    pub fn al_inhibit_screensaver(inhibit: bool) -> bool;

    // Clipboard
    pub fn al_get_clipboard_text(display: *const AlDisplay) -> *const c_char;
    pub fn al_set_clipboard_text(display: *const AlDisplay, text: *const c_char) -> bool;
    pub fn al_clipboard_has_text(display: *const AlDisplay) -> bool;
}
