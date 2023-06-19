// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../..
// from ../../gir-files
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT(OSINFO_ARCHITECTURE_ALL);
    PRINT_CONSTANT(OSINFO_AVATAR_FORMAT_PROP_ALPHA);
    PRINT_CONSTANT(OSINFO_AVATAR_FORMAT_PROP_HEIGHT);
    PRINT_CONSTANT(OSINFO_AVATAR_FORMAT_PROP_MIME_TYPE);
    PRINT_CONSTANT(OSINFO_AVATAR_FORMAT_PROP_WIDTH);
    PRINT_CONSTANT(OSINFO_DEVICELINK_PROP_DRIVER);
    PRINT_CONSTANT(OSINFO_DEVICELINK_PROP_SUPPORTED);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_DEFAULT_PRIORITY);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_DEVICE);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_FILE);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_LOCATION);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_PRE_INSTALLABLE);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_PRIORITY);
    PRINT_CONSTANT(OSINFO_DEVICE_DRIVER_PROP_SIGNED);
    PRINT_CONSTANT((gint) OSINFO_DEVICE_DRIVER_SIGNING_REQ_NONE);
    PRINT_CONSTANT((gint) OSINFO_DEVICE_DRIVER_SIGNING_REQ_STRICT);
    PRINT_CONSTANT((gint) OSINFO_DEVICE_DRIVER_SIGNING_REQ_WARN);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_BUS_TYPE);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_CLASS);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_NAME);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_PRODUCT);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_PRODUCT_ID);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_SUBSYSTEM);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_VENDOR);
    PRINT_CONSTANT(OSINFO_DEVICE_PROP_VENDOR_ID);
    PRINT_CONSTANT(OSINFO_ENTITY_PROP_ID);
    PRINT_CONSTANT((gint) OSINFO_ERROR_GENERIC);
    PRINT_CONSTANT(OSINFO_FIRMWARE_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_FIRMWARE_PROP_SUPPORTED);
    PRINT_CONSTANT(OSINFO_FIRMWARE_PROP_TYPE);
    PRINT_CONSTANT(OSINFO_GIBIBYTES);
    PRINT_CONSTANT(OSINFO_IMAGE_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_IMAGE_PROP_CLOUD_INIT);
    PRINT_CONSTANT(OSINFO_IMAGE_PROP_FORMAT);
    PRINT_CONSTANT(OSINFO_IMAGE_PROP_URL);
    PRINT_CONSTANT(OSINFO_IMAGE_PROP_VARIANT);
    PRINT_CONSTANT((gint) OSINFO_INSTALL_CONFIG_PARAM_POLICY_NONE);
    PRINT_CONSTANT((gint) OSINFO_INSTALL_CONFIG_PARAM_POLICY_OPTIONAL);
    PRINT_CONSTANT((gint) OSINFO_INSTALL_CONFIG_PARAM_POLICY_REQUIRED);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PARAM_PROP_DATAMAP);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PARAM_PROP_NAME);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PARAM_PROP_POLICY);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_ADMIN_PASSWORD);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_AVATAR_DISK);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_AVATAR_LOCATION);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_DRIVER_SIGNING);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_HARDWARE_ARCH);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_HOSTNAME);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_INSTALLATION_URL);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_L10N_KEYBOARD);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_L10N_LANGUAGE);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_L10N_TIMEZONE);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_POST_INSTALL_DRIVERS_DISK);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_POST_INSTALL_DRIVERS_LOCATION);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_PRE_INSTALL_DRIVERS_DISK);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_PRE_INSTALL_DRIVERS_LOCATION);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_REG_LOGIN);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_REG_PASSWORD);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_REG_PRODUCTKEY);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_SCRIPT_DISK);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_TARGET_DISK);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_USER_ADMIN);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_USER_AUTOLOGIN);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_USER_LOGIN);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_USER_PASSWORD);
    PRINT_CONSTANT(OSINFO_INSTALL_CONFIG_PROP_USER_REALNAME);
    PRINT_CONSTANT((guint) OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_CDROM);
    PRINT_CONSTANT((guint) OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_DISK);
    PRINT_CONSTANT((guint) OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_FLOPPY);
    PRINT_CONSTANT((guint) OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_INITRD);
    PRINT_CONSTANT((guint) OSINFO_INSTALL_SCRIPT_INJECTION_METHOD_WEB);
    PRINT_CONSTANT((gint) OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_MEDIA);
    PRINT_CONSTANT((gint) OSINFO_INSTALL_SCRIPT_INSTALLATION_SOURCE_NETWORK);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROFILE_DESKTOP);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROFILE_JEOS);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_CAN_POST_INSTALL_DRIVERS);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_CAN_PRE_INSTALL_DRIVERS);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_EXPECTED_FILENAME);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_INJECTION_METHOD);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_INSTALLATION_SOURCE);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_NEEDS_INTERNET);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_PATH_FORMAT);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_POST_INSTALL_DRIVERS_SIGNING_REQ);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_PREFERRED_INJECTION_METHOD);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_PRE_INSTALL_DRIVERS_SIGNING_REQ);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_PRODUCT_KEY_FORMAT);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_PROFILE);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_TEMPLATE_DATA);
    PRINT_CONSTANT(OSINFO_INSTALL_SCRIPT_PROP_TEMPLATE_URI);
    PRINT_CONSTANT(OSINFO_KIBIBYTES);
    PRINT_CONSTANT(OSINFO_MAJOR_VERSION);
    PRINT_CONSTANT(OSINFO_MEBIBYTES);
    PRINT_CONSTANT((guint) OSINFO_MEDIA_DETECT_REQUIRE_BOOTABLE);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_INSUFFICIENT_METADATA);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_NOT_BOOTABLE);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_NO_DESCRIPTORS);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_NO_DIRECTORY_RECORD_EXTENT);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_NO_PVD);
    PRINT_CONSTANT((gint) OSINFO_MEDIA_ERROR_NO_SVD);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_APPLICATION_ID);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_BOOTABLE);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_EJECT_AFTER_INSTALL);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_INITRD);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_INSTALLER);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_INSTALLER_REBOOTS);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_INSTALLER_SCRIPT);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_KERNEL);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_LANG);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_LANG_MAP);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_LANG_REGEX);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_LIVE);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_PUBLISHER_ID);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_SYSTEM_ID);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_URL);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_VARIANT);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_VOLUME_ID);
    PRINT_CONSTANT(OSINFO_MEDIA_PROP_VOLUME_SIZE);
    PRINT_CONSTANT(OSINFO_MEGAHERTZ);
    PRINT_CONSTANT(OSINFO_MICRO_VERSION);
    PRINT_CONSTANT(OSINFO_MINOR_VERSION);
    PRINT_CONSTANT(OSINFO_OS_PROP_CLOUD_IMAGE_USERNAME);
    PRINT_CONSTANT(OSINFO_OS_PROP_DISTRO);
    PRINT_CONSTANT(OSINFO_OS_PROP_FAMILY);
    PRINT_CONSTANT(OSINFO_OS_PROP_KERNEL_URL_ARGUMENT);
    PRINT_CONSTANT(OSINFO_OS_PROP_RELEASE_STATUS);
    PRINT_CONSTANT(OSINFO_OS_VARIANT_PROP_NAME);
    PRINT_CONSTANT((gint) OSINFO_PATH_FORMAT_DOS);
    PRINT_CONSTANT((gint) OSINFO_PATH_FORMAT_UNIX);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_CODENAME);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_EOL_DATE);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_LOGO);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_NAME);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_RELEASE_DATE);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_SHORT_ID);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_VENDOR);
    PRINT_CONSTANT(OSINFO_PRODUCT_PROP_VERSION);
    PRINT_CONSTANT((gint) OSINFO_PRODUCT_RELATIONSHIP_CLONES);
    PRINT_CONSTANT((gint) OSINFO_PRODUCT_RELATIONSHIP_DERIVES_FROM);
    PRINT_CONSTANT((gint) OSINFO_PRODUCT_RELATIONSHIP_UPGRADES);
    PRINT_CONSTANT((gint) OSINFO_RELEASE_STATUS_PRERELEASE);
    PRINT_CONSTANT((gint) OSINFO_RELEASE_STATUS_RELEASED);
    PRINT_CONSTANT((gint) OSINFO_RELEASE_STATUS_ROLLING);
    PRINT_CONSTANT((gint) OSINFO_RELEASE_STATUS_SNAPSHOT);
    PRINT_CONSTANT(OSINFO_RESOURCES_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_RESOURCES_PROP_CPU);
    PRINT_CONSTANT(OSINFO_RESOURCES_PROP_N_CPUS);
    PRINT_CONSTANT(OSINFO_RESOURCES_PROP_RAM);
    PRINT_CONSTANT(OSINFO_RESOURCES_PROP_STORAGE);
    PRINT_CONSTANT((gint) OSINFO_TREE_ERROR_NOT_SUPPORTED_PROTOCOL);
    PRINT_CONSTANT((gint) OSINFO_TREE_ERROR_NO_TREEINFO);
    PRINT_CONSTANT(OSINFO_TREE_PROP_ARCHITECTURE);
    PRINT_CONSTANT(OSINFO_TREE_PROP_BOOT_ISO);
    PRINT_CONSTANT(OSINFO_TREE_PROP_HAS_TREEINFO);
    PRINT_CONSTANT(OSINFO_TREE_PROP_INITRD);
    PRINT_CONSTANT(OSINFO_TREE_PROP_KERNEL);
    PRINT_CONSTANT(OSINFO_TREE_PROP_TREEINFO_ARCH);
    PRINT_CONSTANT(OSINFO_TREE_PROP_TREEINFO_FAMILY);
    PRINT_CONSTANT(OSINFO_TREE_PROP_TREEINFO_VARIANT);
    PRINT_CONSTANT(OSINFO_TREE_PROP_TREEINFO_VERSION);
    PRINT_CONSTANT(OSINFO_TREE_PROP_URL);
    PRINT_CONSTANT(OSINFO_TREE_PROP_VARIANT);
    return 0;
}
