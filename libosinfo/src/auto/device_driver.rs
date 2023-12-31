// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{DeviceList, Entity};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeviceDriver")]
    pub struct DeviceDriver(Object<ffi::OsinfoDeviceDriver, ffi::OsinfoDeviceDriverClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_device_driver_get_type(),
    }
}

impl DeviceDriver {
    pub const NONE: Option<&'static DeviceDriver> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeviceDriver`] objects.
    ///
    /// This method returns an instance of [`DeviceDriverBuilder`](crate::builders::DeviceDriverBuilder) which can be used to create [`DeviceDriver`] objects.
    pub fn builder() -> DeviceDriverBuilder {
        DeviceDriverBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeviceDriver`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeviceDriverBuilder {
    builder: glib::object::ObjectBuilder<'static, DeviceDriver>,
}

impl DeviceDriverBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DeviceDriver`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DeviceDriver {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DeviceDriver>> Sealed for T {}
}

pub trait DeviceDriverExt: IsA<DeviceDriver> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_device_driver_get_architecture")]
    #[doc(alias = "get_architecture")]
    fn architecture(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_architecture(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_device_driver_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self) -> DeviceList {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_device_driver_get_files")]
    #[doc(alias = "get_files")]
    fn files(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_device_driver_get_files(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_device_driver_get_location")]
    #[doc(alias = "get_location")]
    fn location(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_device_driver_get_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_device_driver_get_pre_installable")]
    #[doc(alias = "get_pre_installable")]
    fn is_pre_installable(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_device_driver_get_pre_installable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_7")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_7")))]
    #[doc(alias = "osinfo_device_driver_get_priority")]
    #[doc(alias = "get_priority")]
    fn priority(&self) -> i64 {
        unsafe { ffi::osinfo_device_driver_get_priority(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_device_driver_get_signed")]
    #[doc(alias = "get_signed")]
    fn is_signed(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_device_driver_get_signed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<DeviceDriver>> DeviceDriverExt for O {}

impl fmt::Display for DeviceDriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceDriver")
    }
}
