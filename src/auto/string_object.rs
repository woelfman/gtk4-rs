// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct StringObject(Object<gtk_sys::GtkStringObject, gtk_sys::GtkStringObjectClass, StringObjectClass>);

    match fn {
        get_type => || gtk_sys::gtk_string_object_get_type(),
    }
}

impl StringObject {
    pub fn new(string: &str) -> StringObject {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_string_object_new(string.to_glib_none().0)) }
    }
}

pub const NONE_STRING_OBJECT: Option<&StringObject> = None;

pub trait StringObjectExt: 'static {
    fn get_string(&self) -> GString;

    fn connect_property_string_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StringObject>> StringObjectExt for O {
    fn get_string(&self) -> GString {
        unsafe {
            from_glib_none(gtk_sys::gtk_string_object_get_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_property_string_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_string_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkStringObject,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StringObject>,
        {
            let f: &F = &*(f as *const F);
            f(&StringObject::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::string\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_string_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StringObject")
    }
}
