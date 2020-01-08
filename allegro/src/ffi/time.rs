#[repr(C)]
#[derive(Default)]
pub struct AlTimeout {
    pad1: u64,
    pad2: u64
}

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_get_time() -> f64;
    pub fn al_init_timeout(timeout: *mut AlTimeout, seconds: f64);
    pub fn al_rest(seconds: f64);
}
