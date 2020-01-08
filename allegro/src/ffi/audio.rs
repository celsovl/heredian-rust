use std::os::raw::c_char;
use std::ffi::c_void;

use libc::size_t;

use super::events::AlEvent;
use super::file::AlFile;
use super::events::AlEventSource;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AlAudioEventType {
    _KCM_STREAM_FEEDER_QUIT_EVENT_TYPE    = 512,
    ALLEGRO_EVENT_AUDIO_STREAM_FRAGMENT   = 513,
    ALLEGRO_EVENT_AUDIO_STREAM_FINISHED   = 514,
    ALLEGRO_EVENT_AUDIO_RECORDER_FRAGMENT = 515, 
}

pub type AlAudioDepth = u8;

/* Sample depth and type, and signedness. Mixers only use 32-bit signed
* float (-1..+1). The unsigned value is a bit-flag applied to the depth
* value.
*/
pub const ALLEGRO_AUDIO_DEPTH_INT8: AlAudioDepth      = 0x00;
pub const ALLEGRO_AUDIO_DEPTH_INT16: AlAudioDepth     = 0x01;
pub const ALLEGRO_AUDIO_DEPTH_INT24: AlAudioDepth     = 0x02;
pub const ALLEGRO_AUDIO_DEPTH_FLOAT32: AlAudioDepth   = 0x03;

pub const ALLEGRO_AUDIO_DEPTH_UNSIGNED: AlAudioDepth  = 0x08;

/* For convenience */
pub const ALLEGRO_AUDIO_DEPTH_UINT8: AlAudioDepth  = ALLEGRO_AUDIO_DEPTH_INT8 | ALLEGRO_AUDIO_DEPTH_UNSIGNED;
pub const ALLEGRO_AUDIO_DEPTH_UINT16: AlAudioDepth = ALLEGRO_AUDIO_DEPTH_INT16 | ALLEGRO_AUDIO_DEPTH_UNSIGNED;
pub const ALLEGRO_AUDIO_DEPTH_UINT24: AlAudioDepth = ALLEGRO_AUDIO_DEPTH_INT24 | ALLEGRO_AUDIO_DEPTH_UNSIGNED;

#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum AlChannelConf {
    /* Speaker configuration (mono, stereo, 2.1, 3, etc). With regards to
    * behavior, most of this code makes no distinction between, say, 4.1 and
    * 5 speaker setups.. they both have 5 "channels". However, users would
    * like the distinction, and later when the higher-level stuff is added,
    * the differences will become more important. (v>>4)+(v&0xF) should yield
    * the total channel count.
    */
    ALLEGRO_CHANNEL_CONF_1   = 0x10,
    ALLEGRO_CHANNEL_CONF_2   = 0x20,
    ALLEGRO_CHANNEL_CONF_3   = 0x30,
    ALLEGRO_CHANNEL_CONF_4   = 0x40,
    ALLEGRO_CHANNEL_CONF_5_1 = 0x51,
    ALLEGRO_CHANNEL_CONF_6_1 = 0x61,
    ALLEGRO_CHANNEL_CONF_7_1 = 0x71      
}

#[derive(Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum AlMixerQuality {
    ALLEGRO_MIXER_QUALITY_POINT   = 0x110,
    ALLEGRO_MIXER_QUALITY_LINEAR  = 0x111,
    ALLEGRO_MIXER_QUALITY_CUBIC   = 0x112 
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum AlPlaymode {
    ALLEGRO_PLAYMODE_ONCE   = 0x100,
    ALLEGRO_PLAYMODE_LOOP   = 0x101,
    ALLEGRO_PLAYMODE_BIDIR  = 0x102,
    _ALLEGRO_PLAYMODE_STREAM_ONCE   = 0x103,   /* internal */
    _ALLEGRO_PLAYMODE_STREAM_ONEDIR = 0x104    /* internal */
}


#[derive(Default)]
#[repr(C)]
pub struct AlSampleID {
    _index: i32,
    _id: i32
}

pub const ALLEGRO_AUDIO_PAN_NONE: f32 = -1000.0;

pub type AlMixer = c_void;
pub type AlSample = c_void;
pub type AlSampleInstance = c_void;
pub type AlAudioStream = c_void;
pub type AlVoice = c_void;
pub type AlAudioRecorder = c_void;
pub type AlAudioRecorderEvent = c_void;

#[link(name="liballegro_monolith.dll")]
extern {
    // Setting up audio
    pub fn al_install_audio() -> bool;
    pub fn al_uninstall_audio();
    pub fn al_is_audio_installed() -> bool;
    pub fn al_reserve_samples(reserve_samples: i32) -> bool;

    // Misc audio functions
    pub fn al_get_allegro_audio_version() -> u32;
    pub fn al_get_audio_depth_size(depth: AlAudioDepth) -> size_t;
    pub fn al_get_channel_count(conf: AlChannelConf) -> size_t;
    pub fn al_fill_silence(buf: *mut c_void, samples: u32, depth: AlAudioDepth, chan_conf: AlChannelConf);
    
    // Voice functions
    pub fn al_create_voice(freq: u32, depth: AlAudioDepth, chan_conf: AlChannelConf) -> *const AlVoice;
    pub fn al_destroy_voice(voice: *const AlVoice);
    pub fn al_detach_voice(voice: *const AlVoice);
    pub fn al_attach_audio_stream_to_voice(stream: *const AlAudioStream, voice: *const AlVoice) -> bool;
    pub fn al_attach_mixer_to_voice(mixer: *const AlMixer, voice: *const AlVoice) -> bool;
    pub fn al_attach_sample_instance_to_voice(spl: *const AlSampleInstance, voice: *const AlVoice) -> bool;
    pub fn al_get_voice_frequency(voice: *const AlVoice) -> u32;
    pub fn al_get_voice_channels(voice: *const AlVoice) -> AlChannelConf;
    pub fn al_get_voice_depth(voice: *const AlVoice) -> AlAudioDepth;
    pub fn al_get_voice_playing(voice: *const AlVoice) -> bool;
    pub fn al_set_voice_playing(voice: *const AlVoice, val: bool ) -> bool;
    pub fn al_get_voice_position(voice: *const AlVoice) -> u32;
    pub fn al_set_voice_position(voice: *const AlVoice, val: u32) -> bool;

    // Sample functions
    pub fn al_create_sample(buf: *const c_void, samples: u32, freq: u32, depth: AlAudioDepth, chan_conf: AlChannelConf, free_buf: bool) -> *const AlSample;
    pub fn al_destroy_sample(spl: *const AlSample);
    pub fn al_play_sample(spl: *const AlSample, gain: f32, pan: f32 , speed: f32, r#loop: AlPlaymode, ret_id: *mut AlSampleID) -> bool;
    pub fn al_stop_sample(spl_id: *const AlSampleID);
    pub fn al_lock_sample_id(spl_id: *const AlSampleID) -> *const AlSampleInstance ;
    pub fn al_unlock_sample_id(spl_id: *const AlSampleID);
    pub fn al_stop_samples();
    pub fn al_get_sample_channels(spl: *const AlSample) -> AlChannelConf;
    pub fn al_get_sample_depth(spl: *const AlSample) -> AlAudioDepth;
    pub fn al_get_sample_frequency(spl: *const AlSample) -> u32;
    pub fn al_get_sample_length(spl: *const AlSample) -> u32;
    pub fn al_get_sample_data(spl: *const AlSample) -> *const c_void;

    // Sample instance functions
    pub fn al_create_sample_instance(sample_data: *const AlSample) -> *const AlSampleInstance;
    pub fn al_destroy_sample_instance(spl: *const AlSampleInstance);
    pub fn al_play_sample_instance(spl: *const AlSampleInstance) -> bool;
    pub fn al_stop_sample_instance(spl: *const AlSampleInstance) -> bool;
    pub fn al_get_sample_instance_channels(spl: *const AlSampleInstance) -> AlChannelConf;
    pub fn al_get_sample_instance_depth(spl: *const AlSampleInstance) -> AlAudioDepth;
    pub fn al_get_sample_instance_frequency(spl: *const AlSampleInstance) -> u32;
    pub fn al_get_sample_instance_length(spl: *const AlSampleInstance) -> u32;
    pub fn al_set_sample_instance_length(spl: *const AlSampleInstance, val: u32) -> bool;
    pub fn al_get_sample_instance_position(spl: *const AlSampleInstance) -> u32;
    pub fn al_set_sample_instance_position(spl: *const AlSampleInstance, val: u32) -> bool;
    pub fn al_get_sample_instance_speed(spl: *const AlSampleInstance) -> f32;
    pub fn al_set_sample_instance_speed(spl: *const AlSampleInstance, val: f32) -> bool;
    pub fn al_get_sample_instance_gain(spl: *const AlSampleInstance) -> f32;
    pub fn al_set_sample_instance_gain(spl: *const AlSampleInstance, val: f32) -> bool;
    pub fn al_get_sample_instance_pan(spl: *const AlSampleInstance) -> f32;
    pub fn al_set_sample_instance_pan(spl: *const AlSampleInstance, val: f32) -> bool;
    pub fn al_get_sample_instance_time(spl: *const AlSampleInstance) -> f32;
    pub fn al_get_sample_instance_playmode(spl: *const AlSampleInstance) -> AlPlaymode;
    pub fn al_set_sample_instance_playmode(spl: *const AlSampleInstance, val: AlPlaymode) -> bool;
    pub fn al_get_sample_instance_playing(spl: *const AlSampleInstance) -> bool;
    pub fn al_set_sample_instance_playing(spl: *const AlSampleInstance, val: bool) -> bool;
    pub fn al_get_sample_instance_attached(spl: *const AlSampleInstance) -> bool;
    pub fn al_detach_sample_instance(spl: *const AlSampleInstance) -> bool;
    pub fn al_get_sample(spl: *const AlSampleInstance) -> *const AlSample;
    pub fn al_set_sample(spl: *const AlSampleInstance, data: *const AlSample) -> bool;
    pub fn al_set_sample_instance_channel_matrix(spl: *const AlSampleInstance, matrix: *const f32) -> bool;

    // Mixer functions
    pub fn al_create_mixer(freq: u32, depth: AlAudioDepth, chan_conf: AlChannelConf) -> *const AlMixer;
    pub fn al_destroy_mixer(mixer: *const AlMixer);
    pub fn al_get_default_mixer() -> *const AlMixer;
    pub fn al_set_default_mixer(mixer: *const AlMixer) -> bool;
    pub fn al_restore_default_mixer() -> bool;
    pub fn al_get_default_voice() -> *const AlVoice;
    pub fn al_set_default_voice(voice: *const AlVoice);
    pub fn al_attach_mixer_to_mixer(stream: *const AlMixer, mixer: *const AlMixer) -> bool;
    pub fn al_attach_sample_instance_to_mixer(spl: *const AlSampleInstance, mixer: *const AlMixer) -> bool;
    pub fn al_attach_audio_stream_to_mixer(stream: *const AlAudioStream, mixer: *const AlMixer) -> bool;
    pub fn al_get_mixer_frequency(mixer: *const AlMixer) -> u32;
    pub fn al_set_mixer_frequency(mixer: *const AlMixer, val: u32) -> bool;
    pub fn al_get_mixer_channels(mixer: *const AlMixer) -> AlChannelConf;
    pub fn al_get_mixer_depth(mixer: *const AlMixer) -> AlAudioDepth;
    pub fn al_get_mixer_gain(mixer: *const AlMixer) -> f32;
    pub fn al_set_mixer_gain(mixer: *const AlMixer, new_gain: f32) -> bool;
    pub fn al_get_mixer_quality(mixer: *const AlMixer) -> AlMixerQuality;
    pub fn al_set_mixer_quality(mixer: *const AlMixer, new_quality: AlMixerQuality) -> bool;
    pub fn al_get_mixer_playing(mixer: *const AlMixer) -> bool;
    pub fn al_set_mixer_playing(mixer: *const AlMixer, val: bool) -> bool;
    pub fn al_get_mixer_attached(mixer: *const AlMixer) -> bool;
    pub fn al_detach_mixer(mixer: *const AlMixer) -> bool;
    pub fn al_set_mixer_postprocess_callback(mixer: *const AlMixer, pp_callback: extern fn(buf: *const c_void, samples: u32, data: *const c_void), pp_callback_userdata: *const c_void) -> bool;
    
    // Stream functions
    pub fn al_create_audio_stream(fragment_count: size_t, frag_samples: u32, freq: u32, depth: AlAudioDepth, chan_conf: AlChannelConf) -> *const AlAudioStream;
    pub fn al_destroy_audio_stream(stream: *const AlAudioStream);
    pub fn al_get_audio_stream_event_source( stream: *const AlAudioStream) -> *const AlEventSource;
    pub fn al_drain_audio_stream(stream: *const AlAudioStream);
    pub fn al_rewind_audio_stream(stream: *const AlAudioStream) -> bool;
    pub fn al_get_audio_stream_frequency(stream: *const AlAudioStream) -> u32;
    pub fn al_get_audio_stream_channels(stream: *const AlAudioStream) -> AlChannelConf;
    pub fn al_get_audio_stream_depth(stream: *const AlAudioStream) -> AlAudioDepth;
    pub fn al_get_audio_stream_length(stream: *const AlAudioStream) -> u32;
    pub fn al_get_audio_stream_speed(stream: *const AlAudioStream) -> f32;
    pub fn al_set_audio_stream_speed(stream: *const AlAudioStream, val: f32) -> bool;
    pub fn al_get_audio_stream_gain(stream: *const AlAudioStream) -> f32;
    pub fn al_set_audio_stream_gain(stream: *const AlAudioStream, val: f32) -> bool;
    pub fn al_get_audio_stream_pan(stream: *const AlAudioStream) -> f32;
    pub fn al_set_audio_stream_pan(stream: *const AlAudioStream, val: f32) -> bool;
    pub fn al_get_audio_stream_playing(stream: *const AlAudioStream) -> bool;
    pub fn al_set_audio_stream_playing(stream: *const AlAudioStream, val: bool) -> bool;
    pub fn al_get_audio_stream_playmode(stream: *const AlAudioStream) -> AlPlaymode;
    pub fn al_set_audio_stream_playmode(stream: *const AlAudioStream, val: AlPlaymode) -> bool;
    pub fn al_get_audio_stream_attached(stream: *const AlAudioStream) -> bool;
    pub fn al_detach_audio_stream(stream: *const AlAudioStream) -> bool;
    pub fn al_get_audio_stream_played_samples(stream: *const AlAudioStream) -> u64;
    pub fn al_get_audio_stream_fragment(stream: *const AlAudioStream) -> *const c_void;
    pub fn al_set_audio_stream_fragment(stream: *const AlAudioStream, val: *const c_void) -> bool;
    pub fn al_get_audio_stream_fragments(stream: *const AlAudioStream) -> u32;
    pub fn al_get_available_audio_stream_fragments(stream: *const AlAudioStream) -> u32;
    pub fn al_seek_audio_stream_secs(stream: *const AlAudioStream, time: f64) -> bool;
    pub fn al_get_audio_stream_position_secs(stream: *const AlAudioStream) -> f64;
    pub fn al_get_audio_stream_length_secs(stream: *const AlAudioStream) -> f64;
    pub fn al_set_audio_stream_loop_secs(stream: *const AlAudioStream, start: f64, end: f64) -> bool;
    pub fn al_set_audio_stream_channel_matrix(stream: *const AlAudioStream, matrix: *const f32) -> bool;

    // Audio file I/O
    pub fn al_register_sample_loader(ext: *const c_char, loader: extern fn(filename: *const c_char) -> *const AlSample) -> bool;
    pub fn al_register_sample_loader_f(ext: *const c_char, loader: extern fn(fp: *const AlFile) -> *const AlSample) -> bool;
    pub fn al_register_sample_saver(ext: *const c_char, saver: extern fn(filename: *const c_char, spl: *const AlSample) -> bool) -> bool;
    pub fn al_register_sample_saver_f(ext: *const c_char, saver: extern fn(fp: *const AlFile, spl: *const AlSample) -> bool) -> bool;
    pub fn al_register_audio_stream_loader(ext: *const c_char, stream_loader: extern fn(filename: *const c_char, buffer_count: size_t, samples: u32) -> *const AlAudioStream) -> bool;
    pub fn al_register_audio_stream_loader_f(ext: *const c_char, stream_loader: extern fn(fp: *const AlFile, buffer_count: size_t, samples: u32) -> *const AlAudioStream) -> bool;
    pub fn al_load_sample(filename: *const c_char) -> *const AlSample;
    pub fn al_load_sample_f(fp: *const AlFile, ident: *const c_char) -> *const AlSample;
    pub fn al_load_audio_stream(filename: *const c_char, buffer_count: size_t, samples: u32) -> *const AlAudioStream;
    pub fn al_load_audio_stream_f(fp: *const AlFile, ident: *const c_char, buffer_count: size_t, samples: u32) -> *const AlAudioStream;
    pub fn al_save_sample(filename: *const c_char, spl: *const AlSample) -> bool;
    pub fn al_save_sample_f(fp: *const AlFile, ident: *const c_char, spl: *const AlSample) -> bool;

    // Audio recording
    pub fn al_create_audio_recorder(fragment_count: size_t, samples: u32, frequency: u32, depth: AlAudioDepth, chan_conf: AlChannelConf) -> *const AlAudioRecorder;
    pub fn al_start_audio_recorder(r: *const AlAudioRecorder) -> bool;
    pub fn al_stop_audio_recorder(r: *const AlAudioRecorder);
    pub fn al_is_audio_recorder_recording(r: *const AlAudioRecorder) -> bool;
    pub fn al_get_audio_recorder_event(event: *const AlEvent) -> *const AlAudioRecorderEvent;
    pub fn al_get_audio_recorder_event_source(r: *const AlAudioRecorder) -> *const AlEventSource;
    pub fn al_destroy_audio_recorder(r: *const AlAudioRecorder);
}