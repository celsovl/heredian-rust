use std::os::raw::c_char;
use std::ffi::c_void;

use super::file::AlFile;
use super::display::AlDisplay;

pub type AlBitmap = c_void;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct AlColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

#[repr(C)]
pub struct AlLockedRegion {
    data: *const c_void,
    format: i32,
    pitch: i32,
    pixel_size: i32,
 }

impl Default for AlLockedRegion {
    fn default() -> Self {
        AlLockedRegion {
            data: std::ptr::null(),
            format: Default::default(),
            pitch: Default::default(),
            pixel_size: Default::default(),
        }
    }
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AlPixelFormat {
    ALLEGRO_PIXEL_FORMAT_ANY                   = 0,
    ALLEGRO_PIXEL_FORMAT_ANY_NO_ALPHA          = 1,
    ALLEGRO_PIXEL_FORMAT_ANY_WITH_ALPHA        = 2,
    ALLEGRO_PIXEL_FORMAT_ANY_15_NO_ALPHA       = 3,
    ALLEGRO_PIXEL_FORMAT_ANY_16_NO_ALPHA       = 4,
    ALLEGRO_PIXEL_FORMAT_ANY_16_WITH_ALPHA     = 5,
    ALLEGRO_PIXEL_FORMAT_ANY_24_NO_ALPHA       = 6,
    ALLEGRO_PIXEL_FORMAT_ANY_32_NO_ALPHA       = 7,
    ALLEGRO_PIXEL_FORMAT_ANY_32_WITH_ALPHA     = 8,
    ALLEGRO_PIXEL_FORMAT_ARGB_8888             = 9,
    ALLEGRO_PIXEL_FORMAT_RGBA_8888             = 10,
    ALLEGRO_PIXEL_FORMAT_ARGB_4444             = 11,
    ALLEGRO_PIXEL_FORMAT_RGB_888               = 12, /* 24 bit format */
    ALLEGRO_PIXEL_FORMAT_RGB_565               = 13,
    ALLEGRO_PIXEL_FORMAT_RGB_555               = 14,
    ALLEGRO_PIXEL_FORMAT_RGBA_5551             = 15,
    ALLEGRO_PIXEL_FORMAT_ARGB_1555             = 16,
    ALLEGRO_PIXEL_FORMAT_ABGR_8888             = 17,
    ALLEGRO_PIXEL_FORMAT_XBGR_8888             = 18,
    ALLEGRO_PIXEL_FORMAT_BGR_888               = 19, /* 24 bit format */
    ALLEGRO_PIXEL_FORMAT_BGR_565               = 20,
    ALLEGRO_PIXEL_FORMAT_BGR_555               = 21,
    ALLEGRO_PIXEL_FORMAT_RGBX_8888             = 22,
    ALLEGRO_PIXEL_FORMAT_XRGB_8888             = 23,
    ALLEGRO_PIXEL_FORMAT_ABGR_F32              = 24,
    ALLEGRO_PIXEL_FORMAT_ABGR_8888_LE          = 25,
    ALLEGRO_PIXEL_FORMAT_RGBA_4444             = 26,
    ALLEGRO_PIXEL_FORMAT_SINGLE_CHANNEL_8      = 27,
    ALLEGRO_PIXEL_FORMAT_COMPRESSED_RGBA_DXT1  = 28,
    ALLEGRO_PIXEL_FORMAT_COMPRESSED_RGBA_DXT3  = 29,
    ALLEGRO_PIXEL_FORMAT_COMPRESSED_RGBA_DXT5  = 30,
    ALLEGRO_NUM_PIXEL_FORMATS 
}

pub const ALLEGRO_LOCK_READWRITE: i32  = 0;
pub const ALLEGRO_LOCK_READONLY: i32   = 1;
pub const ALLEGRO_LOCK_WRITEONLY: i32  = 2;

pub const ALLEGRO_MEMORY_BITMAP: i32            = 0x0001;
pub const _ALLEGRO_KEEP_BITMAP_FORMAT: i32      = 0x0002;	/* now a bitmap loader flag */
pub const ALLEGRO_FORCE_LOCKING: i32            = 0x0004;	/* no longer honoured */
pub const ALLEGRO_NO_PRESERVE_TEXTURE: i32      = 0x0008;
pub const _ALLEGRO_ALPHA_TEST: i32              = 0x0010;   /* now a render state flag */
pub const _ALLEGRO_INTERNAL_OPENGL: i32         = 0x0020;
pub const ALLEGRO_MIN_LINEAR: i32               = 0x0040;
pub const ALLEGRO_MAG_LINEAR: i32               = 0x0080;
pub const ALLEGRO_MIPMAP: i32                   = 0x0100;
pub const _ALLEGRO_NO_PREMULTIPLIED_ALPHA: i32  = 0x0200;	/* now a bitmap loader flag */
pub const ALLEGRO_VIDEO_BITMAP: i32             = 0x0400;
pub const ALLEGRO_CONVERT_BITMAP: i32           = 0x1000;

pub const ALLEGRO_FLIP_HORIZONTAL: i32 = 0x00001;
pub const ALLEGRO_FLIP_VERTICAL: i32   = 0x00002;

pub const ALLEGRO_KEEP_BITMAP_FORMAT: i32       = 0x0002;   /* was a bitmap flag in 5.0 */
pub const ALLEGRO_NO_PREMULTIPLIED_ALPHA: i32   = 0x0200;   /* was a bitmap flag in 5.0 */
pub const ALLEGRO_KEEP_INDEX: i32               = 0x0800;

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AlBlendOperations {
    ALLEGRO_ADD                = 0,
    ALLEGRO_SRC_MINUS_DEST     = 1,
    ALLEGRO_DEST_MINUS_SRC     = 2,
    ALLEGRO_NUM_BLEND_OPERATIONS
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AlBlendMode {
    ALLEGRO_ZERO                = 0,
    ALLEGRO_ONE                 = 1,
    ALLEGRO_ALPHA               = 2,
    ALLEGRO_INVERSE_ALPHA       = 3,
    ALLEGRO_SRC_COLOR           = 4,
    ALLEGRO_DEST_COLOR          = 5,
    ALLEGRO_INVERSE_SRC_COLOR   = 6,
    ALLEGRO_INVERSE_DEST_COLOR  = 7,
    ALLEGRO_CONST_COLOR         = 8,
    ALLEGRO_INVERSE_CONST_COLOR = 9,
    ALLEGRO_NUM_BLEND_MODES
}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AlRenderState {
    /* ALLEGRO_ALPHA_TEST was the name of a rare bitmap flag only used on the
    * Wiz port.  Reuse the name but retain the same value.
    */
    ALLEGRO_ALPHA_TEST = 0x0010,
    ALLEGRO_WRITE_MASK,
    ALLEGRO_DEPTH_TEST,
    ALLEGRO_DEPTH_FUNCTION,
    ALLEGRO_ALPHA_FUNCTION,
    ALLEGRO_ALPHA_TEST_VALUE
}

pub type AlRenderFunction = u32;
pub const ALLEGRO_RENDER_NEVER: AlRenderFunction = 0;
pub const ALLEGRO_RENDER_ALWAYS: AlRenderFunction = 1;
pub const ALLEGRO_RENDER_LESS: AlRenderFunction = 2;
pub const ALLEGRO_RENDER_EQUAL: AlRenderFunction = 3; 
pub const ALLEGRO_RENDER_LESS_EQUAL: AlRenderFunction = 4;
pub const ALLEGRO_RENDER_GREATER: AlRenderFunction = 5;
pub const ALLEGRO_RENDER_NOT_EQUAL: AlRenderFunction = 6;
pub const ALLEGRO_RENDER_GREATER_EQUAL: AlRenderFunction = 7;

pub type AlWriteMaskFlags = u32;
pub const ALLEGRO_MASK_RED: AlWriteMaskFlags = 1 << 0;
pub const ALLEGRO_MASK_GREEN: AlWriteMaskFlags = 1 << 1;
pub const ALLEGRO_MASK_BLUE: AlWriteMaskFlags = 1 << 2;
pub const ALLEGRO_MASK_ALPHA: AlWriteMaskFlags = 1 << 3;
pub const ALLEGRO_MASK_DEPTH: AlWriteMaskFlags = 1 << 4;
pub const ALLEGRO_MASK_RGB: AlWriteMaskFlags = (ALLEGRO_MASK_RED | ALLEGRO_MASK_GREEN | ALLEGRO_MASK_BLUE);
pub const ALLEGRO_MASK_RGBA: AlWriteMaskFlags = (ALLEGRO_MASK_RGB | ALLEGRO_MASK_ALPHA);
 
#[link(name="liballegro_monolith.dll")]
extern {
    // Colors
    pub fn al_map_rgb(r: u8, g: u8, b: u8) -> AlColor;
    pub fn al_map_rgb_f(r: f32, g: f32, b: f32) -> AlColor;
    pub fn al_map_rgba(r: u8, g: u8, b: u8, a: u8) -> AlColor;
    pub fn al_premul_rgba(r: u8, g: u8, b: u8, a: u8) -> AlColor;
    pub fn al_map_rgba_f(r: f32, g: f32, b: f32, a: f32) -> AlColor;
    pub fn al_premul_rgba_f(r: f32, g: f32, b: f32, a: f32) -> AlColor;
    pub fn al_unmap_rgb(color: AlColor, r: *mut u8, g: *mut u8, b: *mut u8);
    pub fn al_unmap_rgb_f(color: AlColor, r: *mut f32, g: *mut f32, b: *mut f32);
    pub fn al_unmap_rgba(color: AlColor, r: *mut u8, g: *mut u8, b: *mut u8, a: *mut u8);
    pub fn al_unmap_rgba_f(color: AlColor, r: *mut f32, g: *mut f32, b: *mut f32, a: *mut f32);

    // Locking and pixel formats
    pub fn al_get_pixel_size(format: i32) -> i32;
    pub fn al_get_pixel_format_bits(format: i32) -> i32;
    pub fn al_get_pixel_block_size(format: i32) -> i32;
    pub fn al_get_pixel_block_width(format: i32) -> i32;
    pub fn al_get_pixel_block_height(format: i32) -> i32;
    pub fn al_lock_bitmap(bitmap: *const AlBitmap, format: i32, flags: i32) -> *const AlLockedRegion;
    pub fn al_lock_bitmap_region(bitmap: *const AlBitmap, x: i32, y: i32, width: i32, height: i32, format: i32, flags: i32) -> *const AlLockedRegion;
    pub fn al_unlock_bitmap(bitmap: *const AlBitmap);
    pub fn al_lock_bitmap_blocked(bitmap: *const AlBitmap, flags: i32) -> *const AlLockedRegion;
    pub fn al_lock_bitmap_region_blocked(bitmap: *const AlBitmap, x_block: i32, y_block: i32, width_block: i32, height_block: i32, flags: i32) -> *const AlLockedRegion;

    // Bitmap creation
    pub fn al_create_bitmap(w: i32, h: i32) -> *const AlBitmap;
    pub fn al_create_sub_bitmap(parent: *const AlBitmap, x: i32, y: i32, w: i32, h: i32) -> *const AlBitmap;
    pub fn al_clone_bitmap(bitmap: *const AlBitmap) -> *const AlBitmap;
    pub fn al_convert_bitmap(bitmap: *const AlBitmap);
    pub fn al_convert_memory_bitmaps();
    pub fn al_destroy_bitmap(bitmap: *const AlBitmap);
    pub fn al_get_new_bitmap_flags() -> i32;
    pub fn al_get_new_bitmap_format() -> i32;
    pub fn al_set_new_bitmap_flags(flags: i32);
    pub fn al_add_new_bitmap_flag(flag: i32);
    pub fn al_set_new_bitmap_format(format: i32);
    pub fn al_set_new_bitmap_depth(depth: i32);
    pub fn al_get_new_bitmap_depth() -> i32;
    pub fn al_set_new_bitmap_samples(samples: i32);
    pub fn al_get_new_bitmap_samples() -> i32;

    // Bitmap properties
    pub fn al_get_bitmap_flags(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_format(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_height(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_width(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_depth(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_samples(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_pixel(bitmap: *const AlBitmap, x: i32, y: i32) -> AlColor;
    pub fn al_is_bitmap_locked(bitmap: *const AlBitmap) -> bool;
    pub fn al_is_compatible_bitmap(bitmap: *const AlBitmap) -> bool;
    pub fn al_is_sub_bitmap(bitmap: *const AlBitmap) -> bool;
    pub fn al_get_parent_bitmap(bitmap: *const AlBitmap) -> *const AlBitmap;
    pub fn al_get_bitmap_x(bitmap: *const AlBitmap) -> i32;
    pub fn al_get_bitmap_y(bitmap: *const AlBitmap) -> i32;
    pub fn al_reparent_bitmap(bitmap: *const AlBitmap, parent: *const AlBitmap, x: i32, y: i32, w: i32, h: i32);
    pub fn al_get_bitmap_blender(op: *mut i32, src: *mut i32, dst: *mut i32);
    pub fn al_get_separate_bitmap_blender(op: *mut i32, src: *mut i32, dst: *mut i32, alpha_op: *mut i32, alpha_src: *mut i32, alpha_dst: *mut i32);
    pub fn al_get_bitmap_blend_color() -> AlColor;
    pub fn al_set_bitmap_blender(op: i32, src: i32, dest: i32);
    pub fn al_set_separate_bitmap_blender(op: i32, src: i32, dst: i32, alpha_op: i32, alpha_src: i32, alpha_dst: i32);
    pub fn al_set_bitmap_blend_color(col: AlColor);
    pub fn al_reset_bitmap_blender();

    // Drawing operations
    pub fn al_clear_to_color(color: AlColor);
    pub fn al_clear_depth_buffer(z: f32);
    pub fn al_draw_bitmap(bitmap: *const AlBitmap, dx: f32, dy: f32, flags: i32);
    pub fn al_draw_tinted_bitmap(bitmap: *const AlBitmap, tint: AlColor, dx: f32, dy: f32, flags: i32);
    pub fn al_draw_bitmap_region(bitmap: *const AlBitmap, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: i32);
    pub fn al_draw_tinted_bitmap_region(bitmap: *const AlBitmap, tint: AlColor, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: i32);
    pub fn al_draw_pixel(x: f32, y: f32, color: AlColor);
    pub fn al_draw_rotated_bitmap(bitmap: *const AlBitmap, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: i32);
    pub fn al_draw_tinted_rotated_bitmap(bitmap: *const AlBitmap, tint: AlColor, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: i32);
    pub fn al_draw_scaled_rotated_bitmap(bitmap: *const AlBitmap, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: i32);
    pub fn al_draw_tinted_scaled_rotated_bitmap(bitmap: *const AlBitmap, tint: AlColor, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: i32);
    pub fn al_draw_tinted_scaled_rotated_bitmap_region(bitmap: *const AlBitmap, sx: f32, sy: f32, sw: f32, sh: f32, tint: AlColor, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: i32);
    pub fn al_draw_scaled_bitmap(bitmap: *const AlBitmap, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: i32);
    pub fn al_draw_tinted_scaled_bitmap(bitmap: *const AlBitmap, tint: AlColor, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: i32);
    pub fn al_get_target_bitmap() -> *const AlBitmap;
    pub fn al_put_pixel(x: i32, y: i32, color: AlColor);
    pub fn al_put_blended_pixel(x: i32, y: i32, color: AlColor);

    // Target bitmap
    pub fn al_set_target_bitmap(bitmap: *const AlBitmap);
    pub fn al_set_target_backbuffer(display: *const AlDisplay);
    pub fn al_get_current_display() -> *const AlDisplay;

    // Blending modes
    pub fn al_get_blender(op: *mut AlBlendOperations, src: *mut AlBlendMode, dst: *mut AlBlendMode);
    pub fn al_get_separate_blender(op: *mut AlBlendOperations, src: *mut AlBlendMode, dst: *mut AlBlendMode, alpha_op: *mut AlBlendOperations, alpha_src: *mut AlBlendMode, alpha_dst: *mut AlBlendMode);
    pub fn al_get_blend_color() -> AlColor;
    pub fn al_set_blender(op: AlBlendOperations, src: AlBlendMode, dst: AlBlendMode);
    pub fn al_set_separate_blender(op: AlBlendOperations, src: AlBlendMode, dst: AlBlendMode, alpha_op: AlBlendOperations, alpha_src: AlBlendMode, alpha_dst: AlBlendMode);
    pub fn al_set_blend_color(color: AlColor);

    // Clipping
    pub fn al_get_clipping_rectangle(x: *mut i32, y: *mut i32, w: *mut i32, h: *mut i32);
    pub fn al_set_clipping_rectangle(x: i32, y: i32, width: i32, height: i32);
    pub fn al_reset_clipping_rectangle();

    // Graphics utility functions
    pub fn al_convert_mask_to_alpha(bitmap: *const AlBitmap, mask_color: AlColor);

    // Deferred drawing
    pub fn al_hold_bitmap_drawing(hold: bool);
    pub fn al_is_bitmap_drawing_held() -> bool;

    // Image I/O
    pub fn al_register_bitmap_loader(extension: *const c_char, loader: extern fn(filename: *const c_char, flags: i32) -> *const AlBitmap) -> bool;
    pub fn al_register_bitmap_saver(extension: *const c_char, saver: extern fn(filename: *const c_char, bmp: *const AlBitmap) -> bool) -> bool;
    pub fn al_register_bitmap_loader_f(extension: *const c_char, fs_loader: extern fn(fp: *const AlFile, flags: i32) -> *const AlBitmap) -> bool;
    pub fn al_register_bitmap_saver_f(extension: *const c_char, fs_saver: extern fn(fp: *const AlFile, bmp: *const AlBitmap) -> bool) -> bool;
    pub fn al_load_bitmap(filename: *const c_char) -> *const AlBitmap;
    pub fn al_load_bitmap_flags(filename: *const c_char, flags: i32) -> *const AlBitmap;
    pub fn al_load_bitmap_f(fp: *const AlFile, ident: *const c_char) -> *const AlBitmap;
    pub fn al_load_bitmap_flags_f(fp: *const AlFile, ident: *const c_char, flags: i32) -> *const AlBitmap;
    pub fn al_save_bitmap(filename: *const c_char, bitmap: *const AlBitmap) -> bool;
    pub fn al_save_bitmap_f(fp: *const AlFile, ident: *const c_char, bitmap: *const AlBitmap) -> bool;
    pub fn al_register_bitmap_identifier(extension: *const c_char, identifier: extern fn(f: *const AlFile) -> bool) -> bool;
    pub fn al_identify_bitmap(filename: *const c_char) -> *const c_char;
    pub fn al_identify_bitmap_f(fp: *const AlFile) -> *const c_char;

    // Render State
    pub fn al_set_render_state(state: AlRenderState, value: i32);
    pub fn al_backup_dirty_bitmap(bitmap: *const AlBitmap);
    pub fn al_backup_dirty_bitmaps(display: *const AlDisplay);
}