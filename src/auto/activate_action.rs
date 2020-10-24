// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_sys;
use std::fmt;
use ShortcutAction;

glib_wrapper! {
    pub struct ActivateAction(Object<gtk_sys::GtkActivateAction, gtk_sys::GtkActivateActionClass, ActivateActionClass>) @extends ShortcutAction;

    match fn {
        get_type => || gtk_sys::gtk_activate_action_get_type(),
    }
}

impl ActivateAction {
    pub fn get() -> Option<ShortcutAction> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_sys::gtk_activate_action_get()) }
    }
}

impl fmt::Display for ActivateAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActivateAction")
    }
}
