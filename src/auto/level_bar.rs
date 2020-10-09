// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Accessible;
use Buildable;
use LevelBarMode;
use Orientable;
use Widget;

glib_wrapper! {
    pub struct LevelBar(Object<gtk_sys::GtkLevelBar, LevelBarClass>) @extends Widget, @implements Accessible, Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_level_bar_new()).unsafe_cast() }
    }

    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_level_bar_new_for_interval(
                min_value, max_value,
            ))
            .unsafe_cast()
        }
    }

    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_add_offset_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value,
            );
        }
    }

    pub fn get_inverted(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_level_bar_get_inverted(self.to_glib_none().0)) }
    }

    pub fn get_max_value(&self) -> f64 {
        unsafe { gtk_sys::gtk_level_bar_get_max_value(self.to_glib_none().0) }
    }

    pub fn get_min_value(&self) -> f64 {
        unsafe { gtk_sys::gtk_level_bar_get_min_value(self.to_glib_none().0) }
    }

    pub fn get_mode(&self) -> LevelBarMode {
        unsafe { from_glib(gtk_sys::gtk_level_bar_get_mode(self.to_glib_none().0)) }
    }

    pub fn get_offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_level_bar_get_offset_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            let value = value.assume_init();
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    pub fn get_value(&self) -> f64 {
        unsafe { gtk_sys::gtk_level_bar_get_value(self.to_glib_none().0) }
    }

    pub fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_level_bar_remove_offset_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            gtk_sys::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    pub fn set_max_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    pub fn set_min_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            gtk_sys::gtk_level_bar_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn set_value(&self, value: f64) {
        unsafe {
            gtk_sys::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

    pub fn connect_offset_changed<F: Fn(&LevelBar, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn offset_changed_trampoline<F: Fn(&LevelBar, &str) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
            name: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(name))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"offset-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    offset_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_inverted_notify<F: Fn(&LevelBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
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
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_max_value_notify<F: Fn(&LevelBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
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
                b"notify::max-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_min_value_notify<F: Fn(&LevelBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
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
                b"notify::min-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mode_notify<F: Fn(&LevelBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
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

    pub fn connect_property_value_notify<F: Fn(&LevelBar) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&LevelBar) + 'static>(
            this: *mut gtk_sys::GtkLevelBar,
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
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for LevelBar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LevelBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LevelBar")
    }
}
