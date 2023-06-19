// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{List, ProductList};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoPlatformList")]
    pub struct PlatformList(Object<ffi::OsinfoPlatformList, ffi::OsinfoPlatformListClass>) @extends ProductList, List;

    match fn {
        type_ => || ffi::osinfo_platformlist_get_type(),
    }
}

impl PlatformList {
    pub const NONE: Option<&'static PlatformList> = None;

    #[doc(alias = "osinfo_platformlist_new")]
    pub fn new() -> PlatformList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_platformlist_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PlatformList`] objects.
    ///
    /// This method returns an instance of [`PlatformListBuilder`](crate::builders::PlatformListBuilder) which can be used to create [`PlatformList`] objects.
    pub fn builder() -> PlatformListBuilder {
        PlatformListBuilder::new()
    }
}

impl Default for PlatformList {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PlatformList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PlatformListBuilder {
    builder: glib::object::ObjectBuilder<'static, PlatformList>,
}

impl PlatformListBuilder {
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
    /// Build the [`PlatformList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PlatformList {
        self.builder.build()
    }
}

impl fmt::Display for PlatformList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PlatformList")
    }
}