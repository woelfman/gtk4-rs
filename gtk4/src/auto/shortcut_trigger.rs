// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkShortcutTrigger")]
    pub struct ShortcutTrigger(Object<ffi::GtkShortcutTrigger, ffi::GtkShortcutTriggerClass>);

    match fn {
        type_ => || ffi::gtk_shortcut_trigger_get_type(),
    }
}

impl ShortcutTrigger {
    pub const NONE: Option<&'static ShortcutTrigger> = None;

    #[doc(alias = "gtk_shortcut_trigger_parse_string")]
    pub fn parse_string(string: &str) -> Option<ShortcutTrigger> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_parse_string(
                string.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ShortcutTrigger {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&ShortcutTriggerExt::to_str(self))
    }
}

pub trait ShortcutTriggerExt: IsA<ShortcutTrigger> + 'static {
    #[doc(alias = "gtk_shortcut_trigger_to_label")]
    fn to_label(&self, display: &impl IsA<gdk::Display>) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_to_label(
                self.as_ref().to_glib_none().0,
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_trigger_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_shortcut_trigger_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_trigger_trigger")]
    fn trigger(&self, event: impl AsRef<gdk::Event>, enable_mnemonics: bool) -> gdk::KeyMatch {
        unsafe {
            from_glib(ffi::gtk_shortcut_trigger_trigger(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                enable_mnemonics.into_glib(),
            ))
        }
    }
}

impl<O: IsA<ShortcutTrigger>> ShortcutTriggerExt for O {}
