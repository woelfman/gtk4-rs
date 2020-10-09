// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use Buildable;
use FontChooser;
use Widget;

glib_wrapper! {
    pub struct FontChooserWidget(Object<gtk_sys::GtkFontChooserWidget, FontChooserWidgetClass>) @extends Widget, @implements Accessible, Buildable, FontChooser;

    match fn {
        get_type => || gtk_sys::gtk_font_chooser_widget_get_type(),
    }
}

impl FontChooserWidget {
    pub fn new() -> FontChooserWidget {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_font_chooser_widget_new()).unsafe_cast() }
    }

    pub fn get_property_tweak_action(&self) -> Option<gio::Action> {
        unsafe {
            let mut value = Value::from_type(<gio::Action as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"tweak-action\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tweak-action` getter")
        }
    }

    pub fn connect_property_tweak_action_notify<F: Fn(&FontChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tweak_action_trampoline<F: Fn(&FontChooserWidget) + 'static>(
            this: *mut gtk_sys::GtkFontChooserWidget,
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
                b"notify::tweak-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tweak_action_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FontChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FontChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontChooserWidget")
    }
}
