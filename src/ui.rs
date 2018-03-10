
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr::{null, null_mut};
use lv2_raw::ui::{LV2UIDescriptor, LV2UIHandle, LV2UIControllerRaw, LV2UIWidget, LV2UIWriteFunctionRaw, LV2UIExternalUIWidget, LV2UIExternalUIHost};
use lv2_raw::core::{LV2Feature};
use libc::strcmp;


/// A group of plugin methods that are defined by the plugin and called by the host.
pub trait PluginUI {
    fn instantiate(human_id: &str) -> Self;
    fn cleanup(&mut self) {}
    fn port_event() {}

    fn run<F: Fn(u32, f32)>(&mut self, send_float: F) -> bool;
    fn show(&mut self);
    fn hide(&mut self);
}

pub struct PluginUIExt<P> {
    widget: LV2UIExternalUIWidget,
    host: *const LV2UIExternalUIHost,
    controller: LV2UIControllerRaw,
    write_function: LV2UIWriteFunctionRaw, // TODO: make this Option<...> field a raw fn pointer
    plugin: P,
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
            extension_data: None,
        };

        #[no_mangle]
        pub extern "C" fn lv2ui_descriptor(index: i32) -> *const lv2::LV2UIDescriptor {
            eprintln!("ui_descriptor");
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
    }
}

#[allow(non_upper_case_globals)]
const LV2_EXTERNAL_UI__Host: &[u8] = b"http://kxstudio.sf.net/ns/lv2ext/external-ui#Host\0";

#[doc(hidden)]
pub extern "C" fn instantiate<P: PluginUI>(
    _descriptor: *const LV2UIDescriptor,
    _plugin_uri: *const c_char,
    _bundle_path: *const c_char,
    write_function: LV2UIWriteFunctionRaw,
    controller: LV2UIControllerRaw,
    widget: *mut LV2UIWidget,
    features: *const (*const LV2Feature))
-> LV2UIHandle {
    let mut p_feature = features;
    let mut host: *const LV2UIExternalUIHost = null();
    unsafe {
        while *p_feature != null() {
            if strcmp((**p_feature).uri, LV2_EXTERNAL_UI__Host as *const _ as *const i8) == 0 {
                host = (**p_feature).data as *const LV2UIExternalUIHost;
            }
            p_feature = p_feature.offset(1);
        }
    }
    if host == null() {
        return null_mut();
    }

    let human_id =
        if let Ok(h_id) = unsafe { ::std::ffi::CStr::from_ptr((*host).plugin_human_id).to_str() } {
            h_id
        } else {
            return null_mut();
        };
    if let None = write_function {
        return null_mut();
    }
    let plugin_ext = PluginUIExt {
        widget: LV2UIExternalUIWidget {
            run: Some(run::<P>),
            show: Some(show::<P>),
            hide: Some(hide::<P>),
        },
        host,
        controller,
        write_function,
        plugin: P::instantiate(human_id),
    };

    let mut t = Box::new(plugin_ext);
    let ptr = &mut *t as *mut _ as *mut c_void;

    unsafe {
        let w = widget as *mut *const LV2UIExternalUIWidget;
        *w = (&t.widget) as *const _ as *const LV2UIExternalUIWidget;
    }

    ::std::mem::forget(t);
    return ptr;
}

#[doc(hidden)]
pub extern "C" fn cleanup<P: PluginUI>(handle: LV2UIHandle) {
    let mut plugin: Box<PluginUIExt<P>> = unsafe { ::std::mem::transmute(handle) };
    plugin.plugin.cleanup();
}

#[doc(hidden)]
pub extern "C" fn port_event<P: PluginUI>(
    _ui: LV2UIHandle,
    _port_index: c_uint,
    _buffer_size: c_uint,
    _format: c_uint,
    _buffer: *const c_void)
{
    eprintln!("port event");
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(0 as *const $ty)).$field as *const _ as isize
    }
}

pub extern "C" fn run<P: PluginUI>(ui: *const LV2UIExternalUIWidget) {
    unsafe {
        let plugin_ext = &mut *(ui.offset(-offset_of!(PluginUIExt<P>, widget)) as *mut PluginUIExt<P>);
        // NOTE: the `instantiate` function should check that this `unwrap` is safe
        let writefn_ptr = plugin_ext.write_function.unwrap();
        let controller = plugin_ext.controller;
        let writefn = |port, value| (writefn_ptr)(controller, port, 4, 0, &*Box::new(value) as *const _ as *const c_void);

        if !plugin_ext.plugin.run(writefn) {
            ((*plugin_ext.host).ui_closed)(plugin_ext.controller);
        }
    }
}

pub extern "C" fn show<P: PluginUI>(ui: *const LV2UIExternalUIWidget) {
    unsafe {
        (*(ui.offset(-offset_of!(PluginUIExt<P>, widget)) as *mut PluginUIExt<P>)).plugin.show();
    }
}

pub extern "C" fn hide<P: PluginUI>(ui: *const LV2UIExternalUIWidget) {
    unsafe {
        (*(ui.offset(-offset_of!(PluginUIExt<P>, widget)) as *mut PluginUIExt<P>)).plugin.hide();
    }
}
