// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct Spinner(Object<gtk_sys::GtkSpinner, SpinnerClass>) @extends Widget, @implements Accessible, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_spinner_get_type(),
    }
}

impl Spinner {
    pub fn new() -> Spinner {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_spinner_new()).unsafe_cast() }
    }

    pub fn get_spinning(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_spinner_get_spinning(self.to_glib_none().0)) }
    }

    pub fn set_spinning(&self, spinning: bool) {
        unsafe {
            gtk_sys::gtk_spinner_set_spinning(self.to_glib_none().0, spinning.to_glib());
        }
    }

    pub fn start(&self) {
        unsafe {
            gtk_sys::gtk_spinner_start(self.to_glib_none().0);
        }
    }

    pub fn stop(&self) {
        unsafe {
            gtk_sys::gtk_spinner_stop(self.to_glib_none().0);
        }
    }

    pub fn connect_property_spinning_notify<F: Fn(&Spinner) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_spinning_trampoline<F: Fn(&Spinner) + 'static>(
            this: *mut gtk_sys::GtkSpinner,
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
                b"notify::spinning\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spinning_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Spinner")
    }
}
