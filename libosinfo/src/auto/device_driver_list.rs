// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::List;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeviceDriverList")]
    pub struct DeviceDriverList(Object<ffi::OsinfoDeviceDriverList, ffi::OsinfoDeviceDriverListClass>) @extends List;

    match fn {
        type_ => || ffi::osinfo_device_driverlist_get_type(),
    }
}

impl DeviceDriverList {
    pub const NONE: Option<&'static DeviceDriverList> = None;

    #[doc(alias = "osinfo_device_driverlist_new")]
    pub fn new() -> DeviceDriverList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_device_driverlist_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeviceDriverList`] objects.
    ///
    /// This method returns an instance of [`DeviceDriverListBuilder`](crate::builders::DeviceDriverListBuilder) which can be used to create [`DeviceDriverList`] objects.
    pub fn builder() -> DeviceDriverListBuilder {
        DeviceDriverListBuilder::new()
    }
}

impl Default for DeviceDriverList {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeviceDriverList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeviceDriverListBuilder {
    builder: glib::object::ObjectBuilder<'static, DeviceDriverList>,
}

impl DeviceDriverListBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn element_type(self, element_type: glib::types::Type) -> Self {
        Self {
            builder: self.builder.property("element-type", element_type),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DeviceDriverList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DeviceDriverList {
        self.builder.build()
    }
}

impl fmt::Display for DeviceDriverList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceDriverList")
    }
}