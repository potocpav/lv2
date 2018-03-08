
use std::os::raw::{c_char, c_int, c_uint, c_void};
use lv2_raw::ui::{LV2UIDescriptor, LV2UIHandle, LV2UIControllerRaw, LV2UIWidget, LV2UIWriteFunctionRaw};
use lv2_raw::core::{LV2Feature};


/// A group of plugin methods that are defined by the plugin and called by the host.
pub trait PluginUI {
    fn instantiate() -> Self;
    fn cleanup(&mut self) {}
    fn port_event() {}

    // showInterface extension
    fn show(&mut self);
    fn hide(&mut self);

    // idleInterface extension
    fn idle(&mut self);
}

/// This macro takes a type which must implement the trait `PluginUI`
#[macro_export]
macro_rules! plugin_ui {
    ($t:ty, $url:expr) => {
        static mut DESC: lv2::LV2UIDescriptor = lv2::LV2UIDescriptor {
            uri: 0 as *const std::os::raw::c_char,
            instantiate: lv2::ui::instantiate::<$t>,
            cleanup: lv2::ui::cleanup::<$t>,
            port_event: lv2::ui::port_event::<$t>,
            extension_data: Some(extension_data::<$t>),
        };

        #[no_mangle]
        pub extern "C" fn lv2ui_descriptor(index: i32) -> *const lv2::LV2UIDescriptor {
            if index != 0 {
                return std::ptr::null();
            } else {
                let ptr = std::ffi::CStr::from_bytes_with_nul($url).unwrap().as_ptr() as *const std::os::raw::c_char;
                unsafe {
                    DESC.uri = ptr;
                    return &DESC as *const lv2::LV2UIDescriptor;
                }
            }
        }

        // TODO: move to the lv2_raw library, maybe gen automatically from `ui.h`?
        #[allow(non_upper_case_globals)]
        const LV2_UI__idleInterface: &[u8] = b"http://lv2plug.in/ns/extensions/ui#idleInterface\0";
        #[allow(non_upper_case_globals)]
        const LV2_UI__showInterface: &[u8] = b"http://lv2plug.in/ns/extensions/ui#showInterface\0";

        use std::os::raw::{c_void, c_char};

        static mut IDLE_INTERFACE: lv2::LV2UIIdleInterface = lv2::LV2UIIdleInterface {
            idle: lv2::ui::idle::<$t>,
        };

        static mut SHOW_INTERFACE: lv2::LV2UIShowInterface = lv2::LV2UIShowInterface {
            show: lv2::ui::show::<$t>,
            hide: lv2::ui::hide::<$t>,
        };

        extern "C" fn extension_data<T: lv2::PluginUI>(uri: *const c_char) -> (*const c_void) {
            unsafe {
                if lv2::strcmp(uri, LV2_UI__idleInterface as *const _ as *const i8) == 0 {
                    return &IDLE_INTERFACE as *const _ as *const c_void;
                } else if lv2::strcmp(uri, LV2_UI__showInterface as *const _ as *const i8) == 0 {
                    return &SHOW_INTERFACE as *const _ as *const c_void;
                } else {
                    ::std::ptr::null()
                }
            }
        }
    }
}

#[doc(hidden)]
pub extern "C" fn instantiate<P: PluginUI>(
    _descriptor: *const LV2UIDescriptor,
    _plugin_uri: *const c_char,
    _bundle_path: *const c_char,
    _write_function: LV2UIWriteFunctionRaw,
    _controller: LV2UIControllerRaw,
    _widget: *mut LV2UIWidget,
    _features: *const (*const LV2Feature))
-> LV2UIHandle {
    let mut t = Box::new(P::instantiate());
    let ptr = &mut *t as *mut _ as *mut c_void;
    ::std::mem::forget(t);
    return ptr;
}

#[doc(hidden)]
pub extern "C" fn cleanup<P: PluginUI>(handle: LV2UIHandle) {
    let mut plugin: Box<P> = unsafe { ::std::mem::transmute(handle) };
    plugin.cleanup();
}

#[doc(hidden)]
pub extern "C" fn port_event<P: PluginUI>(
    _ui: LV2UIHandle,
    _port_index: c_uint,
    _buffer_size: c_uint,
    _format: c_uint,
    _buffer: *const c_void)
{
    unimplemented!()
}

pub extern "C" fn idle<P: PluginUI>(ui: LV2UIHandle) -> c_int {
    unsafe {
        (*(ui as *mut P)).idle();
    }
    0
}

pub extern "C" fn show<P: PluginUI>(ui: LV2UIHandle) -> c_int {
    unsafe {
        (*(ui as *mut P)).show();
    }
    0
}

pub extern "C" fn hide<P: PluginUI>(ui: LV2UIHandle) -> c_int {
    unsafe {
        (*(ui as *mut P)).hide();
    }
    0
}
