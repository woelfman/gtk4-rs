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
use Stack;
use Widget;

glib_wrapper! {
    pub struct StackSwitcher(Object<gtk_sys::GtkStackSwitcher, StackSwitcherClass>) @extends Widget, @implements Accessible, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_stack_switcher_get_type(),
    }
}

impl StackSwitcher {
    pub fn new() -> StackSwitcher {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_stack_switcher_new()).unsafe_cast() }
    }

    pub fn get_stack(&self) -> Option<Stack> {
        unsafe { from_glib_none(gtk_sys::gtk_stack_switcher_get_stack(self.to_glib_none().0)) }
    }

    pub fn set_stack(&self, stack: Option<&Stack>) {
        unsafe {
            gtk_sys::gtk_stack_switcher_set_stack(self.to_glib_none().0, stack.to_glib_none().0);
        }
    }

    pub fn connect_property_stack_notify<F: Fn(&StackSwitcher) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stack_trampoline<F: Fn(&StackSwitcher) + 'static>(
            this: *mut gtk_sys::GtkStackSwitcher,
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
                b"notify::stack\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stack_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for StackSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StackSwitcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackSwitcher")
    }
}
