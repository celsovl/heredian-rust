#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_init_image_addon() -> bool;
    pub fn al_is_image_addon_initialized() -> bool;
    pub fn al_shutdown_image_addon();
    pub fn al_get_allegro_image_version() -> u32;
}
