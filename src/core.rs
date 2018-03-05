extern crate lv2_raw;

/// A group of plugin methods that are defined by the plugin and called
/// by the host.
pub trait Plugin<'a> {
    /// Does everything `instantiate()` does in the C code, except allocating
    /// memory.
    fn initialize(&mut self) {}
    fn connect_port(&mut self, _port: u32, _data: &'a mut [f32]) {}
    fn activate(&mut self) {}
    fn run(&mut self, _n_samples: u32) {}
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

        extern "C" fn instantiate<'a, T: lv2::Plugin<'a>>(_descriptor: *const lv2::LV2Descriptor,
                                                            _rate: f64,
                                                            _bundle_path: *const i8,
                                                            _features: *const *const lv2::LV2Feature)
                                                            -> lv2::LV2Handle {

            let ptr: *mut libc::c_void;
            unsafe {
                ptr = libc::malloc(::std::mem::size_of::<T>() as libc::size_t) as *mut libc::c_void;
                let plgptr = ptr as *mut T;
                (*plgptr).initialize()
            }
            return ptr;
        }

        extern "C" fn connect_port<'a, T: lv2::Plugin<'a>>(handle: lv2::LV2Handle,
                                                             port: u32,
                                                             data: *mut libc::c_void) {
            let d = data as *mut f32;
            let plgptr = handle as *mut T;
            unsafe {
                // TODO: This should be sample_count. How do we get that number? During initialization? Set to some random (high) number.
                // https://www.alsa-project.org/main/index.php/FramesPeriods
                let bs: &mut [f32] = ::std::slice::from_raw_parts_mut(d, 65536 * ::std::mem::size_of::<f32>());
                (*plgptr).connect_port(port, bs)
            }
        }

        extern "C" fn activate(_instance: lv2::LV2Handle) {}
        extern "C" fn run<'a, T: lv2::Plugin<'a>>(instance: lv2::LV2Handle, n_samples: u32) {
            let plgptr = instance as *mut T;
            unsafe { (*plgptr).run(n_samples) }
        }

        extern "C" fn deactivate(_instance: lv2::LV2Handle) {}
        extern "C" fn cleanup(instance: lv2::LV2Handle) {

            unsafe {
                // ptr::read(instance as *mut Amp); // no need for this?
                libc::free(instance as lv2::LV2Handle)
            }
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
