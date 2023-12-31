// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::{
    error::ErrorDomain, translate::*, value::FromValue, value::ToValue, Quark, StaticType, Type,
};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoDeviceDriverSigningReq")]
pub enum DeviceDriverSigningReq {
    #[doc(alias = "OSINFO_DEVICE_DRIVER_SIGNING_REQ_NONE")]
    None,
    #[doc(alias = "OSINFO_DEVICE_DRIVER_SIGNING_REQ_STRICT")]
    Strict,
    #[doc(alias = "OSINFO_DEVICE_DRIVER_SIGNING_REQ_WARN")]
    Warn,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DeviceDriverSigningReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DeviceDriverSigningReq::{}",
            match *self {
                Self::None => "None",
                Self::Strict => "Strict",
                Self::Warn => "Warn",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DeviceDriverSigningReq {
    type GlibType = ffi::OsinfoDeviceDriverSigningReq;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoDeviceDriverSigningReq {
        match self {
            Self::None => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_NONE,
            Self::Strict => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_STRICT,
            Self::Warn => ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_WARN,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoDeviceDriverSigningReq> for DeviceDriverSigningReq {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoDeviceDriverSigningReq) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_NONE => Self::None,
            ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_STRICT => Self::Strict,
            ffi::OSINFO_DEVICE_DRIVER_SIGNING_REQ_WARN => Self::Warn,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for DeviceDriverSigningReq {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_device_driver_signing_req_get_type()) }
    }
}

impl glib::HasParamSpec for DeviceDriverSigningReq {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for DeviceDriverSigningReq {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DeviceDriverSigningReq {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for DeviceDriverSigningReq {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<DeviceDriverSigningReq> for glib::Value {
    #[inline]
    fn from(v: DeviceDriverSigningReq) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoError")]
pub enum Error {
    #[doc(alias = "OSINFO_ERROR_GENERIC")]
    Generic,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl Error {
    #[doc(alias = "osinfo_error_quark")]
    pub fn quark() -> glib::Quark {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::osinfo_error_quark()) }
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error::{}",
            match *self {
                Self::Generic => "Generic",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
#[doc(hidden)]
impl IntoGlib for Error {
    type GlibType = ffi::OsinfoError;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoError {
        match self {
            Self::Generic => ffi::OSINFO_ERROR_GENERIC,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
#[doc(hidden)]
impl FromGlib<ffi::OsinfoError> for Error {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoError) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_ERROR_GENERIC => Self::Generic,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl ErrorDomain for Error {
    #[inline]
    fn domain() -> Quark {
        skip_assert_initialized!();

        static QUARK: once_cell::sync::Lazy<glib::ffi::GQuark> =
            once_cell::sync::Lazy::new(|| unsafe {
                glib::ffi::g_quark_from_static_string(b"libosinfo\0".as_ptr() as *const _)
            });
        unsafe { from_glib(*QUARK) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match unsafe { from_glib(code) } {
            value => Some(value),
        }
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl StaticType for Error {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_error_get_type()) }
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl glib::HasParamSpec for Error {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl glib::value::ValueType for Error {
    type Type = Self;
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
unsafe impl<'a> FromValue<'a> for Error {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl ToValue for Error {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_8")))]
impl From<Error> for glib::Value {
    #[inline]
    fn from(v: Error) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoInstallConfigParamPolicy")]
pub enum InstallConfigParamPolicy {
    #[doc(alias = "OSINFO_INSTALL_CONFIG_PARAM_POLICY_NONE")]
    None,
    #[doc(alias = "OSINFO_INSTALL_CONFIG_PARAM_POLICY_REQUIRED")]
    Required,
    #[doc(alias = "OSINFO_INSTALL_CONFIG_PARAM_POLICY_OPTIONAL")]
    Optional,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InstallConfigParamPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InstallConfigParamPolicy::{}",
            match *self {
                Self::None => "None",
                Self::Required => "Required",
                Self::Optional => "Optional",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for InstallConfigParamPolicy {
    type GlibType = ffi::OsinfoInstallConfigParamPolicy;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoInstallConfigParamPolicy {
        match self {
            Self::None => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_NONE,
            Self::Required => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_REQUIRED,
            Self::Optional => ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_OPTIONAL,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoInstallConfigParamPolicy> for InstallConfigParamPolicy {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoInstallConfigParamPolicy) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_NONE => Self::None,
            ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_REQUIRED => Self::Required,
            ffi::OSINFO_INSTALL_CONFIG_PARAM_POLICY_OPTIONAL => Self::Optional,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for InstallConfigParamPolicy {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_install_config_param_policy_get_type()) }
    }
}

impl glib::HasParamSpec for InstallConfigParamPolicy {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for InstallConfigParamPolicy {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InstallConfigParamPolicy {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for InstallConfigParamPolicy {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InstallConfigParamPolicy> for glib::Value {
    #[inline]
    fn from(v: InstallConfigParamPolicy) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoInstallScriptInstallationSource")]
pub enum InstallScriptInstallationSource {
    #[doc(alias = "OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_MEDIA")]
    Media,
    #[doc(alias = "OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_NETWORK")]
    Network,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl fmt::Display for InstallScriptInstallationSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InstallScriptInstallationSource::{}",
            match *self {
                Self::Media => "Media",
                Self::Network => "Network",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
#[doc(hidden)]
impl IntoGlib for InstallScriptInstallationSource {
    type GlibType = ffi::OsinfoInstallScriptInstallationSource;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoInstallScriptInstallationSource {
        match self {
            Self::Media => ffi::OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_MEDIA,
            Self::Network => ffi::OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_NETWORK,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
#[doc(hidden)]
impl FromGlib<ffi::OsinfoInstallScriptInstallationSource> for InstallScriptInstallationSource {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoInstallScriptInstallationSource) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_MEDIA => Self::Media,
            ffi::OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_NETWORK => Self::Network,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl StaticType for InstallScriptInstallationSource {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_install_script_installation_source_get_type()) }
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl glib::HasParamSpec for InstallScriptInstallationSource {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl glib::value::ValueType for InstallScriptInstallationSource {
    type Type = Self;
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
unsafe impl<'a> FromValue<'a> for InstallScriptInstallationSource {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl ToValue for InstallScriptInstallationSource {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_3")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_3")))]
impl From<InstallScriptInstallationSource> for glib::Value {
    #[inline]
    fn from(v: InstallScriptInstallationSource) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoMediaError")]
pub enum MediaError {
    #[doc(alias = "OSINFO_MEDIA_ERROR_NO_DESCRIPTORS")]
    NoDescriptors,
    #[doc(alias = "OSINFO_MEDIA_ERROR_NO_PVD")]
    NoPvd,
    #[doc(alias = "OSINFO_MEDIA_ERROR_NO_SVD")]
    NoSvd,
    #[doc(alias = "OSINFO_MEDIA_ERROR_INSUFFICIENT_METADATA")]
    InsufficientMetadata,
    #[doc(alias = "OSINFO_MEDIA_ERROR_NOT_BOOTABLE")]
    NotBootable,
    #[doc(alias = "OSINFO_MEDIA_ERROR_NO_DIRECTORY_RECORD_EXTENT")]
    NoDirectoryRecordExtent,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MediaError::{}",
            match *self {
                Self::NoDescriptors => "NoDescriptors",
                Self::NoPvd => "NoPvd",
                Self::NoSvd => "NoSvd",
                Self::InsufficientMetadata => "InsufficientMetadata",
                Self::NotBootable => "NotBootable",
                Self::NoDirectoryRecordExtent => "NoDirectoryRecordExtent",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for MediaError {
    type GlibType = ffi::OsinfoMediaError;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoMediaError {
        match self {
            Self::NoDescriptors => ffi::OSINFO_MEDIA_ERROR_NO_DESCRIPTORS,
            Self::NoPvd => ffi::OSINFO_MEDIA_ERROR_NO_PVD,
            Self::NoSvd => ffi::OSINFO_MEDIA_ERROR_NO_SVD,
            Self::InsufficientMetadata => ffi::OSINFO_MEDIA_ERROR_INSUFFICIENT_METADATA,
            Self::NotBootable => ffi::OSINFO_MEDIA_ERROR_NOT_BOOTABLE,
            Self::NoDirectoryRecordExtent => ffi::OSINFO_MEDIA_ERROR_NO_DIRECTORY_RECORD_EXTENT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoMediaError> for MediaError {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoMediaError) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_MEDIA_ERROR_NO_DESCRIPTORS => Self::NoDescriptors,
            ffi::OSINFO_MEDIA_ERROR_NO_PVD => Self::NoPvd,
            ffi::OSINFO_MEDIA_ERROR_NO_SVD => Self::NoSvd,
            ffi::OSINFO_MEDIA_ERROR_INSUFFICIENT_METADATA => Self::InsufficientMetadata,
            ffi::OSINFO_MEDIA_ERROR_NOT_BOOTABLE => Self::NotBootable,
            ffi::OSINFO_MEDIA_ERROR_NO_DIRECTORY_RECORD_EXTENT => Self::NoDirectoryRecordExtent,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for MediaError {
    #[inline]
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::osinfo_media_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match unsafe { from_glib(code) } {
            value => Some(value),
        }
    }
}

impl StaticType for MediaError {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_media_error_get_type()) }
    }
}

impl glib::HasParamSpec for MediaError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for MediaError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for MediaError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for MediaError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<MediaError> for glib::Value {
    #[inline]
    fn from(v: MediaError) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoPathFormat")]
pub enum PathFormat {
    #[doc(alias = "OSINFO_PATH_FORMAT_UNIX")]
    Unix,
    #[doc(alias = "OSINFO_PATH_FORMAT_DOS")]
    Dos,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PathFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PathFormat::{}",
            match *self {
                Self::Unix => "Unix",
                Self::Dos => "Dos",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for PathFormat {
    type GlibType = ffi::OsinfoPathFormat;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoPathFormat {
        match self {
            Self::Unix => ffi::OSINFO_PATH_FORMAT_UNIX,
            Self::Dos => ffi::OSINFO_PATH_FORMAT_DOS,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoPathFormat> for PathFormat {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoPathFormat) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_PATH_FORMAT_UNIX => Self::Unix,
            ffi::OSINFO_PATH_FORMAT_DOS => Self::Dos,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PathFormat {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_path_format_get_type()) }
    }
}

impl glib::HasParamSpec for PathFormat {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for PathFormat {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PathFormat {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PathFormat {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PathFormat> for glib::Value {
    #[inline]
    fn from(v: PathFormat) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoProductRelationship")]
pub enum ProductRelationship {
    #[doc(alias = "OSINFO_PRODUCT_RELATIONSHIP_DERIVES_FROM")]
    DerivesFrom,
    #[doc(alias = "OSINFO_PRODUCT_RELATIONSHIP_UPGRADES")]
    Upgrades,
    #[doc(alias = "OSINFO_PRODUCT_RELATIONSHIP_CLONES")]
    Clones,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ProductRelationship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProductRelationship::{}",
            match *self {
                Self::DerivesFrom => "DerivesFrom",
                Self::Upgrades => "Upgrades",
                Self::Clones => "Clones",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ProductRelationship {
    type GlibType = ffi::OsinfoProductRelationship;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoProductRelationship {
        match self {
            Self::DerivesFrom => ffi::OSINFO_PRODUCT_RELATIONSHIP_DERIVES_FROM,
            Self::Upgrades => ffi::OSINFO_PRODUCT_RELATIONSHIP_UPGRADES,
            Self::Clones => ffi::OSINFO_PRODUCT_RELATIONSHIP_CLONES,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoProductRelationship> for ProductRelationship {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoProductRelationship) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_PRODUCT_RELATIONSHIP_DERIVES_FROM => Self::DerivesFrom,
            ffi::OSINFO_PRODUCT_RELATIONSHIP_UPGRADES => Self::Upgrades,
            ffi::OSINFO_PRODUCT_RELATIONSHIP_CLONES => Self::Clones,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ProductRelationship {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_product_relationship_get_type()) }
    }
}

impl glib::HasParamSpec for ProductRelationship {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for ProductRelationship {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ProductRelationship {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ProductRelationship {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ProductRelationship> for glib::Value {
    #[inline]
    fn from(v: ProductRelationship) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoReleaseStatus")]
pub enum ReleaseStatus {
    #[doc(alias = "OSINFO_RELEASE_STATUS_RELEASED")]
    Released,
    #[doc(alias = "OSINFO_RELEASE_STATUS_SNAPSHOT")]
    Snapshot,
    #[doc(alias = "OSINFO_RELEASE_STATUS_PRERELEASE")]
    Prerelease,
    #[doc(alias = "OSINFO_RELEASE_STATUS_ROLLING")]
    Rolling,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ReleaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ReleaseStatus::{}",
            match *self {
                Self::Released => "Released",
                Self::Snapshot => "Snapshot",
                Self::Prerelease => "Prerelease",
                Self::Rolling => "Rolling",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ReleaseStatus {
    type GlibType = ffi::OsinfoReleaseStatus;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoReleaseStatus {
        match self {
            Self::Released => ffi::OSINFO_RELEASE_STATUS_RELEASED,
            Self::Snapshot => ffi::OSINFO_RELEASE_STATUS_SNAPSHOT,
            Self::Prerelease => ffi::OSINFO_RELEASE_STATUS_PRERELEASE,
            Self::Rolling => ffi::OSINFO_RELEASE_STATUS_ROLLING,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::OsinfoReleaseStatus> for ReleaseStatus {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoReleaseStatus) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_RELEASE_STATUS_RELEASED => Self::Released,
            ffi::OSINFO_RELEASE_STATUS_SNAPSHOT => Self::Snapshot,
            ffi::OSINFO_RELEASE_STATUS_PRERELEASE => Self::Prerelease,
            ffi::OSINFO_RELEASE_STATUS_ROLLING => Self::Rolling,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ReleaseStatus {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_release_status_get_type()) }
    }
}

impl glib::HasParamSpec for ReleaseStatus {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for ReleaseStatus {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ReleaseStatus {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ReleaseStatus {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ReleaseStatus> for glib::Value {
    #[inline]
    fn from(v: ReleaseStatus) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "OsinfoTreeError")]
pub enum TreeError {
    #[doc(alias = "OSINFO_TREE_ERROR_NO_TREEINFO")]
    NoTreeinfo,
    #[doc(alias = "OSINFO_TREE_ERROR_NOT_SUPPORTED_PROTOCOL")]
    NotSupportedProtocol,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl fmt::Display for TreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TreeError::{}",
            match *self {
                Self::NoTreeinfo => "NoTreeinfo",
                Self::NotSupportedProtocol => "NotSupportedProtocol",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
#[doc(hidden)]
impl IntoGlib for TreeError {
    type GlibType = ffi::OsinfoTreeError;

    #[inline]
    fn into_glib(self) -> ffi::OsinfoTreeError {
        match self {
            Self::NoTreeinfo => ffi::OSINFO_TREE_ERROR_NO_TREEINFO,
            Self::NotSupportedProtocol => ffi::OSINFO_TREE_ERROR_NOT_SUPPORTED_PROTOCOL,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
#[doc(hidden)]
impl FromGlib<ffi::OsinfoTreeError> for TreeError {
    #[inline]
    unsafe fn from_glib(value: ffi::OsinfoTreeError) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::OSINFO_TREE_ERROR_NO_TREEINFO => Self::NoTreeinfo,
            ffi::OSINFO_TREE_ERROR_NOT_SUPPORTED_PROTOCOL => Self::NotSupportedProtocol,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl ErrorDomain for TreeError {
    #[inline]
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::osinfo_tree_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match unsafe { from_glib(code) } {
            value => Some(value),
        }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl StaticType for TreeError {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::osinfo_tree_error_get_type()) }
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl glib::HasParamSpec for TreeError {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl glib::value::ValueType for TreeError {
    type Type = Self;
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
unsafe impl<'a> FromValue<'a> for TreeError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_6")))]
impl ToValue for TreeError {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
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
impl From<TreeError> for glib::Value {
    #[inline]
    fn from(v: TreeError) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
