#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_init_primitives_addon() -> bool;
}
