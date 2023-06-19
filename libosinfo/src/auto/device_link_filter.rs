// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::Filter;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeviceLinkFilter")]
    pub struct DeviceLinkFilter(Object<ffi::OsinfoDeviceLinkFilter, ffi::OsinfoDeviceLinkFilterClass>) @extends Filter;

    match fn {
        type_ => || ffi::osinfo_devicelinkfilter_get_type(),
    }
}

impl DeviceLinkFilter {
    pub const NONE: Option<&'static DeviceLinkFilter> = None;

    #[doc(alias = "osinfo_devicelinkfilter_new")]
    pub fn new(filter: &impl IsA<Filter>) -> DeviceLinkFilter {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::osinfo_devicelinkfilter_new(
                filter.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeviceLinkFilter`] objects.
    ///
    /// This method returns an instance of [`DeviceLinkFilterBuilder`](crate::builders::DeviceLinkFilterBuilder) which can be used to create [`DeviceLinkFilter`] objects.
    pub fn builder() -> DeviceLinkFilterBuilder {
        DeviceLinkFilterBuilder::new()
    }
}

impl Default for DeviceLinkFilter {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeviceLinkFilter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeviceLinkFilterBuilder {
    builder: glib::object::ObjectBuilder<'static, DeviceLinkFilter>,
}

impl DeviceLinkFilterBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn target_filter(self, target_filter: &impl IsA<Filter>) -> Self {
        Self {
            builder: self
                .builder
                .property("target-filter", target_filter.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DeviceLinkFilter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DeviceLinkFilter {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DeviceLinkFilter>> Sealed for T {}
}

pub trait DeviceLinkFilterExt: IsA<DeviceLinkFilter> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_devicelinkfilter_get_target_filter")]
    #[doc(alias = "get_target_filter")]
    fn target_filter(&self) -> Filter {
        unsafe {
            from_glib_none(ffi::osinfo_devicelinkfilter_get_target_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<DeviceLinkFilter>> DeviceLinkFilterExt for O {}

impl fmt::Display for DeviceLinkFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceLinkFilter")
    }
}