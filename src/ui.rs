
use std::os::raw::{c_char, c_uint, c_void};
use lv2_raw;

pub struct LV2UIController(pub lv2_raw::LV2UIControllerRaw);

unsafe impl Send for LV2UIController {}

impl Copy for LV2UIController { }

impl Clone for LV2UIController {
    fn clone(&self) -> LV2UIController {
        *self
    }
}

pub type LV2UIWriteFunction = Option<extern "C" fn(controller: LV2UIController,
                                                      port_index: c_uint,
                                                      buffer_size: c_uint,
                                                      port_protocol: c_uint,
                                                      buffer: *const c_void)>;

#[repr(C)]
pub struct LV2UIDescriptor {
    pub uri: *const c_char,
    pub instantiate: extern "C" fn(descriptor: *const LV2UIDescriptor,
                                       plugin_uri: *const c_char,
                                       bundle_path: *const c_char,
                                       write_function: LV2UIWriteFunction,
                                       controller: LV2UIController,
                                       widget: *mut lv2_raw::LV2UIWidget,
                                       features: *const (*const lv2_raw::LV2Feature))
                                       -> lv2_raw::LV2UIHandle,

    pub cleanup: extern "C" fn(lv2_raw::LV2UIHandle),
    pub port_event: extern "C" fn(ui: lv2_raw::LV2UIHandle,
                                      port_index: c_uint,
                                      buffer_size: c_uint,
                                      format: c_uint,
                                      buffer: *const c_void),
    pub extension_data: Option<extern "C" fn(*const c_char) -> *const c_void>,
}
