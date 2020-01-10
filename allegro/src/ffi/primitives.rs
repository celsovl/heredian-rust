use super::graphics::{AlColor};

#[link(name="liballegro_monolith.dll")]
extern {
    // General
    pub fn al_get_allegro_primitives_version() -> u32;
    pub fn al_init_primitives_addon() -> bool;
    pub fn al_shutdown_primitives_addon();
    
    // High level drawing routines
    pub fn al_draw_line(x1: f32, y1: f32, x2: f32, y2: f32, color: AlColor, thickness: f32);
    pub fn al_draw_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: AlColor);
    pub fn al_draw_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, color: AlColor);
    pub fn al_draw_rounded_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_rounded_rectangle(x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: AlColor);
    pub fn al_calculate_arc(dest: *mut f32, stride: i32, cx: f32, cy: f32, rx: f32, ry: f32, start_theta: f32, delta_theta: f32, thickness: f32, num_points: i32);
    pub fn al_draw_pieslice(cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_pieslice(cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: AlColor);
    pub fn al_draw_ellipse(cx: f32, cy: f32, rx: f32, ry: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_ellipse(cx: f32, cy: f32, rx: f32, ry: f32, color: AlColor);
    pub fn al_draw_circle(cx: f32, cy: f32, r: f32, color: AlColor, thickness: f32);
    pub fn al_draw_filled_circle(cx: f32, cy: f32, r: f32, color: AlColor);
    pub fn al_draw_arc(cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: AlColor, thickness: f32);
    pub fn al_draw_elliptical_arc(cx: f32, cy: f32, rx: f32, ry: f32, start_theta: f32, delta_theta: f32, color: AlColor, thickness: f32);
    pub fn al_calculate_spline(dest: *mut f32, stride: i32, points: [f32; 8], thickness: f32, num_segments: i32);
    pub fn al_draw_spline(points: [f32; 8], color: AlColor, thickness: f32);
    pub fn al_calculate_ribbon(dest: *mut f32, dest_stride: i32, points: *const f32, points_stride: i32, thickness: f32, num_segments: i32);
    pub fn al_draw_ribbon(points: *const f32, points_stride: i32, color: AlColor, thickness: f32, num_segments: i32);

}
