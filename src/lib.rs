#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

extern crate libc;
use libc::{ c_void, c_char, size_t };

mod system_info;
use system_info::SystemInfo;

mod system_av_info;
use system_av_info::SystemAvInfo;

mod callbacks;
use callbacks::*;

mod context;
use context::Context;

static mut GLOBAL_CALLBACKS: Callbacks = Callbacks {
	environment_fn: None,
	video_refresh_fn: None,
	audio_sample_fn: None,
	audio_sample_batch_fn: None,
	input_poll_fn: None,
	input_state_fn: None
};

static mut GLOBAL_CONTEXT: *mut Context = 0 as *mut _;

unsafe fn has_context() -> bool {
	!GLOBAL_CONTEXT.is_null()
}

unsafe fn get_context() -> &'static mut Context {
	if !has_context() {
		panic!("Attempted to access global context outside of game");
	}

	&mut *GLOBAL_CONTEXT
}

unsafe fn set_context(context: Box<Context>) {
	if has_context() {
		panic!("Attempted to set global context when one already exists");
	}

	GLOBAL_CONTEXT = Box::into_raw(context);
}

unsafe fn delete_context() {
	if !has_context() {
		panic!("Attempted to delete non-existent context");
	}

	// This frees GLOBAL_CONTEXT, since the newly created Box goes out of scope immediately
	Box::from_raw(GLOBAL_CONTEXT);
	GLOBAL_CONTEXT = 0 as *mut _;
}

// libretro API
// ------------

macro_rules! not_implemented {
    ( $fname:expr ) => {
    	panic!(concat!($fname, "(): not yet implemented"));
    }
}

#[no_mangle]
pub unsafe extern fn retro_set_environment(callback: EnvironmentCallback) {
	GLOBAL_CALLBACKS.environment_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_video_refresh(callback: VideoRefreshCallback) {
	GLOBAL_CALLBACKS.video_refresh_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_audio_sample(callback: AudioSampleCallback) {
	GLOBAL_CALLBACKS.audio_sample_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_audio_sample_batch(callback: AudioSampleBatchCallback) {
	GLOBAL_CALLBACKS.audio_sample_batch_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_input_poll(callback: InputPollCallback) {
	GLOBAL_CALLBACKS.input_poll_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_input_state(callback: InputStateCallback) {
	GLOBAL_CALLBACKS.input_state_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_get_system_info(info: *mut SystemInfo) {
	*info = SystemInfo::new();
}

#[no_mangle]
pub unsafe extern fn retro_get_system_av_info(av_info: *mut SystemAvInfo) {
	*av_info = get_context().system_av_info();
}

#[no_mangle]
pub unsafe extern fn retro_set_controller_port_device(port: u32, device: u32) {
	not_implemented!("retro_set_controller_port_device");
}

#[no_mangle]
pub unsafe extern fn retro_init() {}

#[no_mangle]
pub unsafe extern fn retro_deinit() {}

#[no_mangle]
pub unsafe extern fn retro_load_game() {
	not_implemented!("retro_load_game");
}

#[no_mangle]
pub unsafe extern fn retro_load_game_special(game_type: u32, game_infos: *const GameInfo, num_game_infos: size_t) -> bool {
	// Neither required nor recommended
	false
}

#[no_mangle]
pub unsafe extern fn retro_unload_game() {
	not_implemented!("retro_unload_game");
}

#[no_mangle]
pub unsafe extern fn retro_get_region() -> u32 {
	not_implemented!("retro_get_region");
}

#[no_mangle]
pub unsafe extern fn retro_get_memory_data(id: u32) -> *mut c_void {
	not_implemented!("retro_get_memory_data");
}

#[no_mangle]
pub unsafe extern fn retro_get_memory_size(id: u32) -> size_t {
	not_implemented!("retro_get_memory_size");
}

#[no_mangle]
pub unsafe extern fn retro_reset() {
	not_implemented!("retro_reset");
}

#[no_mangle]
pub unsafe extern fn retro_run() {
	not_implemented!("retro_run");
}

#[no_mangle]
pub unsafe extern fn retro_serialize_size() -> size_t {
	0
}

#[no_mangle]
pub unsafe extern fn retro_serialize(data: *mut c_void, size: size_t) -> bool {
	not_implemented!("retro_serialize");
}

#[no_mangle]
pub unsafe extern fn retro_unserialize(data: *const c_void, size: size_t) -> bool {
	not_implemented!("retro_unserialize");
}

#[no_mangle]
pub unsafe extern fn retro_cheat_reset() {
	not_implemented!("retro_cheat_reset");
}

#[no_mangle]
pub unsafe extern fn retro_cheat_set(index: u32, enabled: bool, code: *const c_char) {
	not_implemented!("retro_cheat_set");
}

#[no_mangle]
pub extern fn retro_api_version() -> u32 { 1 }