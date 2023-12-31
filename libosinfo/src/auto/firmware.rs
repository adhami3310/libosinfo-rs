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
    #[doc(alias = "OsinfoFirmware")]
    pub struct Firmware(Object<ffi::OsinfoFirmware, ffi::OsinfoFirmwareClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_firmware_get_type(),
    }
}

impl Firmware {
    pub const NONE: Option<&'static Firmware> = None;

    #[doc(alias = "osinfo_firmware_new")]
    pub fn new(id: &str, architecture: &str, type_: &str) -> Firmware {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::osinfo_firmware_new(
                id.to_glib_none().0,
                architecture.to_glib_none().0,
                type_.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Firmware`] objects.
    ///
    /// This method returns an instance of [`FirmwareBuilder`](crate::builders::FirmwareBuilder) which can be used to create [`Firmware`] objects.
    pub fn builder() -> FirmwareBuilder {
        FirmwareBuilder::new()
    }
}

impl Default for Firmware {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Firmware`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FirmwareBuilder {
    builder: glib::object::ObjectBuilder<'static, Firmware>,
}

impl FirmwareBuilder {
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

    pub fn type_(self, type_: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("type", type_.into()),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Firmware`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Firmware {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Firmware>> Sealed for T {}
}

pub trait FirmwareExt: IsA<Firmware> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "osinfo_firmware_get_architecture")]
    #[doc(alias = "get_architecture")]
    fn architecture(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_firmware_get_architecture(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "osinfo_firmware_get_firmware_type")]
    #[doc(alias = "get_firmware_type")]
    fn firmware_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_firmware_get_firmware_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
    #[doc(alias = "osinfo_firmware_is_supported")]
    fn is_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_firmware_is_supported(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_architecture(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "architecture")
    }

    fn set_architecture(&self, architecture: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "architecture", architecture)
    }

    #[doc(alias = "type")]
    fn type_(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "type")
    }

    #[doc(alias = "type")]
    fn set_type(&self, type_: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "type", type_)
    }

    #[doc(alias = "architecture")]
    fn connect_architecture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_architecture_trampoline<
            P: IsA<Firmware>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::OsinfoFirmware,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Firmware::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::architecture\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_architecture_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P: IsA<Firmware>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoFirmware,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Firmware::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Firmware>> FirmwareExt for O {}

impl fmt::Display for Firmware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Firmware")
    }
}
