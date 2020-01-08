use super::graphics::AlColor;
use std::os::raw::c_char;

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_color_cmyk(c: f32, m: f32, y: f32, k: f32) -> AlColor;
    pub fn al_color_cmyk_to_rgb(cyan: f32, magenta: f32, yellow: f32, key: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_hsl(h: f32, s: f32, l: f32) -> AlColor;
    pub fn al_color_hsl_to_rgb(hue: f32, saturation: f32, lightness: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_hsv(h: f32, s: f32, v: f32) -> AlColor;
    pub fn al_color_hsv_to_rgb(hue: f32, saturation: f32, value: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_html(string: *const c_char) -> AlColor;
    pub fn al_color_html_to_rgb(string: *const c_char, red: *mut f32, green: *mut f32, blue: *mut f32) -> bool;
    pub fn al_color_rgb_to_html(red: f32, green: f32, blue: f32, string: *const c_char);
    pub fn al_color_name(name: *const c_char) -> AlColor;
    pub fn al_color_name_to_rgb(name: *const c_char, r: *mut f32, g: *mut f32, b: *mut f32) -> bool;
    pub fn al_color_rgb_to_cmyk(red: f32, green: f32, blue: f32, cyan: *mut f32, magenta: *mut f32, yellow: *mut f32, key: *mut f32);
    pub fn al_color_rgb_to_hsl(red: f32, green: f32, blue: f32, hue: *mut f32, saturation: *mut f32, lightness: *mut f32);
    pub fn al_color_rgb_to_hsv(red: f32, green: f32, blue: f32, hue: *mut f32, saturation: *mut f32, value: *mut f32);
    pub fn al_color_rgb_to_name(r: f32, g: f32, b: f32) -> *const c_char;
    pub fn al_color_rgb_to_xyz(red: f32, green: f32, blue: f32, x: *mut f32, y: *mut f32, z: *mut f32);
    pub fn al_color_xyz(x: f32, y: f32, z: f32) -> AlColor;
    pub fn al_color_xyz_to_rgb(x: f32, y: f32, z: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_rgb_to_xyy(red: f32, green: f32, blue: f32, x: *mut f32, y: *mut f32, y2: *mut f32);
    pub fn al_color_xyy(x: f32, y: f32, y2: f32) -> AlColor;
    pub fn al_color_xyy_to_rgb(x: f32, y: f32, y2: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_rgb_to_lab(red: f32, green: f32, blue: f32, l: *mut f32, a: *mut f32, b: *mut f32);
    pub fn al_color_lab(l: f32, a: f32, b: f32) -> AlColor;
    pub fn al_color_lab_to_rgb(l: f32, a: f32, b: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_rgb_to_lch(red: f32, green: f32, blue: f32, l: *mut f32, c: *mut f32, h: *mut f32);
    pub fn al_color_lch(l: f32, c: f32, h: f32) -> AlColor;
    pub fn al_color_lch_to_rgb(l: f32, c: f32, h: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_color_rgb_to_yuv(red: f32, green: f32, blue: f32, y: *mut f32, u: *mut f32, v: *mut f32);
    pub fn al_color_yuv(y: f32, u: f32, v: f32) -> AlColor;
    pub fn al_color_yuv_to_rgb(y: f32, u: f32, v: f32, red: *mut f32, green: *mut f32, blue: *mut f32);
    pub fn al_get_allegro_color_version() -> u32;
}