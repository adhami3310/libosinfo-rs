// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct InstallScriptInjectionMethod: u32 {
        const CDROM = 1;
        const DISK = 2;
        const FLOPPY = 4;
        const INITRD = 8;
        const WEB = 16;
    }
}

#[doc(hidden)]
impl ToGlib for InstallScriptInjectionMethod {
    type GlibType = ffi::OsinfoInstallScriptInjectionMethod;

    fn to_glib(&self) -> ffi::OsinfoInstallScriptInjectionMethod {
        ffi::OsinfoInstallScriptInjectionMethod::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoInstallScriptInjectionMethod> for InstallScriptInjectionMethod {
    fn from_glib(value: ffi::OsinfoInstallScriptInjectionMethod) -> InstallScriptInjectionMethod {
        InstallScriptInjectionMethod::from_bits_truncate(value.bits())
    }
}

impl StaticType for InstallScriptInjectionMethod {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_install_script_injection_method_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InstallScriptInjectionMethod {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InstallScriptInjectionMethod {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::OsinfoInstallScriptInjectionMethod::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for InstallScriptInjectionMethod {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}
