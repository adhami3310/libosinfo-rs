// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::List;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeploymentList")]
    pub struct DeploymentList(Object<ffi::OsinfoDeploymentList, ffi::OsinfoDeploymentListClass>) @extends List;

    match fn {
        type_ => || ffi::osinfo_deploymentlist_get_type(),
    }
}

impl DeploymentList {
    pub const NONE: Option<&'static DeploymentList> = None;

    #[doc(alias = "osinfo_deploymentlist_new")]
    pub fn new() -> DeploymentList {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_deploymentlist_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DeploymentList`] objects.
    ///
    /// This method returns an instance of [`DeploymentListBuilder`](crate::builders::DeploymentListBuilder) which can be used to create [`DeploymentList`] objects.
    pub fn builder() -> DeploymentListBuilder {
        DeploymentListBuilder::new()
    }
}

impl Default for DeploymentList {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DeploymentList`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeploymentListBuilder {
    builder: glib::object::ObjectBuilder<'static, DeploymentList>,
}

impl DeploymentListBuilder {
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
    /// Build the [`DeploymentList`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DeploymentList {
        self.builder.build()
    }
}

impl fmt::Display for DeploymentList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeploymentList")
    }
}
