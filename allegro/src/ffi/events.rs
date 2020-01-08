use super::*;
use super::display::AlDisplay;
use super::keyboard::AlKeyboard;
use super::timer::AlTimer;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AlEventType {
    ALLEGRO_EVENT_JOYSTICK_AXIS               =  1,
    ALLEGRO_EVENT_JOYSTICK_BUTTON_DOWN        =  2,
    ALLEGRO_EVENT_JOYSTICK_BUTTON_UP          =  3,
    ALLEGRO_EVENT_JOYSTICK_CONFIGURATION      =  4,
 
    ALLEGRO_EVENT_KEY_DOWN                    = 10,
    ALLEGRO_EVENT_KEY_CHAR                    = 11,
    ALLEGRO_EVENT_KEY_UP                      = 12,
 
    ALLEGRO_EVENT_MOUSE_AXES                  = 20,
    ALLEGRO_EVENT_MOUSE_BUTTON_DOWN           = 21,
    ALLEGRO_EVENT_MOUSE_BUTTON_UP             = 22,
    ALLEGRO_EVENT_MOUSE_ENTER_DISPLAY         = 23,
    ALLEGRO_EVENT_MOUSE_LEAVE_DISPLAY         = 24,
    ALLEGRO_EVENT_MOUSE_WARPED                = 25,
 
    ALLEGRO_EVENT_TIMER                       = 30,
 
    ALLEGRO_EVENT_DISPLAY_EXPOSE              = 40,
    ALLEGRO_EVENT_DISPLAY_RESIZE              = 41,
    ALLEGRO_EVENT_DISPLAY_CLOSE               = 42,
    ALLEGRO_EVENT_DISPLAY_LOST                = 43,
    ALLEGRO_EVENT_DISPLAY_FOUND               = 44,
    ALLEGRO_EVENT_DISPLAY_SWITCH_IN           = 45,
    ALLEGRO_EVENT_DISPLAY_SWITCH_OUT          = 46,
    ALLEGRO_EVENT_DISPLAY_ORIENTATION         = 47,
    ALLEGRO_EVENT_DISPLAY_HALT_DRAWING        = 48,
    ALLEGRO_EVENT_DISPLAY_RESUME_DRAWING      = 49,
 
    ALLEGRO_EVENT_TOUCH_BEGIN                 = 50,
    ALLEGRO_EVENT_TOUCH_END                   = 51,
    ALLEGRO_EVENT_TOUCH_MOVE                  = 52,
    ALLEGRO_EVENT_TOUCH_CANCEL                = 53,
    
    ALLEGRO_EVENT_DISPLAY_CONNECTED           = 60,
    ALLEGRO_EVENT_DISPLAY_DISCONNECTED        = 61
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlAnyEvent {
    pub r#type: AlEventType,
    pub source: *const AlEventSource,
    pub timestamp: f64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlDisplayEvent {
    pub r#type: AlEventType,
    pub source: *const AlDisplay,
    pub timestamp: f64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub orientation: i32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlJoystickEvent {
    r#type: AlEventType,
    source: *const AlJoystick,
    timestamp: f64,
    id: *const AlJoystick,
    stick: i32,
    axis: i32,
    pos: f32,
    button: i32
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlKeyboardEvent {
    pub r#type: AlEventType,
    pub source: *const AlKeyboard,
    pub timestamp: f64,
    pub display: *const AlDisplay,
    pub keycode: i32,
    pub unichar: i32,
    pub modifiers: u32,
    pub repeat: bool
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlMouseEvent {
    pub r#type: AlEventType,
    pub source: *const AlMouse,
    pub timestamp: f64,
    pub display: *const AlDisplay,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
    pub dw: i32,
    pub button: u32,
    pub pressure: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlTimerEvent {
    pub r#type: AlEventType,
    pub source: *const AlTimer,
    pub timestamp: f64,
    pub count: i64,
    pub error: f64
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlTouchEvent {
    pub r#type: AlEventType,
    pub source: *const AlTouchInput,
    pub timestamp: f64,
    pub display: *const AlDisplay,
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub primary: bool
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlUserEvent {
    pub r#type: AlEventType,
    pub source: *const AlEventSource,
    pub timestamp: f64,
    pub __internal_descr: *const AlUserEventDescriptor,
    pub data1: *const c_void,
    pub data2: *const c_void,
    pub data3: *const c_void,
    pub data4: *const c_void,
}


#[repr(C)]
pub union AlEvent {
    pub r#type: AlEventType,
    pub any: AlAnyEvent,
    pub display: AlDisplayEvent,
    pub joystick: AlJoystickEvent,
    pub keyboard: AlKeyboardEvent,
    pub mouse: AlMouseEvent,
    pub timer: AlTimerEvent,
    pub touch: AlTouchEvent,
    pub user: AlUserEvent,
}

impl AlEvent {
    pub fn get_type(&self) -> AlEventType {
        unsafe { self.r#type }
    }

    pub fn get_timer(&self) -> &AlTimerEvent {
        unsafe { &self.timer }
    }

    pub fn get_keyboard(&self) -> &AlKeyboardEvent {
        unsafe { &self.keyboard }
    }
}

impl Default for AlEvent {
    fn default() -> Self {
        AlEvent { r#type: AlEventType::ALLEGRO_EVENT_MOUSE_WARPED }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct AlEventSource {
    __pad: [i32; 32]
}


pub type AlEventQueue = c_void;
pub type AlUserEventDescriptor = c_void;

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_create_event_queue() -> *const AlEventQueue;
    pub fn al_destroy_event_queue(queue: *const AlEventQueue);
    pub fn al_register_event_source(queue: *const AlEventQueue, source: *const AlEventSource);
    pub fn al_unregister_event_source(queue: *const AlEventQueue, source: *const AlEventSource);
    pub fn al_is_event_source_registered(queue: *const AlEventQueue, source: *const AlEventSource) -> bool;
    pub fn al_pause_event_queue(queue: *const AlEventQueue, pause: bool);
    pub fn al_is_event_queue_paused(queue: *const AlEventQueue) -> bool;
    pub fn al_is_event_queue_empty(queue: *const AlEventQueue) -> bool;
    pub fn al_get_next_event(queue: *const AlEventQueue, ret_event: *mut AlEvent) -> bool;
    pub fn al_peek_next_event(queue: *const AlEventQueue, ret_event: *mut AlEvent) -> bool;
    pub fn al_drop_next_event(queue: *const AlEventQueue) -> bool;
    pub fn al_flush_event_queue(queue: *const AlEventQueue) -> bool;
    pub fn al_wait_for_event(queue: *const AlEventQueue, ret_event: *mut AlEvent);
    pub fn al_wait_for_event_timed(queue: *const AlEventQueue, ret_event: *mut AlEvent, secs: f32) -> bool;
    pub fn al_wait_for_event_until(queue: *const AlEventQueue, ret_event: *mut AlEvent, timeout: *const AlTimeout) -> bool;
    pub fn al_init_user_event_source(src: *mut AlEventSource);
    pub fn al_destroy_user_event_source(src: *mut AlEventSource);
    pub fn al_emit_user_event(src: *const AlEventSource, event: *const AlEvent, dtor: extern fn(*const AlUserEvent)) -> bool;
    pub fn al_unref_user_event(event: AlUserEvent);
    pub fn al_get_event_source_data(source: *const AlEventSource) -> libc::intptr_t;
    pub fn al_set_event_source_data(source: *const AlEventSource, data: libc::intptr_t);
}

#[allow(non_snake_case)]
pub fn ALLEGRO_EVENT_TYPE_IS_USER(t: u32) -> bool {
    return t >= 512;
}
