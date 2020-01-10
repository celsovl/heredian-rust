#[repr(C)]
#[derive(Default, Debug)]
pub struct AlTransform {
    m: [[f32;4];4]
}

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_copy_transform(dest: *mut AlTransform, src: *const AlTransform);
    pub fn al_use_transform(trans: *const AlTransform);
    pub fn al_get_current_transform() -> *const AlTransform;
    pub fn al_use_projection_transform(trans: *const AlTransform);
    pub fn al_get_current_projection_transform() -> *const AlTransform;
    pub fn al_get_current_inverse_transform() -> *const AlTransform;
    pub fn al_invert_transform(trans: *mut AlTransform);
    pub fn al_check_inverse(trans: *const AlTransform, tol: f32) -> i32;
    pub fn al_identity_transform(trans: *mut AlTransform);
    pub fn al_build_transform(trans: *mut AlTransform, x: f32, y: f32, sx: f32, sy: f32, theta: f32);
    pub fn al_build_camera_transform(trans: *mut AlTransform, position_x: f32, position_y: f32, position_z: f32, look_x: f32, look_y: f32, look_z: f32, up_x: f32, up_y: f32, up_z: f32);
    pub fn al_translate_transform(trans: *mut AlTransform, x: f32, y: f32);
    pub fn al_rotate_transform(trans: *mut AlTransform, theta: f32);
    pub fn al_scale_transform(trans: *mut AlTransform, sx: f32, sy: f32);
    pub fn al_transform_coordinates(trans: *const AlTransform, x: *mut f32, y: *mut f32);
    pub fn al_transform_coordinates_3d(trans: *const AlTransform, x: *mut f32, y: *mut f32, z: *mut f32);
    pub fn al_compose_transform(trans: *mut AlTransform, other: *const AlTransform);
    pub fn al_orthographic_transform(trans: *mut AlTransform, left: f32, top: f32, n: f32, right: f32, bottom: f32, f: f32);
    pub fn al_perspective_transform(trans: *mut AlTransform, left: f32, top: f32, n: f32, right: f32, bottom: f32, f: f32);
    pub fn al_translate_transform_3d(trans: *mut AlTransform, x: f32, y: f32, z: f32);
    pub fn al_scale_transform_3d(trans: *mut AlTransform, sx: f32, sy: f32, sz: f32);
    pub fn al_rotate_transform_3d(trans: *mut AlTransform, x: f32, y: f32, z: f32, angle: f32);
    pub fn al_horizontal_shear_transform(trans: *mut AlTransform, theta: f32);
    pub fn al_vertical_shear_transform(trans: *mut AlTransform, theta: f32);
}
