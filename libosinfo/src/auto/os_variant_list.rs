// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::List;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoOsVariantList")]
    pub struct OsVariantList(Object<ffi::OsinfoOsVariantList, ffi::OsinfoOsVariantListClass>) @extends List;

    match fn {
        type_ => || ffi::osinfo_os_variantlist_get_type(),
    }
}

impl OsVariantList {
    pub const NONE: Option<&'static OsVariantList> = None;

    #[doc(alias = "osinfo_os_variantlist_new")]
    pub fn new() -> OsVariantList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_os_variantlist_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`OsVariantList`] objects.
    ///
    /// This method returns an instance of [`OsVariantListBuilder`](crate::builders::OsVariantListBuilder) which can be used to create [`OsVariantList`] objects.
    pub fn builder() -> OsVariantListBuilder {
        OsVariantListBuilder::new()
    }
}

impl Default for OsVariantList {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`OsVariantList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct OsVariantListBuilder {
    builder: glib::object::ObjectBuilder<'static, OsVariantList>,
}

impl OsVariantListBuilder {
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
    /// Build the [`OsVariantList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> OsVariantList {
        self.builder.build()
    }
}

impl fmt::Display for OsVariantList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OsVariantList")
    }
}
