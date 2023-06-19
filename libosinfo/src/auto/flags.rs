// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use bitflags::bitflags;
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use std::fmt;

bitflags! {
    #[doc(alias = "OsinfoInstallScriptInjectionMethod")]
    pub struct InstallScriptInjectionMethod: u32 {
        #[doc(alias = "OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_CDROM")]
        const CDROM = ffi::OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_CDROM as _;
        #[doc(alias = "OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_DISK")]
        const DISK = ffi::OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_DISK as _;
        #[doc(alias = "OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_FLOPPY")]
        const FLOPPY = ffi::OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_FLOPPY as _;
        #[doc(alias = "OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_INITRD")]
        const INITRD = ffi::OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_INITRD as _;
        #[doc(alias = "OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_WEB")]
        const WEB = ffi::OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_WEB as _;
    }
}

impl fmt::Display for InstallScriptInjectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for InstallScriptInjectionMethod {
    type GlibType = ffi::OsinfoInstallScriptInjectionMethod;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoInstallScriptInjectionMethod {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoInstallScriptInjectionMethod> for InstallScriptInjectionMethod {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoInstallScriptInjectionMethod) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for InstallScriptInjectionMethod {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_install_script_injection_method_get_type()) }
    }
}

impl glib::HasParamSpec for InstallScriptInjectionMethod {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for InstallScriptInjectionMethod {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InstallScriptInjectionMethod {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for InstallScriptInjectionMethod {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InstallScriptInjectionMethod> for glib::Value {
    #[inline]
    fn from(v: InstallScriptInjectionMethod) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_6")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
    #[doc(alias = "OsinfoMediaDetectFlags")]
    pub struct MediaDetectFlags: u32 {
        #[doc(alias = "OSINFO_MEDIA_DETECT_REQUIRE_BOOTABLE")]
        const BOOTABLE = ffi::OSINFO_MEDIA_DETECT_REQUIRE_BOOTABLE as _;
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl fmt::Display for MediaDetectFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
#[doc(hidden)]
impl IntoGlib for MediaDetectFlags {
    type GlibType = ffi::OsinfoMediaDetectFlags;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoMediaDetectFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
#[doc(hidden)]
impl FromGlib<ffi::OsinfoMediaDetectFlags> for MediaDetectFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoMediaDetectFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl StaticType for MediaDetectFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_media_detect_flags_get_type()) }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl glib::HasParamSpec for MediaDetectFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl glib::value::ValueType for MediaDetectFlags {
    type Type = Self;
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
unsafe impl<'a> FromValue<'a> for MediaDetectFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl ToValue for MediaDetectFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl From<MediaDetectFlags> for glib::Value {
    #[inline]
    fn from(v: MediaDetectFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
