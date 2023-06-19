// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{Device, DeviceLink, DeviceLinkList, DeviceList, Entity, Filter, Os, Platform};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoDeployment")]
    pub struct Deployment(Object<ffi::OsinfoDeployment, ffi::OsinfoDeploymentClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_deployment_get_type(),
    }
}

impl Deployment {
    pub const NONE: Option<&'static Deployment> = None;

    #[doc(alias = "osinfo_deployment_new")]
    pub fn new(id: &str, os: &impl IsA<Os>, platform: &impl IsA<Platform>) -> Deployment {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::osinfo_deployment_new(
                id.to_glib_none().0,
                os.as_ref().to_glib_none().0,
                platform.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Deployment`] objects.
    ///
    /// This method returns an instance of [`DeploymentBuilder`](crate::builders::DeploymentBuilder) which can be used to create [`Deployment`] objects.
    pub fn builder() -> DeploymentBuilder {
        DeploymentBuilder::new()
    }
}

impl Default for Deployment {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Deployment`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeploymentBuilder {
    builder: glib::object::ObjectBuilder<'static, Deployment>,
}

impl DeploymentBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn os(self, os: &impl IsA<Os>) -> Self {
        Self {
            builder: self.builder.property("os", os.clone().upcast()),
        }
    }

    pub fn platform(self, platform: &impl IsA<Platform>) -> Self {
        Self {
            builder: self.builder.property("platform", platform.clone().upcast()),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Deployment`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Deployment {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Deployment>> Sealed for T {}
}

pub trait DeploymentExt: IsA<Deployment> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_deployment_add_device")]
    fn add_device(&self, dev: &impl IsA<Device>) -> DeviceLink {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_add_device(
                self.as_ref().to_glib_none().0,
                dev.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_device_links")]
    #[doc(alias = "get_device_links")]
    fn device_links(&self, filter: Option<&impl IsA<Filter>>) -> DeviceLinkList {
        unsafe {
            from_glib_full(ffi::osinfo_deployment_get_device_links(
                self.as_ref().to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self, filter: Option<&impl IsA<Filter>>) -> DeviceList {
        unsafe {
            from_glib_full(ffi::osinfo_deployment_get_devices(
                self.as_ref().to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_os")]
    #[doc(alias = "get_os")]
    fn os(&self) -> Os {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_os(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_platform")]
    #[doc(alias = "get_platform")]
    fn platform(&self) -> Platform {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_platform(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_preferred_device")]
    #[doc(alias = "get_preferred_device")]
    fn preferred_device(&self, filter: Option<&impl IsA<Filter>>) -> Device {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_preferred_device(
                self.as_ref().to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_deployment_get_preferred_device_link")]
    #[doc(alias = "get_preferred_device_link")]
    fn preferred_device_link(&self, filter: Option<&impl IsA<Filter>>) -> DeviceLink {
        unsafe {
            from_glib_none(ffi::osinfo_deployment_get_preferred_device_link(
                self.as_ref().to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<Deployment>> DeploymentExt for O {}

impl fmt::Display for Deployment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Deployment")
    }
}
