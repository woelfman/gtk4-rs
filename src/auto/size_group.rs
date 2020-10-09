// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use SizeGroupMode;
use Widget;

glib_wrapper! {
    pub struct SizeGroup(Object<gtk_sys::GtkSizeGroup, SizeGroupClass>) @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_size_group_get_type(),
    }
}

impl SizeGroup {
    pub fn new(mode: SizeGroupMode) -> SizeGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_size_group_new(mode.to_glib())) }
    }

    pub fn add_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            gtk_sys::gtk_size_group_add_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn get_mode(&self) -> SizeGroupMode {
        unsafe { from_glib(gtk_sys::gtk_size_group_get_mode(self.to_glib_none().0)) }
    }

    pub fn get_widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_size_group_get_widgets(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn remove_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            gtk_sys::gtk_size_group_remove_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn set_mode(&self, mode: SizeGroupMode) {
        unsafe {
            gtk_sys::gtk_size_group_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn connect_property_mode_notify<F: Fn(&SizeGroup) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&SizeGroup) + 'static>(
            this: *mut gtk_sys::GtkSizeGroup,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SizeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SizeGroup")
    }
}
