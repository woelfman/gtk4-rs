// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use Actionable;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct ListBoxRow(Object<gtk_sys::GtkListBoxRow, gtk_sys::GtkListBoxRowClass, ListBoxRowClass>) @extends Widget, @implements Accessible, Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_list_box_row_new()).unsafe_cast() }
    }
}

impl Default for ListBoxRow {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LIST_BOX_ROW: Option<&ListBoxRow> = None;

pub trait ListBoxRowExt: 'static {
    fn changed(&self);

    fn get_activatable(&self) -> bool;

    fn get_child(&self) -> Option<Widget>;

    fn get_header(&self) -> Option<Widget>;

    fn get_index(&self) -> i32;

    fn get_selectable(&self) -> bool;

    fn is_selected(&self) -> bool;

    fn set_activatable(&self, activatable: bool);

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>);

    fn set_header<P: IsA<Widget>>(&self, header: Option<&P>);

    fn set_selectable(&self, selectable: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBoxRow>> ListBoxRowExt for O {
    fn changed(&self) {
        unsafe {
            gtk_sys::gtk_list_box_row_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_get_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_list_box_row_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_list_box_row_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_index(&self) -> i32 {
        unsafe { gtk_sys::gtk_list_box_row_get_index(self.as_ref().to_glib_none().0) }
    }

    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_get_selectable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_list_box_row_is_selected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_activatable(&self, activatable: bool) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_activatable(
                self.as_ref().to_glib_none().0,
                activatable.to_glib(),
            );
        }
    }

    fn set_child<P: IsA<Widget>>(&self, child: Option<&P>) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_header<P: IsA<Widget>>(&self, header: Option<&P>) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_header(
                self.as_ref().to_glib_none().0,
                header.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_selectable(&self, selectable: bool) {
        unsafe {
            gtk_sys::gtk_list_box_row_set_selectable(
                self.as_ref().to_glib_none().0,
                selectable.to_glib(),
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("activate", &[])
                .unwrap()
        };
    }

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkListBoxRow,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ListBoxRow>,
        {
            let f: &F = &*(f as *const F);
            f(&ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListBoxRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBoxRow")
    }
}
