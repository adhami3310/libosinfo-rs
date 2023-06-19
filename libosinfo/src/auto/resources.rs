// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::Entity;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "OsinfoResources")]
    pub struct Resources(Object<ffi::OsinfoResources, ffi::OsinfoResourcesClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_resources_get_type(),
    }
}

impl Resources {
    pub const NONE: Option<&'static Resources> = None;

    #[doc(alias = "osinfo_resources_new")]
    pub fn new(id: &str, architecture: &str) -> Resources {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::osinfo_resources_new(
                id.to_glib_none().0,
                architecture.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Resources`] objects.
    ///
    /// This method returns an instance of [`ResourcesBuilder`](crate::builders::ResourcesBuilder) which can be used to create [`Resources`] objects.
    pub fn builder() -> ResourcesBuilder {
        ResourcesBuilder::new()
    }
}

impl Default for Resources {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Resources`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ResourcesBuilder {
    builder: glib::object::ObjectBuilder<'static, Resources>,
}

impl ResourcesBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn architecture(self, architecture: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("architecture", architecture.into()),
        }
    }

    pub fn cpu(self, cpu: i64) -> Self {
        Self {
            builder: self.builder.property("cpu", cpu),
        }
    }

    pub fn n_cpus(self, n_cpus: i32) -> Self {
        Self {
            builder: self.builder.property("n-cpus", n_cpus),
        }
    }

    pub fn ram(self, ram: i64) -> Self {
        Self {
            builder: self.builder.property("ram", ram),
        }
    }

    pub fn storage(self, storage: i64) -> Self {
        Self {
            builder: self.builder.property("storage", storage),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Resources`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Resources {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Resources>> Sealed for T {}
}

pub trait ResourcesExt: IsA<Resources> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_resources_get_architecture")]
    #[doc(alias = "get_architecture")]
    fn architecture(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_resources_get_architecture(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_resources_get_cpu")]
    #[doc(alias = "get_cpu")]
    fn cpu(&self) -> i64 {
        unsafe { ffi::osinfo_resources_get_cpu(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_resources_get_n_cpus")]
    #[doc(alias = "get_n_cpus")]
    fn n_cpus(&self) -> i32 {
        unsafe { ffi::osinfo_resources_get_n_cpus(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_resources_get_ram")]
    #[doc(alias = "get_ram")]
    fn ram(&self) -> i64 {
        unsafe { ffi::osinfo_resources_get_ram(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_resources_get_storage")]
    #[doc(alias = "get_storage")]
    fn storage(&self) -> i64 {
        unsafe { ffi::osinfo_resources_get_storage(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_resources_set_cpu")]
    fn set_cpu(&self, cpu: i64) {
        unsafe {
            ffi::osinfo_resources_set_cpu(self.as_ref().to_glib_none().0, cpu);
        }
    }

    #[doc(alias = "osinfo_resources_set_n_cpus")]
    fn set_n_cpus(&self, n_cpus: i32) {
        unsafe {
            ffi::osinfo_resources_set_n_cpus(self.as_ref().to_glib_none().0, n_cpus);
        }
    }

    #[doc(alias = "osinfo_resources_set_ram")]
    fn set_ram(&self, ram: i64) {
        unsafe {
            ffi::osinfo_resources_set_ram(self.as_ref().to_glib_none().0, ram);
        }
    }

    #[doc(alias = "osinfo_resources_set_storage")]
    fn set_storage(&self, storage: i64) {
        unsafe {
            ffi::osinfo_resources_set_storage(self.as_ref().to_glib_none().0, storage);
        }
    }

    #[doc(alias = "cpu")]
    fn connect_cpu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cpu_trampoline<P: IsA<Resources>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoResources,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resources::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cpu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cpu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-cpus")]
    fn connect_n_cpus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_cpus_trampoline<P: IsA<Resources>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoResources,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resources::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-cpus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_cpus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ram")]
    fn connect_ram_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ram_trampoline<P: IsA<Resources>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoResources,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resources::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ram\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ram_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "storage")]
    fn connect_storage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_storage_trampoline<P: IsA<Resources>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoResources,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Resources::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::storage\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_storage_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Resources>> ResourcesExt for O {}

impl fmt::Display for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Resources")
    }
}