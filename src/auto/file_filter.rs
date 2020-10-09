// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;

glib_wrapper! {
    pub struct FileFilter(Object<gtk_sys::GtkFileFilter, FileFilterClass>) @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_file_filter_new()) }
    }

    pub fn from_gvariant(variant: &glib::Variant) -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_file_filter_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            gtk_sys::gtk_file_filter_add_mime_type(
                self.to_glib_none().0,
                mime_type.to_glib_none().0,
            );
        }
    }

    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            gtk_sys::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    pub fn add_pixbuf_formats(&self) {
        unsafe {
            gtk_sys::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    pub fn get_attributes(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_file_filter_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(gtk_sys::gtk_file_filter_to_gvariant(self.to_glib_none().0)) }
    }

    pub fn connect_property_name_notify<F: Fn(&FileFilter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&FileFilter) + 'static>(
            this: *mut gtk_sys::GtkFileFilter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FileFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileFilter")
    }
}
