// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{Entity, Filter};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OsinfoList")]
    pub struct List(Object<ffi::OsinfoList, ffi::OsinfoListClass>);

    match fn {
        type_ => || ffi::osinfo_list_get_type(),
    }
}

impl List {
    pub const NONE: Option<&'static List> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::List>> Sealed for T {}
}

pub trait ListExt: IsA<List> + sealed::Sealed + 'static {
    #[doc(alias = "osinfo_list_add")]
    fn add(&self, entity: &impl IsA<Entity>) {
        unsafe {
            ffi::osinfo_list_add(
                self.as_ref().to_glib_none().0,
                entity.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_list_add_all")]
    fn add_all(&self, source: &impl IsA<List>) {
        unsafe {
            ffi::osinfo_list_add_all(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_list_add_filtered")]
    fn add_filtered(&self, source: &impl IsA<List>, filter: &impl IsA<Filter>) {
        unsafe {
            ffi::osinfo_list_add_filtered(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                filter.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_list_add_intersection")]
    fn add_intersection(&self, sourceOne: &impl IsA<List>, sourceTwo: &impl IsA<List>) {
        unsafe {
            ffi::osinfo_list_add_intersection(
                self.as_ref().to_glib_none().0,
                sourceOne.as_ref().to_glib_none().0,
                sourceTwo.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_list_add_union")]
    fn add_union(&self, sourceOne: &impl IsA<List>, sourceTwo: &impl IsA<List>) {
        unsafe {
            ffi::osinfo_list_add_union(
                self.as_ref().to_glib_none().0,
                sourceOne.as_ref().to_glib_none().0,
                sourceTwo.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "osinfo_list_find_by_id")]
    fn find_by_id(&self, id: &str) -> Entity {
        unsafe {
            from_glib_none(ffi::osinfo_list_find_by_id(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_list_get_element_type")]
    #[doc(alias = "get_element_type")]
    fn element_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::osinfo_list_get_element_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_list_get_elements")]
    #[doc(alias = "get_elements")]
    fn elements(&self) -> Vec<Entity> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::osinfo_list_get_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_list_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> i32 {
        unsafe { ffi::osinfo_list_get_length(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "osinfo_list_get_nth")]
    #[doc(alias = "get_nth")]
    fn nth(&self, idx: i32) -> Entity {
        unsafe {
            from_glib_none(ffi::osinfo_list_get_nth(
                self.as_ref().to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "osinfo_list_new_copy")]
    #[must_use]
    fn new_copy(&self) -> List {
        unsafe { from_glib_full(ffi::osinfo_list_new_copy(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "osinfo_list_new_filtered")]
    #[must_use]
    fn new_filtered(&self, filter: &impl IsA<Filter>) -> List {
        unsafe {
            from_glib_full(ffi::osinfo_list_new_filtered(
                self.as_ref().to_glib_none().0,
                filter.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_list_new_intersection")]
    #[must_use]
    fn new_intersection(&self, sourceTwo: &impl IsA<List>) -> List {
        unsafe {
            from_glib_full(ffi::osinfo_list_new_intersection(
                self.as_ref().to_glib_none().0,
                sourceTwo.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "osinfo_list_new_union")]
    #[must_use]
    fn new_union(&self, sourceTwo: &impl IsA<List>) -> List {
        unsafe {
            from_glib_full(ffi::osinfo_list_new_union(
                self.as_ref().to_glib_none().0,
                sourceTwo.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<List>> ListExt for O {}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("List")
    }
}