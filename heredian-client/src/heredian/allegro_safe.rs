use allegro::ffi;
use std::ffi::CString;

pub use ffi::al_init;
pub use ffi::display::{
    AlDisplay, ALLEGRO_FULLSCREEN_WINDOW, ALLEGRO_FULLSCREEN
};
pub use ffi::graphics::{
    AlBitmap, AlColor, AlLockedRegion,
    ALLEGRO_LOCK_READONLY, ALLEGRO_MEMORY_BITMAP
};
pub use ffi::events::{
    AlEvent, AlEventType, AlEventSource, AlEventQueue, AlKeyboardEvent
};
pub use ffi::audio::{
    AlSample, AlPlaymode, AlSampleID
};
pub use ffi::font::{
    AlFont, al_draw_textf, ALLEGRO_ALIGN_CENTRE, ALLEGRO_ALIGN_LEFT
};
pub use ffi::timer::{AlTimer};
pub use ffi::keyboard::{
    AlKeyboardState,
    ALLEGRO_KEY_UP, ALLEGRO_KEY_DOWN, ALLEGRO_KEY_LEFT, 
    ALLEGRO_KEY_RIGHT, ALLEGRO_KEY_ENTER, ALLEGRO_KEY_D,
    ALLEGRO_KEY_F, ALLEGRO_KEY_1, ALLEGRO_KEY_2
};
pub use ffi::transformations::{
    AlTransform
};

pub fn al_set_window_title(display: *const AlDisplay, title: &str) {
    unsafe { ffi::display::al_set_window_title(display, CString::new(title).unwrap().as_ptr()) }
}

pub fn al_load_sample(filename: &str) -> *const AlSample {
    let res = unsafe { ffi::audio::al_load_sample(CString::new(filename).unwrap().as_ptr()) };
    if res.is_null() {
        panic!("Can't load sample {:?}.", filename);
    }

    res
}

pub fn al_load_bitmap(filename: &str) -> *const AlBitmap {
    let res = unsafe { ffi::graphics::al_load_bitmap(CString::new(filename).unwrap().as_ptr()) };
    if res.is_null() {
        panic!("Can't load bitmap {:?}.", filename);
    }

    res
}

pub fn al_create_timer(speed_secs: f64) -> *const AlTimer {
    let res = unsafe { ffi::timer::al_create_timer(speed_secs) };
    if res.is_null() {
        panic!("Can't create timer for {}s.", speed_secs);
    }

    res
}

pub fn al_get_timer_event_source(timer: *const AlTimer) -> *const AlEventSource {
    let res = unsafe { ffi::timer::al_get_timer_event_source(timer) };
    if res.is_null() {
        panic!("Can't get timer event source.");
    }

    res
}

pub fn al_get_display_event_source(display: *const AlDisplay) -> *const AlEventSource {
    let res = unsafe { ffi::display::al_get_display_event_source(display) };
    if res.is_null() {
        panic!("Can't get display event source.");
    }

    res
}

pub fn al_get_keyboard_event_source() -> *const AlEventSource {
    unsafe { ffi::keyboard::al_get_keyboard_event_source() }
}

pub fn al_create_event_queue() -> *const AlEventQueue {
    let res = unsafe { ffi::events::al_create_event_queue() };
    if res.is_null() {
        panic!("Can't create event queue.");
    }

    res
}

pub fn al_register_event_source(queue: *const AlEventQueue, source: *const AlEventSource) {
    unsafe { ffi::events::al_register_event_source(queue, source) };
}

pub fn al_unregister_event_source(queue: *const AlEventQueue, source: *const AlEventSource) {
    unsafe { ffi::events::al_unregister_event_source(queue, source) };
}

pub fn al_start_timer(timer: *const AlTimer) {
    unsafe { ffi::timer::al_start_timer(timer) };
}

pub fn al_stop_timer(timer: *const AlTimer) {
    unsafe { ffi::timer::al_stop_timer(timer) };
}

pub fn al_lock_bitmap(bitmap: *const AlBitmap, format: i32, flags: i32) -> *const AlLockedRegion {
    let res = unsafe { ffi::graphics::al_lock_bitmap(bitmap, format, flags) };
    if res.is_null() {
        panic!("Can't lock bitmap.");
    }

    res
}

pub fn al_get_bitmap_format(bitmap: *const AlBitmap) -> i32 {
    unsafe { ffi::graphics::al_get_bitmap_format(bitmap) }
}

pub fn al_get_bitmap_width(bitmap: *const AlBitmap) -> i32 {
    unsafe { ffi::graphics::al_get_bitmap_width(bitmap) }
}

pub fn al_get_bitmap_height(bitmap: *const AlBitmap) -> i32 {
    unsafe { ffi::graphics::al_get_bitmap_height(bitmap) }
}

pub fn al_get_new_bitmap_flags() -> i32 {
    unsafe { ffi::graphics::al_get_new_bitmap_flags() }
}

pub fn al_add_new_bitmap_flag(flags: i32) {
    unsafe { ffi::graphics::al_add_new_bitmap_flag(flags) };
}

pub fn al_set_new_bitmap_flags(flags: i32) {
    unsafe { ffi::graphics::al_set_new_bitmap_flags(flags) };
}

pub fn al_reserve_samples(reserve_samples: i32) {
    let res = unsafe { ffi::audio::al_reserve_samples(reserve_samples) };
    if !res {
        panic!("Can't reserve {} samples.", reserve_samples);
    }
}

pub fn al_play_sample(spl: *const AlSample, gain: f32, pan: f32 , speed: f32, r#loop: AlPlaymode, ret_id: *mut AlSampleID) {
    let res = al_play_sample_b(spl, gain, pan, speed, r#loop, ret_id);
    if !res {
        panic!("Can't play sample.");
    }
}

pub fn al_play_sample_b(spl: *const AlSample, gain: f32, pan: f32 , speed: f32, r#loop: AlPlaymode, ret_id: *mut AlSampleID) -> bool {
    unsafe { ffi::audio::al_play_sample(spl, gain, pan, speed, r#loop, ret_id) }
}

pub fn al_map_rgb(r: u8, g: u8, b: u8) -> AlColor {
    unsafe { ffi::graphics::al_map_rgb(r, g, b) }
}

pub fn al_map_rgba(r: u8, g: u8, b: u8, a: u8) -> AlColor {
    unsafe { ffi::graphics::al_map_rgba(r, g, b, a) }
}

pub fn al_draw_tinted_scaled_bitmap(bitmap: *const AlBitmap, tint: AlColor, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: i32) {
    unsafe { ffi::graphics::al_draw_tinted_scaled_bitmap(bitmap, tint, sx, sy, sw, sh, dx, dy, dw, dh, flags) };
}

pub fn al_flip_display() {
    unsafe { ffi::display::al_flip_display() };
}

pub fn al_clear_to_color(color: AlColor) {
    unsafe { ffi::graphics::al_clear_to_color(color) };
}

pub fn al_load_font(filename: &str, size: i32, flags: i32) -> *const AlFont {
    let res = unsafe { ffi::font::al_load_font(CString::new(filename).unwrap().as_ptr(), size, flags) };
    if res.is_null() {
        panic!("Can't load font {}.", filename);
    }

    res
}

pub fn al_create_display(width: i32, height: i32) -> *const AlDisplay {
    let res = unsafe { ffi::display::al_create_display(width, height) };
    if res.is_null() {
        panic!("Can't create display.");
    }

    res
}

pub fn al_destroy_timer(timer: *const AlTimer) {
    unsafe { ffi::timer::al_destroy_timer(timer) };
}

pub fn al_destroy_display(display: *const AlDisplay) {
    unsafe { ffi::display::al_destroy_display(display) };
}

pub fn al_destroy_bitmap(bitmap: *const AlBitmap) {
    unsafe { ffi::graphics::al_destroy_bitmap(bitmap) };
}

pub fn al_destroy_event_queue(queue: *const AlEventQueue) {
    unsafe { ffi::events::al_destroy_event_queue(queue) };
}

pub fn al_destroy_sample(spl: *const AlSample) {
    unsafe { ffi::audio::al_destroy_sample(spl) };
}

pub fn al_destroy_font(f: *const AlFont) {
    unsafe { ffi::font::al_destroy_font(f) };
}

pub fn al_init_font_addon() {
    unsafe {
        let res = ffi::font::al_init_font_addon();
        if !res {
            panic!("Can't init allegro font addon.");
        }
    }
}

pub fn al_init_ttf_addon() {
    unsafe {
        let res = ffi::font::al_init_ttf_addon();
        if !res {
            panic!("Can't init allegro ttf addon.");
        }
    }
}

pub fn al_init_image_addon() {
    unsafe {
        let res = ffi::image::al_init_image_addon();
        if !res {
            panic!("Can't init allegro image addon.");
        }
    }
}

pub fn al_install_audio() {
    unsafe {
        let res = ffi::audio::al_install_audio();
        if !res {
            panic!("Can't init allegro audio.");
        }
    }
}

pub fn al_install_keyboard() {
    unsafe {
        let res = ffi::keyboard::al_install_keyboard();
        if !res {
            panic!("Can't init allegro keyboard.");
        }
    }
}

pub fn al_init_primitives_addon() {
    unsafe {
        let res = ffi::primitives::al_init_primitives_addon();
        if !res {
            panic!("Can't init allegro primitives addon.");
        }
    }
}

pub fn al_init_acodec_addon() {
    unsafe {
        let res = ffi::audiocodecs::al_init_acodec_addon();
        if !res {
            panic!("Can't init allegro acodec addon.");
        }
    }
}

pub fn al_set_new_display_flags(flags: i32) {
    unsafe { ffi::display::al_set_new_display_flags(flags) };
}

pub fn al_get_errno() -> i32 {
    unsafe { ffi::al_get_errno() }
}

pub fn al_wait_for_event(queue: *const AlEventQueue, ret_event: *mut AlEvent) {
    unsafe { ffi::events::al_wait_for_event(queue, ret_event) }
}

pub fn al_rest(seconds: f64) {
    unsafe { ffi::time::al_rest(seconds) }
}

pub fn al_is_event_queue_empty(queue: *const AlEventQueue) -> bool {
    unsafe { ffi::events::al_is_event_queue_empty(queue) }
}

pub fn al_draw_text(font: *const AlFont, color: AlColor, x: f32, y: f32, flags: i32, text: &str) {
    unsafe { ffi::font::al_draw_text(font, color, x, y, flags, CString::new(text).unwrap().as_ptr()) }
}

pub fn al_draw_scaled_bitmap(bitmap: *const AlBitmap, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: i32) {
    unsafe { ffi::graphics::al_draw_scaled_bitmap(bitmap, sx, sy, sw, sh, dx, dy, dw, dh, flags) };
}

pub fn al_identity_transform(trans: *mut AlTransform) {
    unsafe { ffi::transformations::al_identity_transform(trans) };
}

pub fn al_get_keyboard_state(ret_state: *mut AlKeyboardState) {
    unsafe { ffi::keyboard::al_get_keyboard_state(ret_state) };
}

pub fn al_key_down(state: *const AlKeyboardState, keycode: i32) -> bool {
    unsafe { ffi::keyboard::al_key_down(state, keycode) }
}

pub fn al_build_transform(trans: *mut AlTransform, x: f32, y: f32, sx: f32, sy: f32, theta: f32) {
    unsafe { ffi::transformations::al_build_transform(trans, x, y, sx, sy, theta) };
}

pub fn al_use_transform(trans: *const AlTransform) {
    unsafe { ffi::transformations::al_use_transform(trans) };
}

pub fn al_draw_filled_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, color: AlColor) {
    unsafe { ffi::primitives::al_draw_filled_rectangle(x1, y1, x2, y2, color) };
}

pub fn al_get_time() -> f64 {
    unsafe { ffi::time::al_get_time() }
}

pub fn al_create_sub_bitmap(parent: *const AlBitmap, x: i32, y: i32, w: i32, h: i32) -> *const AlBitmap {
    let res = unsafe { ffi::graphics::al_create_sub_bitmap(parent, x, y, w, h) };
    if res.is_null() {
        panic!("Can't create sub bitmap.");
    }

    res
}

pub fn al_flush_event_queue(queue: *const AlEventQueue) {
    unsafe { ffi::events::al_flush_event_queue(queue) }
}

pub fn al_get_pixel(bitmap: *const AlBitmap, x: i32, y: i32) -> AlColor {
    unsafe { ffi::graphics::al_get_pixel(bitmap, x, y) }
}