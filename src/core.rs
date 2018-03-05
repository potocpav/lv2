extern crate lv2_raw;

/// A group of plugin methods that are defined by the plugin and called
/// by the host.
pub trait Plugin {
    /// Does everything `instantiate()` does in the C code, except allocating
    /// memory.
    fn initialize() -> Self;
    // fn connect_port(&mut self, _port: u32, _data: &'a mut [f32]) {}
    fn activate(&mut self) {}
    fn run(&mut self, buffers: &mut [&mut [f32]]);
    fn deactivate(&mut self) {}
    fn cleanup(&mut self) {}
}

/// Exports the necessary symbols for the plugin to be used by a VST host.
///
/// This macro takes a type which must implement the traits `plugin::Plugin` and
/// `std::default::Default`.
#[macro_export]
macro_rules! plugin {
    ($t:ty, $url:expr) => {
        const MAX_N_PORTS: usize = 32;

        pub struct PluginExtras<P> {
            port_bufs: [*mut f32; MAX_N_PORTS],
            plugin: P,
        }

        impl<P> PluginExtras<P> {
            pub fn new(p: P) -> Self {
                PluginExtras {
                    port_bufs: [0 as *mut _; MAX_N_PORTS],
                    plugin: p,
                }
            }
        }

        static mut DESC: lv2::LV2Descriptor = lv2::LV2Descriptor {
            uri: 0 as *const libc::c_char, // ptr::null() isn't const fn (yet)
            instantiate: instantiate::<$t>,
            connect_port: connect_port::<$t>,
            activate: Some(activate),
            run: run::<$t>,
            deactivate: Some(deactivate),
            cleanup: cleanup,
            extension_data: extension_data,
        };

        extern "C" fn instantiate<T: lv2::Plugin>(_descriptor: *const lv2::LV2Descriptor,
                                                            _rate: f64,
                                                            _bundle_path: *const i8,
                                                            _features: *const *const lv2::LV2Feature)
                                                            -> lv2::LV2Handle {

            let mut t = Box::new(PluginExtras::new(T::initialize()));
            let ptr = &mut *t as *mut _ as *mut libc::c_void;
            ::std::mem::forget(t);
            return ptr;
        }

        extern "C" fn connect_port<P: lv2::Plugin>(handle: lv2::LV2Handle,
                                                             port: u32,
                                                             data: *mut libc::c_void) {
            assert!((port as usize) < MAX_N_PORTS);
            let d = data as *mut f32;
            let plgptr = handle as *mut PluginExtras<P>;
            unsafe {
                (*plgptr).port_bufs[port as usize] = d;
                // TODO: This should be sample_count. How do we get that number? During initialization? Set to some random (high) number.
                // https://www.alsa-project.org/main/index.php/FramesPeriods
                // let bs: &mut [f32] = ::std::slice::from_raw_parts_mut(d, 65536 * ::std::mem::size_of::<f32>());
                // (*plgptr).plugin.connect_port(port, bs)
            }
        }

        extern "C" fn activate(_instance: lv2::LV2Handle) {}
        extern "C" fn run<P: lv2::Plugin>(instance: lv2::LV2Handle, n_samples: u32) {
            let plgptr = instance as *mut PluginExtras<P>;
            unsafe {
                let mut buffers: [&mut [f32]; MAX_N_PORTS] = Default::default();
                for i in 0..MAX_N_PORTS {
                    if (*plgptr).port_bufs[i] == std::ptr::null_mut() {
                        buffers[i] = &mut [];
                    } else {
                        buffers[i] = std::slice::from_raw_parts_mut((*plgptr).port_bufs[i], n_samples as usize);
                    }
                }

                (*plgptr).plugin.run(&mut buffers)
            }
        }

        extern "C" fn deactivate(_instance: lv2::LV2Handle) {}
        extern "C" fn cleanup(_instance: lv2::LV2Handle) {
/*
            unsafe {
                // TODO: deallocate the stuff
                // ptr::read(instance as *mut Amp); // no need for this?
                //libc::free(instance as lv2::LV2Handle)
            }*/
        }
        extern "C" fn extension_data(_uri: *const u8) -> (*const libc::c_void) {
            ptr::null()
        }


        #[no_mangle]
        pub extern "C" fn lv2_descriptor(index: i32) -> *const lv2::LV2Descriptor {
            if index != 0 {
                return ptr::null();
            } else {
                let ptr = ::std::ffi::CStr::from_bytes_with_nul($url).unwrap().as_ptr() as *const libc::c_char;
                unsafe {
                    DESC.uri = ptr;
                    return &DESC as *const lv2::LV2Descriptor;
                }
            }
        }
    }
}
