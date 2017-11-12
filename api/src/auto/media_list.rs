// This file was generated by gir (0409d73) from gir-files (???)
// DO NOT EDIT

use Filter;
use List;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MediaList(Object<ffi::OsinfoMediaList, ffi::OsinfoMediaListClass>): List;

    match fn {
        get_type => || ffi::osinfo_medialist_get_type(),
    }
}

impl MediaList {
    pub fn new() -> MediaList {
        unsafe {
            from_glib_full(ffi::osinfo_medialist_new())
        }
    }
}

impl Default for MediaList {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MediaListExt {
    fn new_copy(&self) -> Option<MediaList>;

    fn new_filtered<P: IsA<Filter>>(&self, filter: &P) -> Option<MediaList>;

    fn new_intersection(&self, sourceTwo: &MediaList) -> Option<MediaList>;

    fn new_union(&self, sourceTwo: &MediaList) -> Option<MediaList>;
}

impl<O: IsA<MediaList>> MediaListExt for O {
    fn new_copy(&self) -> Option<MediaList> {
        unsafe {
            from_glib_full(ffi::osinfo_medialist_new_copy(self.to_glib_none().0))
        }
    }

    fn new_filtered<P: IsA<Filter>>(&self, filter: &P) -> Option<MediaList> {
        unsafe {
            from_glib_full(ffi::osinfo_medialist_new_filtered(self.to_glib_none().0, filter.to_glib_none().0))
        }
    }

    fn new_intersection(&self, sourceTwo: &MediaList) -> Option<MediaList> {
        unsafe {
            from_glib_full(ffi::osinfo_medialist_new_intersection(self.to_glib_none().0, sourceTwo.to_glib_none().0))
        }
    }

    fn new_union(&self, sourceTwo: &MediaList) -> Option<MediaList> {
        unsafe {
            from_glib_full(ffi::osinfo_medialist_new_union(self.to_glib_none().0, sourceTwo.to_glib_none().0))
        }
    }
}