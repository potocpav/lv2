extern crate lv2_raw;

use std::os::raw::c_void;

/// A group of plugin methods that are defined by the plugin and called by the host.
pub trait Plugin {
    fn initialize() -> Self;
    fn activate(&mut self) {}
    fn run(&mut self, buffers: &mut [&mut [f32]]);
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

/// This macro takes a type which must implement the trait `Plugin`
#[macro_export]
macro_rules! plugin {
    ($t:ty, $url:expr) => {
        static mut DESC: lv2::LV2Descriptor = lv2::LV2Descriptor {
            uri: 0 as *const std::os::raw::c_char,
            instantiate: lv2::instantiate::<$t>,
            connect_port: lv2::connect_port::<$t>,
            activate: Some(lv2::activate::<$t>),
            run: lv2::run::<$t>,
            deactivate: Some(lv2::deactivate::<$t>),
            cleanup: lv2::cleanup::<$t>,
            extension_data: lv2::extension_data,
        };


        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: i32) -> *const lv2::LV2Descriptor {
            if index != 0 {
                return std::ptr::null();
            } else {
                let ptr = std::ffi::CStr::from_bytes_with_nul($url).unwrap().as_ptr() as *const std::os::raw::c_char;
                unsafe {
                    DESC.uri = ptr;
                    return &DESC as *const lv2::LV2Descriptor;
                }
            }
        }
    }
}

const MAX_N_PORTS: usize = 32;

#[doc(hidden)]
pub struct PluginExtras<P> {
    port_bufs: [*mut f32; MAX_N_PORTS],
    plugin: P,
}

#[doc(hidden)]
impl<P> PluginExtras<P> {
    pub fn new(p: P) -> Self {
        PluginExtras {
            port_bufs: [0 as *mut _; MAX_N_PORTS],
            plugin: p,
        }
    }
}

#[doc(hidden)]
pub extern "C" fn instantiate<T: Plugin>(_descriptor: *const lv2_raw::LV2Descriptor,
                                         _rate: f64,
                                         _bundle_path: *const i8,
                                         _features: *const *const lv2_raw::LV2Feature)
                                         -> lv2_raw::LV2Handle {

    let mut t = Box::new(PluginExtras::new(T::initialize()));
    let ptr = &mut *t as *mut _ as *mut c_void;
    ::std::mem::forget(t);
    return ptr;
}

#[doc(hidden)]
pub extern "C" fn connect_port<P: Plugin>(handle: lv2_raw::LV2Handle,
                                          port: u32,
                                          data: *mut c_void) {
    assert!((port as usize) < MAX_N_PORTS);
    let d = data as *mut f32;
    let plgptr = handle as *mut PluginExtras<P>;
    unsafe {
        (*plgptr).port_bufs[port as usize] = d;
    }
}

#[doc(hidden)]
pub extern "C" fn activate<P: Plugin>(instance: lv2_raw::LV2Handle) {
    unsafe {
        (*(instance as *mut PluginExtras<P>)).plugin.activate();
    }
}

#[doc(hidden)]
pub extern "C" fn run<P: Plugin>(instance: lv2_raw::LV2Handle, n_samples: u32) {
    let plgptr = instance as *mut PluginExtras<P>;
    unsafe {
        let mut buffers: [&mut [f32]; MAX_N_PORTS] = Default::default();
        for i in 0..MAX_N_PORTS {
            if (*plgptr).port_bufs[i] == ::std::ptr::null_mut() {
                buffers[i] = &mut [];
            } else {
                buffers[i] = ::std::slice::from_raw_parts_mut((*plgptr).port_bufs[i], n_samples as usize);
            }
        }

        (*plgptr).plugin.run(&mut buffers)
    }
}

#[doc(hidden)]
pub extern "C" fn deactivate<P: Plugin>(instance: lv2_raw::LV2Handle) {
    unsafe {
        (*(instance as *mut PluginExtras<P>)).plugin.deactivate();
    }
}

#[doc(hidden)]
pub extern "C" fn cleanup<P: Plugin>(instance: lv2_raw::LV2Handle) {
    let mut plugin: Box<PluginExtras<P>> = unsafe { ::std::mem::transmute(instance) };
    plugin.plugin.cleanup();
}

#[doc(hidden)]
pub extern "C" fn extension_data(_uri: *const u8) -> (*const c_void) {
    ::std::ptr::null()
}
