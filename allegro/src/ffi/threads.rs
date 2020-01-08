use std::ffi::c_void;

use super::time::AlTimeout;

pub type AlThread = c_void;
pub type AlMutex = c_void;
pub type AlCond = c_void;

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_create_thread(proc: extern fn(thread: *const AlThread, arg: *const c_void) -> *const c_void, arg: *const c_void) -> *const AlThread;
    pub fn al_create_thread_with_stacksize(proc: extern fn(thread: *const AlThread, arg: *const c_void) -> *const c_void, arg: *const c_void, stacksize: usize) -> *const AlThread;
    pub fn al_start_thread(thread: *const AlThread);
    pub fn al_join_thread(thread: *const AlThread, ret_value: *const *const c_void);
    pub fn al_set_thread_should_stop(thread: *const AlThread);
    pub fn al_get_thread_should_stop(thread: *const AlThread) -> bool;
    pub fn al_destroy_thread(thread: *const AlThread);
    pub fn al_run_detached_thread(proc: extern fn(arg: *const c_void) -> *const c_void, arg: *const c_void);
    pub fn al_create_mutex() -> *const AlMutex;
    pub fn al_create_mutex_recursive() -> *const AlMutex;
    pub fn al_lock_mutex(mutex: *const AlMutex);
    pub fn al_unlock_mutex(mutex: *const AlMutex);
    pub fn al_destroy_mutex(mutex: *const AlMutex);
    pub fn al_create_cond() -> *const AlCond;
    pub fn al_destroy_cond(cond: *const AlCond);
    pub fn al_wait_cond(cond: *const AlCond, mutex: *const AlMutex);
    pub fn al_wait_cond_until(cond: *const AlCond, mutex: *const AlMutex, timeout: *const AlTimeout) -> i32;
    pub fn al_broadcast_cond(cond: *const AlCond);
    pub fn al_signal_cond(cond: *const AlCond);

}
