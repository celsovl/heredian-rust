#[repr(C)]
pub struct AlDisplayMode
{
   width: u32,
   height: u32,
   format: u32,
   refresh_rate: u32
}

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_get_display_mode(index: u32, mode: *mut AlDisplayMode);
    pub fn al_get_num_display_modes() -> u32;
}