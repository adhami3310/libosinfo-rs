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
    #[doc(alias = "OsinfoOsVariant")]
    pub struct OsVariant(Object<ffi::OsinfoOsVariant, ffi::OsinfoOsVariantClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_os_variant_get_type(),
    }
}

impl OsVariant {
    pub const NONE: Option<&'static OsVariant> = None;

    #[doc(alias = "osinfo_os_variant_new")]
    pub fn new(id: &str) -> OsVariant {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_os_variant_new(id.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`OsVariant`] objects.
    ///
    /// This method returns an instance of [`OsVariantBuilder`](crate::builders::OsVariantBuilder) which can be used to create [`OsVariant`] objects.
    pub fn builder() -> OsVariantBuilder {
        OsVariantBuilder::new()
    }
}

impl Default for OsVariant {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

impl fmt::Display for OsVariant {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&OsVariantExt::name(self))
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`OsVariant`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct OsVariantBuilder {
    builder: glib::object::ObjectBuilder<'static, OsVariant>,
}

impl OsVariantBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`OsVariant`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> OsVariant {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::OsVariant>> Sealed for T {}
}

pub trait OsVariantExt: IsA<OsVariant> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_os_variant_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_os_variant_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_name(&self, name: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "name", name)
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<OsVariant>, F: Fn(&P) + 'static>(
            this: *mut ffi::OsinfoOsVariant,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(OsVariant::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<OsVariant>> OsVariantExt for O {}
