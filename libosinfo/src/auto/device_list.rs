// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::List;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeviceList")]
    pub struct DeviceList(Object<ffi::OsinfoDeviceList, ffi::OsinfoDeviceListClass>) @extends List;

    match fn {
        type_ => || ffi::osinfo_devicelist_get_type(),
    }
}

impl DeviceList {
    pub const NONE: Option<&'static DeviceList> = None;

    #[doc(alias = "osinfo_devicelist_new")]
    pub fn new() -> DeviceList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_devicelist_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeviceList`] objects.
    ///
    /// This method returns an instance of [`DeviceListBuilder`](crate::builders::DeviceListBuilder) which can be used to create [`DeviceList`] objects.
    pub fn builder() -> DeviceListBuilder {
        DeviceListBuilder::new()
    }
}

impl Default for DeviceList {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeviceList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeviceListBuilder {
    builder: glib::object::ObjectBuilder<'static, DeviceList>,
}

impl DeviceListBuilder {
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
    /// Build the [`DeviceList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DeviceList {
        self.builder.build()
    }
}

impl fmt::Display for DeviceList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceList")
    }
}
