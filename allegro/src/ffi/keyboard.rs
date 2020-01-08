use std::ffi::c_void;
use std::os::raw::c_char;

use super::display::AlDisplay;
use super::events::AlEventSource;

pub const ALLEGRO_KEY_A: i32		= 1;
pub const ALLEGRO_KEY_B: i32		= 2;
pub const ALLEGRO_KEY_C: i32		= 3;
pub const ALLEGRO_KEY_D: i32		= 4;
pub const ALLEGRO_KEY_E: i32		= 5;
pub const ALLEGRO_KEY_F: i32		= 6;
pub const ALLEGRO_KEY_G: i32		= 7;
pub const ALLEGRO_KEY_H: i32		= 8;
pub const ALLEGRO_KEY_I: i32		= 9;
pub const ALLEGRO_KEY_J: i32		= 10;
pub const ALLEGRO_KEY_K: i32		= 11;
pub const ALLEGRO_KEY_L: i32		= 12;
pub const ALLEGRO_KEY_M: i32		= 13;
pub const ALLEGRO_KEY_N: i32		= 14;
pub const ALLEGRO_KEY_O: i32		= 15;
pub const ALLEGRO_KEY_P: i32		= 16;
pub const ALLEGRO_KEY_Q: i32		= 17;
pub const ALLEGRO_KEY_R: i32		= 18;
pub const ALLEGRO_KEY_S: i32		= 19;
pub const ALLEGRO_KEY_T: i32		= 20;
pub const ALLEGRO_KEY_U: i32		= 21;
pub const ALLEGRO_KEY_V: i32		= 22;
pub const ALLEGRO_KEY_W: i32		= 23;
pub const ALLEGRO_KEY_X: i32		= 24;
pub const ALLEGRO_KEY_Y: i32		= 25;
pub const ALLEGRO_KEY_Z: i32		= 26;

pub const ALLEGRO_KEY_0: i32		= 27;
pub const ALLEGRO_KEY_1: i32		= 28;
pub const ALLEGRO_KEY_2: i32		= 29;
pub const ALLEGRO_KEY_3: i32		= 30;
pub const ALLEGRO_KEY_4: i32		= 31;
pub const ALLEGRO_KEY_5: i32		= 32;
pub const ALLEGRO_KEY_6: i32		= 33;
pub const ALLEGRO_KEY_7: i32		= 34;
pub const ALLEGRO_KEY_8: i32		= 35;
pub const ALLEGRO_KEY_9: i32		= 36;

pub const ALLEGRO_KEY_PAD_0: i32		= 37;
pub const ALLEGRO_KEY_PAD_1: i32		= 38;
pub const ALLEGRO_KEY_PAD_2: i32		= 39;
pub const ALLEGRO_KEY_PAD_3: i32		= 40;
pub const ALLEGRO_KEY_PAD_4: i32		= 41;
pub const ALLEGRO_KEY_PAD_5: i32		= 42;
pub const ALLEGRO_KEY_PAD_6: i32		= 43;
pub const ALLEGRO_KEY_PAD_7: i32		= 44;
pub const ALLEGRO_KEY_PAD_8: i32		= 45;
pub const ALLEGRO_KEY_PAD_9: i32		= 46;

pub const ALLEGRO_KEY_F1: i32		= 47;
pub const ALLEGRO_KEY_F2: i32		= 48;
pub const ALLEGRO_KEY_F3: i32		= 49;
pub const ALLEGRO_KEY_F4: i32		= 50;
pub const ALLEGRO_KEY_F5: i32		= 51;
pub const ALLEGRO_KEY_F6: i32		= 52;
pub const ALLEGRO_KEY_F7: i32		= 53;
pub const ALLEGRO_KEY_F8: i32		= 54;
pub const ALLEGRO_KEY_F9: i32		= 55;
pub const ALLEGRO_KEY_F10: i32		= 56;
pub const ALLEGRO_KEY_F11: i32		= 57;
pub const ALLEGRO_KEY_F12: i32		= 58;

pub const ALLEGRO_KEY_ESCAPE: i32	= 59;
pub const ALLEGRO_KEY_TILDE: i32		= 60;
pub const ALLEGRO_KEY_MINUS: i32		= 61;
pub const ALLEGRO_KEY_EQUALS: i32	= 62;
pub const ALLEGRO_KEY_BACKSPACE: i32	= 63;
pub const ALLEGRO_KEY_TAB: i32		= 64;
pub const ALLEGRO_KEY_OPENBRACE: i32	= 65;
pub const ALLEGRO_KEY_CLOSEBRACE: i32	= 66;
pub const ALLEGRO_KEY_ENTER: i32		= 67;
pub const ALLEGRO_KEY_SEMICOLON: i32	= 68;
pub const ALLEGRO_KEY_QUOTE: i32		= 69;
pub const ALLEGRO_KEY_BACKSLASH: i32	= 70;
pub const ALLEGRO_KEY_BACKSLASH2: i32	= 71; /* DirectInput calls this DIK_OEM_102: "< > | on UK/Germany keyboards" */
pub const ALLEGRO_KEY_COMMA: i32		= 72;
pub const ALLEGRO_KEY_FULLSTOP: i32	= 73;
pub const ALLEGRO_KEY_SLASH: i32		= 74;
pub const ALLEGRO_KEY_SPACE: i32		= 75;

pub const ALLEGRO_KEY_INSERT: i32	= 76;
pub const ALLEGRO_KEY_DELETE: i32	= 77;
pub const ALLEGRO_KEY_HOME: i32		= 78;
pub const ALLEGRO_KEY_END: i32		= 79;
pub const ALLEGRO_KEY_PGUP: i32		= 80;
pub const ALLEGRO_KEY_PGDN: i32		= 81;
pub const ALLEGRO_KEY_LEFT: i32		= 82;
pub const ALLEGRO_KEY_RIGHT: i32		= 83;
pub const ALLEGRO_KEY_UP: i32		= 84;
pub const ALLEGRO_KEY_DOWN: i32		= 85;

pub const ALLEGRO_KEY_PAD_SLASH: i32	= 86;
pub const ALLEGRO_KEY_PAD_ASTERISK: i32	= 87;
pub const ALLEGRO_KEY_PAD_MINUS: i32	= 88;
pub const ALLEGRO_KEY_PAD_PLUS: i32	= 89;
pub const ALLEGRO_KEY_PAD_DELETE: i32	= 90;
pub const ALLEGRO_KEY_PAD_ENTER: i32	= 91;

pub const ALLEGRO_KEY_PRINTSCREEN: i32	= 92;
pub const ALLEGRO_KEY_PAUSE: i32		= 93;

pub const ALLEGRO_KEY_ABNT_C1: i32	= 94;
pub const ALLEGRO_KEY_YEN: i32		= 95;
pub const ALLEGRO_KEY_KANA: i32		= 96;
pub const ALLEGRO_KEY_CONVERT: i32	= 97;
pub const ALLEGRO_KEY_NOCONVERT: i32	= 98;
pub const ALLEGRO_KEY_AT: i32		= 99;
pub const ALLEGRO_KEY_CIRCUMFLEX: i32	= 100;
pub const ALLEGRO_KEY_COLON2: i32	= 101;
pub const ALLEGRO_KEY_KANJI: i32		= 102;

pub const ALLEGRO_KEY_PAD_EQUALS: i32	= 103;	/* MacOS X */
pub const ALLEGRO_KEY_BACKQUOTE: i32	= 104;	/* MacOS X */
pub const ALLEGRO_KEY_SEMICOLON2: i32	= 105;	/* MacOS X -- TODO: ask lillo what this should be */
pub const ALLEGRO_KEY_COMMAND: i32	= 106;	/* MacOS X */
   
pub const ALLEGRO_KEY_BACK: i32 = 107;        /* Android back key */
pub const ALLEGRO_KEY_VOLUME_UP: i32 = 108;
pub const ALLEGRO_KEY_VOLUME_DOWN: i32 = 109;

/* Android game keys */
pub const ALLEGRO_KEY_SEARCH: i32       = 110;
pub const ALLEGRO_KEY_DPAD_CENTER: i32  = 111;
pub const ALLEGRO_KEY_BUTTON_X: i32     = 112;
pub const ALLEGRO_KEY_BUTTON_Y: i32     = 113;
pub const ALLEGRO_KEY_DPAD_UP: i32      = 114;
pub const ALLEGRO_KEY_DPAD_DOWN: i32    = 115;
pub const ALLEGRO_KEY_DPAD_LEFT: i32    = 116;
pub const ALLEGRO_KEY_DPAD_RIGHT: i32   = 117;
pub const ALLEGRO_KEY_SELECT: i32       = 118;
pub const ALLEGRO_KEY_START: i32        = 119;
pub const ALLEGRO_KEY_BUTTON_L1: i32    = 120;
pub const ALLEGRO_KEY_BUTTON_R1: i32    = 121;
pub const ALLEGRO_KEY_BUTTON_L2: i32    = 122;
pub const ALLEGRO_KEY_BUTTON_R2: i32    = 123;
pub const ALLEGRO_KEY_BUTTON_A: i32     = 124;
pub const ALLEGRO_KEY_BUTTON_B: i32     = 125;
pub const ALLEGRO_KEY_THUMBL: i32       = 126;
pub const ALLEGRO_KEY_THUMBR: i32       = 127;
   
pub const ALLEGRO_KEY_UNKNOWN: i32      = 128;

/* All codes up to before ALLEGRO_KEY_MODIFIERS can be freely
* assignedas additional unknown keys, like various multimedia
* and application keys keyboards may have.
*/

pub const ALLEGRO_KEY_MODIFIERS: i32	= 215;

pub const ALLEGRO_KEY_LSHIFT: i32	= 215;
pub const ALLEGRO_KEY_RSHIFT: i32	= 216;
pub const ALLEGRO_KEY_LCTRL: i32	= 217;
pub const ALLEGRO_KEY_RCTRL: i32	= 218;
pub const ALLEGRO_KEY_ALT: i32		= 219;
pub const ALLEGRO_KEY_ALTGR: i32	= 220;
pub const ALLEGRO_KEY_LWIN: i32		= 221;
pub const ALLEGRO_KEY_RWIN: i32		= 222;
pub const ALLEGRO_KEY_MENU: i32		= 223;
pub const ALLEGRO_KEY_SCROLLLOCK: i32 = 224;
pub const ALLEGRO_KEY_NUMLOCK: i32	= 225;
pub const ALLEGRO_KEY_CAPSLOCK: i32	= 226;

pub const ALLEGRO_KEY_MAX: usize = 227;

pub const ALLEGRO_KEYMOD_SHIFT: u32       = 0x00001;
pub const ALLEGRO_KEYMOD_CTRL: u32        = 0x00002;
pub const ALLEGRO_KEYMOD_ALT: u32         = 0x00004;
pub const ALLEGRO_KEYMOD_LWIN: u32        = 0x00008;
pub const ALLEGRO_KEYMOD_RWIN: u32        = 0x00010;
pub const ALLEGRO_KEYMOD_MENU: u32        = 0x00020;
pub const ALLEGRO_KEYMOD_ALTGR: u32       = 0x00040;
pub const ALLEGRO_KEYMOD_COMMAND: u32     = 0x00080;
pub const ALLEGRO_KEYMOD_SCROLLLOCK: u32  = 0x00100;
pub const ALLEGRO_KEYMOD_NUMLOCK: u32     = 0x00200;
pub const ALLEGRO_KEYMOD_CAPSLOCK: u32    = 0x00400;
pub const ALLEGRO_KEYMOD_INALTSEQ: u32	 = 0x00800;
pub const ALLEGRO_KEYMOD_ACCENT1: u32     = 0x01000;
pub const ALLEGRO_KEYMOD_ACCENT2: u32     = 0x02000;
pub const ALLEGRO_KEYMOD_ACCENT3: u32     = 0x04000;
pub const ALLEGRO_KEYMOD_ACCENT4: u32     = 0x08000;

const ALLEGRO_KEYDOWN_INTERNAL_SIZE: usize = (ALLEGRO_KEY_MAX + 31) / 32;

pub type AlKeyboard = c_void;

#[repr(C)]
pub struct AlKeyboardState {
    display: *const AlDisplay,  /* public */
    /* internal */
    __key_down_internal_: [u32; ALLEGRO_KEYDOWN_INTERNAL_SIZE],
}

impl Default for AlKeyboardState {
    fn default() -> Self {
        AlKeyboardState {
            display: std::ptr::null(),
            __key_down_internal_: [0; ALLEGRO_KEYDOWN_INTERNAL_SIZE]
        }
    }
}

#[link(name="liballegro_monolith.dll")]
extern {
    pub fn al_install_keyboard() -> bool;
    pub fn al_is_keyboard_installed() -> bool;
    pub fn al_uninstall_keyboard();
    pub fn al_get_keyboard_state(ret_state: *mut AlKeyboardState);
    pub fn al_clear_keyboard_state(display: *const AlDisplay);
    pub fn al_key_down(state: *const AlKeyboardState, keycode: i32) -> bool;
    pub fn al_keycode_to_name(keycode: i32) -> *const c_char;
    pub fn al_set_keyboard_leds(leds: i32) -> bool;
    pub fn al_get_keyboard_event_source() -> *const AlEventSource;
}