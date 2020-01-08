use std::os::raw::c_char;
use std::ffi::c_void;

use super::utf8::AlUStr;
use super::graphics::AlColor;
use super::file::AlFile;
use super::graphics::AlBitmap;

pub type AlFont = c_void;

#[repr(C)]
pub struct AlGlyph {
    bitmap: *const AlBitmap,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    kerning: i32,
    offset_x: i32,
    offset_y: i32,
    advance: i32,   
}

impl Default for AlGlyph {
    fn default() -> Self {
        AlGlyph {
            bitmap: std::ptr::null(),
            x: Default::default(),
            y: Default::default(),
            w: Default::default(),
            h: Default::default(),
            kerning: Default::default(),
            offset_x: Default::default(),
            offset_y: Default::default(),
            advance: Default::default(),
        }
    }
}

pub const ALLEGRO_NO_KERNING: i32       = -1;
pub const ALLEGRO_ALIGN_LEFT: i32       = 0;
pub const ALLEGRO_ALIGN_CENTRE: i32     = 1;
pub const ALLEGRO_ALIGN_CENTER: i32     = 1;
pub const ALLEGRO_ALIGN_RIGHT: i32      = 2;
pub const ALLEGRO_ALIGN_INTEGER: i32    = 4;

#[link(name="liballegro_monolith.dll")]
extern {
    // General font routines
    pub fn al_init_font_addon() -> bool;
    pub fn al_is_font_addon_initialized() -> bool;
    pub fn al_shutdown_font_addon();
    pub fn al_load_font(filename: *const c_char, size: i32, flags: i32) -> *const AlFont;
    pub fn al_destroy_font(f: *const AlFont);
    pub fn al_register_font_loader(extension: *const c_char, load_font: extern fn(filename: *const c_char, size: i32, flags: i32) -> *const AlFont) -> bool;
    pub fn al_get_font_line_height(f: *const AlFont) -> i32;
    pub fn al_get_font_ascent(f: *const AlFont) -> i32;
    pub fn al_get_font_descent(f: *const AlFont) -> i32;
    pub fn al_get_text_width(f: *const AlFont, str: *const c_char) -> i32;
    pub fn al_get_ustr_width(f: *const AlFont, ustr: *const AlUStr) -> i32;
    pub fn al_draw_text(font: *const AlFont, color: AlColor, x: f32, y: f32, flags: i32, text: *const c_char);
    pub fn al_draw_ustr(font: *const AlFont, color: AlColor, x: f32, y: f32, flags: i32, ustr: *const AlUStr);
    pub fn al_draw_justified_text(font: *const AlFont, color: AlColor, x1: f32, x2: f32, y: f32, diff: f32, flags: i32, text: *const c_char);
    pub fn al_draw_justified_ustr(font: *const AlFont, color: AlColor, x1: f32, x2: f32, y: f32, diff: f32, flags: i32, ustr: *const AlUStr);
    pub fn al_draw_textf(font: *const AlFont, color: AlColor, x: f32, y: f32, flags: i32, format: *const c_char, ...);
    pub fn al_draw_justified_textf(f: *const AlFont, color: AlColor, x1: f32, x2: f32, y: f32, diff: f32, flags: i32, format: *const c_char, ...);
    pub fn al_get_text_dimensions(f: *const AlFont, text: *const c_char, bbx: *mut i32, bby: *mut i32, bbw: *mut i32, bbh: *mut i32);
    pub fn al_get_ustr_dimensions(f: *const AlFont, ustr: *const AlUStr, bbx: *mut i32, bby: *mut i32, bbw: *mut i32, bbh: *mut i32);
    pub fn al_get_allegro_font_version() -> u32;
    pub fn al_get_font_ranges(f: *const AlFont, ranges_count: i32, ranges: *mut i32) -> i32;
    pub fn al_set_fallback_font(font: *const AlFont, fallback: *const AlFont);
    pub fn al_get_fallback_font(font: *const AlFont) -> *const AlFont;

    // Per glyph text handling
    pub fn al_draw_glyph(f: *const AlFont, color: AlColor, x: f32, y: f32, codepoint: i32);
    pub fn al_get_glyph_width(f: *const AlFont, codepoint: i32) -> i32;
    pub fn al_get_glyph_dimensions(f: *const AlFont, codepoint: i32, bbx: *mut i32, bby: *mut i32, bbw: *mut i32, bbh: *mut i32) -> bool;
    pub fn al_get_glyph_advance(f: *const AlFont, codepoint1: i32, codepoint2: i32) -> i32;

    // Multiline text drawing
    pub fn al_draw_multiline_text(font: *const AlFont, color: AlColor, x: f32, y: f32, max_width: f32, line_height: f32, flags: i32, text: *const c_char);
    pub fn al_draw_multiline_ustr(font: *const AlFont, color: AlColor, x: f32, y: f32, max_width: f32, line_height: f32, flags: i32, ustr: *const AlUStr);
    pub fn al_draw_multiline_textf(font: *const AlFont, color: AlColor, x: f32, y: f32, max_width: f32, line_height: f32, flags: i32, format: *const c_char, ...);
    pub fn al_do_multiline_text(font: *const AlFont, max_width: f32, text: *const c_char, cb: extern fn(line_num: i32, line: *const c_char, size: i32, extra: *const c_void) -> bool, extra: *const c_void);
    pub fn al_do_multiline_ustr(font: *const AlFont, max_width: f32, ustr: *const AlUStr, cb: extern fn(line_num: i32, line: *const AlUStr, extra: *const c_void) -> bool, extra: *const c_void);

    // Bitmap fonts
    pub fn al_grab_font_from_bitmap(bmp: *const AlBitmap, ranges_n: i32, ranges: *const i32) -> *const AlFont;
    pub fn al_load_bitmap_font(fname: *const c_char) -> *const AlFont;
    pub fn al_load_bitmap_font_flags(fname: *const c_char, flags: i32) -> *const AlFont;
    pub fn al_create_builtin_font() -> *const AlFont;

    // TTF fonts
    pub fn al_init_ttf_addon() -> bool;
    pub fn al_is_ttf_addon_initialized() -> bool;
    pub fn al_shutdown_ttf_addon();
    pub fn al_load_ttf_font(filename: *const c_char, size: i32, flags: i32) -> *const AlFont;
    pub fn al_load_ttf_font_f(file: *const AlFile, filename: *const c_char, size: i32, flags: i32) -> *const AlFont;
    pub fn al_load_ttf_font_stretch(filename: *const c_char, w: i32, h: i32, flags: i32) -> *const AlFont;
    pub fn al_load_ttf_font_stretch_f(file: *const AlFile, filename: *const c_char, w: i32, h: i32, flags: i32) -> *const AlFont;
    pub fn al_get_allegro_ttf_version() -> u32;
    pub fn al_get_glyph(f: *const AlFont, prev_codepoint: i32, codepoint: i32, glyph: *mut AlGlyph) -> bool;
}
