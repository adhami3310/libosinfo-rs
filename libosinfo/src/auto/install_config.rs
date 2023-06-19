// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::Entity;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoInstallConfig")]
    pub struct InstallConfig(Object<ffi::OsinfoInstallConfig, ffi::OsinfoInstallConfigClass>) @extends Entity;

    match fn {
        type_ => || ffi::osinfo_install_config_get_type(),
    }
}

impl InstallConfig {
    pub const NONE: Option<&'static InstallConfig> = None;

    #[doc(alias = "osinfo_install_config_new")]
    pub fn new(id: &str) -> InstallConfig {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::osinfo_install_config_new(id.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`InstallConfig`] objects.
    ///
    /// This method returns an instance of [`InstallConfigBuilder`](crate::builders::InstallConfigBuilder) which can be used to create [`InstallConfig`] objects.
    pub fn builder() -> InstallConfigBuilder {
        InstallConfigBuilder::new()
    }
}

impl Default for InstallConfig {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`InstallConfig`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct InstallConfigBuilder {
    builder: glib::object::ObjectBuilder<'static, InstallConfig>,
}

impl InstallConfigBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`InstallConfig`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> InstallConfig {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::InstallConfig>> Sealed for T {}
}

pub trait InstallConfigExt: IsA<InstallConfig> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_install_config_get_admin_password")]
    #[doc(alias = "get_admin_password")]
    fn admin_password(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_admin_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_avatar_disk")]
    #[doc(alias = "get_avatar_disk")]
    fn avatar_disk(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_avatar_disk(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_avatar_location")]
    #[doc(alias = "get_avatar_location")]
    fn avatar_location(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_avatar_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_driver_signing")]
    #[doc(alias = "get_driver_signing")]
    fn is_driver_signing(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_config_get_driver_signing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_hardware_arch")]
    #[doc(alias = "get_hardware_arch")]
    fn hardware_arch(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_hardware_arch(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_hostname")]
    #[doc(alias = "get_hostname")]
    fn hostname(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_hostname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "osinfo_install_config_get_installation_url")]
    #[doc(alias = "get_installation_url")]
    fn installation_url(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_installation_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_l10n_keyboard")]
    #[doc(alias = "get_l10n_keyboard")]
    fn l10n_keyboard(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_l10n_keyboard(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_l10n_language")]
    #[doc(alias = "get_l10n_language")]
    fn l10n_language(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_l10n_language(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_l10n_timezone")]
    #[doc(alias = "get_l10n_timezone")]
    fn l10n_timezone(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_l10n_timezone(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_post_install_drivers_disk")]
    #[doc(alias = "get_post_install_drivers_disk")]
    fn post_install_drivers_disk(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_post_install_drivers_disk(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_post_install_drivers_location")]
    #[doc(alias = "get_post_install_drivers_location")]
    fn post_install_drivers_location(&self) -> glib::GString {
        unsafe {
            from_glib_none(
                ffi::osinfo_install_config_get_post_install_drivers_location(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "osinfo_install_config_get_pre_install_drivers_disk")]
    #[doc(alias = "get_pre_install_drivers_disk")]
    fn pre_install_drivers_disk(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_pre_install_drivers_disk(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_pre_install_drivers_location")]
    #[doc(alias = "get_pre_install_drivers_location")]
    fn pre_install_drivers_location(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_pre_install_drivers_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_reg_login")]
    #[doc(alias = "get_reg_login")]
    fn reg_login(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_reg_login(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_reg_password")]
    #[doc(alias = "get_reg_password")]
    fn reg_password(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_reg_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_reg_product_key")]
    #[doc(alias = "get_reg_product_key")]
    fn reg_product_key(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_reg_product_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_script_disk")]
    #[doc(alias = "get_script_disk")]
    fn script_disk(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_script_disk(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_target_disk")]
    #[doc(alias = "get_target_disk")]
    fn target_disk(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_target_disk(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_user_administrator")]
    #[doc(alias = "get_user_administrator")]
    fn is_user_administrator(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_config_get_user_administrator(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_user_autologin")]
    #[doc(alias = "get_user_autologin")]
    fn is_user_autologin(&self) -> bool {
        unsafe {
            from_glib(ffi::osinfo_install_config_get_user_autologin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_user_login")]
    #[doc(alias = "get_user_login")]
    fn user_login(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_user_login(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_user_password")]
    #[doc(alias = "get_user_password")]
    fn user_password(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_user_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_get_user_realname")]
    #[doc(alias = "get_user_realname")]
    fn user_realname(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::osinfo_install_config_get_user_realname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_install_config_set_admin_password")]
    fn set_admin_password(&self, password: &str) {
        unsafe {
            ffi::osinfo_install_config_set_admin_password(
                self.as_ref().to_glib_none().0,
                password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_avatar_disk")]
    fn set_avatar_disk(&self, disk: &str) {
        unsafe {
            ffi::osinfo_install_config_set_avatar_disk(
                self.as_ref().to_glib_none().0,
                disk.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_avatar_location")]
    fn set_avatar_location(&self, location: &str) {
        unsafe {
            ffi::osinfo_install_config_set_avatar_location(
                self.as_ref().to_glib_none().0,
                location.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_driver_signing")]
    fn set_driver_signing(&self, signing: bool) {
        unsafe {
            ffi::osinfo_install_config_set_driver_signing(
                self.as_ref().to_glib_none().0,
                signing.into_glib(),
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_hardware_arch")]
    fn set_hardware_arch(&self, arch: &str) {
        unsafe {
            ffi::osinfo_install_config_set_hardware_arch(
                self.as_ref().to_glib_none().0,
                arch.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_hostname")]
    fn set_hostname(&self, hostname: &str) {
        unsafe {
            ffi::osinfo_install_config_set_hostname(
                self.as_ref().to_glib_none().0,
                hostname.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_5")))]
    #[doc(alias = "osinfo_install_config_set_installation_url")]
    fn set_installation_url(&self, url: &str) {
        unsafe {
            ffi::osinfo_install_config_set_installation_url(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_l10n_keyboard")]
    fn set_l10n_keyboard(&self, keyboard: &str) {
        unsafe {
            ffi::osinfo_install_config_set_l10n_keyboard(
                self.as_ref().to_glib_none().0,
                keyboard.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_l10n_language")]
    fn set_l10n_language(&self, language: &str) {
        unsafe {
            ffi::osinfo_install_config_set_l10n_language(
                self.as_ref().to_glib_none().0,
                language.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_l10n_timezone")]
    fn set_l10n_timezone(&self, tz: &str) {
        unsafe {
            ffi::osinfo_install_config_set_l10n_timezone(
                self.as_ref().to_glib_none().0,
                tz.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_post_install_drivers_disk")]
    fn set_post_install_drivers_disk(&self, disk: &str) {
        unsafe {
            ffi::osinfo_install_config_set_post_install_drivers_disk(
                self.as_ref().to_glib_none().0,
                disk.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_post_install_drivers_location")]
    fn set_post_install_drivers_location(&self, location: &str) {
        unsafe {
            ffi::osinfo_install_config_set_post_install_drivers_location(
                self.as_ref().to_glib_none().0,
                location.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_pre_install_drivers_disk")]
    fn set_pre_install_drivers_disk(&self, disk: &str) {
        unsafe {
            ffi::osinfo_install_config_set_pre_install_drivers_disk(
                self.as_ref().to_glib_none().0,
                disk.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_pre_install_drivers_location")]
    fn set_pre_install_drivers_location(&self, location: &str) {
        unsafe {
            ffi::osinfo_install_config_set_pre_install_drivers_location(
                self.as_ref().to_glib_none().0,
                location.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_reg_login")]
    fn set_reg_login(&self, name: &str) {
        unsafe {
            ffi::osinfo_install_config_set_reg_login(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_reg_password")]
    fn set_reg_password(&self, password: &str) {
        unsafe {
            ffi::osinfo_install_config_set_reg_password(
                self.as_ref().to_glib_none().0,
                password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_reg_product_key")]
    fn set_reg_product_key(&self, key: &str) {
        unsafe {
            ffi::osinfo_install_config_set_reg_product_key(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_script_disk")]
    fn set_script_disk(&self, disk: &str) {
        unsafe {
            ffi::osinfo_install_config_set_script_disk(
                self.as_ref().to_glib_none().0,
                disk.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_target_disk")]
    fn set_target_disk(&self, disk: &str) {
        unsafe {
            ffi::osinfo_install_config_set_target_disk(
                self.as_ref().to_glib_none().0,
                disk.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_user_administrator")]
    fn set_user_administrator(&self, admin: bool) {
        unsafe {
            ffi::osinfo_install_config_set_user_administrator(
                self.as_ref().to_glib_none().0,
                admin.into_glib(),
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_user_autologin")]
    fn set_user_autologin(&self, autologin: bool) {
        unsafe {
            ffi::osinfo_install_config_set_user_autologin(
                self.as_ref().to_glib_none().0,
                autologin.into_glib(),
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_user_login")]
    fn set_user_login(&self, username: &str) {
        unsafe {
            ffi::osinfo_install_config_set_user_login(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_user_password")]
    fn set_user_password(&self, password: &str) {
        unsafe {
            ffi::osinfo_install_config_set_user_password(
                self.as_ref().to_glib_none().0,
                password.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_install_config_set_user_realname")]
    fn set_user_realname(&self, name: &str) {
        unsafe {
            ffi::osinfo_install_config_set_user_realname(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<InstallConfig>> InstallConfigExt for O {}

impl fmt::Display for InstallConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InstallConfig")
    }
}
