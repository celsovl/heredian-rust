use std::ffi::c_void;

use super::events::AlEventSource;

pub type AlTimer = c_void;

pub fn ALLEGRO_USECS_TO_SECS(x: f64) -> f64 { x / 1000000.0 }
pub fn ALLEGRO_MSECS_TO_SECS(x: f64) -> f64 { x / 1000.0 }
pub fn ALLEGRO_BPS_TO_SECS(x: f64) -> f64 { 1.0 / x }
pub fn ALLEGRO_BPM_TO_SECS(x: f64) -> f64 { 60.0 / x }

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_create_timer(speed_secs: f64) -> *const AlTimer;
    pub fn al_start_timer(timer: *const AlTimer);
    pub fn al_resume_timer(timer: *const AlTimer);
    pub fn al_stop_timer(timer: *const AlTimer);
    pub fn al_get_timer_started(timer: *const AlTimer) -> bool;
    pub fn al_destroy_timer(timer: *const AlTimer);
    pub fn al_get_timer_count(timer: *const AlTimer) -> i64;
    pub fn al_set_timer_count(timer: *const AlTimer, new_count: i64);
    pub fn al_add_timer_count(timer: *const AlTimer, diff: i64);
    pub fn al_get_timer_speed(timer: *const AlTimer) -> f64;
    pub fn al_set_timer_speed(timer: *const AlTimer, new_speed_secs: f64);
    pub fn al_get_timer_event_source(timer: *const AlTimer) -> *const AlEventSource;
}
