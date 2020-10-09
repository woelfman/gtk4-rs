// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Accessible(Interface<gtk_sys::GtkAccessible>);

    match fn {
        get_type => || gtk_sys::gtk_accessible_get_type(),
    }
}

pub const NONE_ACCESSIBLE: Option<&Accessible> = None;

pub trait AccessibleExt: 'static {
    //fn get_accessible_role(&self) -> /*Ignored*/AccessibleRole;

    //fn reset_property(&self, property: /*Ignored*/AccessibleProperty);

    //fn reset_relation(&self, relation: /*Ignored*/AccessibleRelation);

    //fn reset_state(&self, state: /*Ignored*/AccessibleState);

    //fn update_property(&self, first_property: /*Ignored*/AccessibleProperty, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn update_property_value(&self, property: /*Ignored*/AccessibleProperty, value: &glib::Value);

    //fn update_relation(&self, first_relation: /*Ignored*/AccessibleRelation, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn update_relation_value(&self, relation: /*Ignored*/AccessibleRelation, value: &glib::Value);

    //fn update_state(&self, first_state: /*Ignored*/AccessibleState, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn update_state_value(&self, state: /*Ignored*/AccessibleState, value: &glib::Value);

    //fn set_property_accessible_role(&self, accessible_role: /*Ignored*/AccessibleRole);

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Accessible>> AccessibleExt for O {
    //fn get_accessible_role(&self) -> /*Ignored*/AccessibleRole {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_get_accessible_role() }
    //}

    //fn reset_property(&self, property: /*Ignored*/AccessibleProperty) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_reset_property() }
    //}

    //fn reset_relation(&self, relation: /*Ignored*/AccessibleRelation) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_reset_relation() }
    //}

    //fn reset_state(&self, state: /*Ignored*/AccessibleState) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_reset_state() }
    //}

    //fn update_property(&self, first_property: /*Ignored*/AccessibleProperty, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_property() }
    //}

    //fn update_property_value(&self, property: /*Ignored*/AccessibleProperty, value: &glib::Value) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_property_value() }
    //}

    //fn update_relation(&self, first_relation: /*Ignored*/AccessibleRelation, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_relation() }
    //}

    //fn update_relation_value(&self, relation: /*Ignored*/AccessibleRelation, value: &glib::Value) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_relation_value() }
    //}

    //fn update_state(&self, first_state: /*Ignored*/AccessibleState, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_state() }
    //}

    //fn update_state_value(&self, state: /*Ignored*/AccessibleState, value: &glib::Value) {
    //    unsafe { TODO: call gtk_sys:gtk_accessible_update_state_value() }
    //}

    //fn set_property_accessible_role(&self, accessible_role: /*Ignored*/AccessibleRole) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accessible-role\0".as_ptr() as *const _, Value::from(&accessible_role).to_glib_none().0);
    //    }
    //}

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAccessible,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Accessible>,
        {
            let f: &F = &*(f as *const F);
            f(&Accessible::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Accessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Accessible")
    }
}
