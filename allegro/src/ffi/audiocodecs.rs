#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_init_acodec_addon() -> bool;
    pub fn al_is_acodec_addon_initialized() -> bool;
    pub fn al_get_allegro_acodec_version() -> u32;
}