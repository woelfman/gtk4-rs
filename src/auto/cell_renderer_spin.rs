// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Adjustment;
use CellRenderer;
use CellRendererText;

glib_wrapper! {
    pub struct CellRendererSpin(Object<gtk_sys::GtkCellRendererSpin, CellRendererSpinClass>) @extends CellRendererText, CellRenderer;

    match fn {
        get_type => || gtk_sys::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(gtk_sys::gtk_cell_renderer_spin_new()).unsafe_cast() }
    }

    pub fn get_property_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            let mut value = Value::from_type(<Adjustment as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"adjustment\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `adjustment` getter")
        }
    }

    pub fn set_property_adjustment<P: IsA<Adjustment> + SetValueOptional>(
        &self,
        adjustment: Option<&P>,
    ) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"adjustment\0".as_ptr() as *const _,
                Value::from(adjustment).to_glib_none().0,
            );
        }
    }

    pub fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"climb-rate\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `climb-rate` getter")
                .unwrap()
        }
    }

    pub fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"climb-rate\0".as_ptr() as *const _,
                Value::from(&climb_rate).to_glib_none().0,
            );
        }
    }

    pub fn get_property_digits(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"digits\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `digits` getter")
                .unwrap()
        }
    }

    pub fn set_property_digits(&self, digits: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"digits\0".as_ptr() as *const _,
                Value::from(&digits).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_adjustment_notify<F: Fn(&CellRendererSpin) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut gtk_sys::GtkCellRendererSpin,
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
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_climb_rate_notify<F: Fn(&CellRendererSpin) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_climb_rate_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut gtk_sys::GtkCellRendererSpin,
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
                b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_climb_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_digits_notify<F: Fn(&CellRendererSpin) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut gtk_sys::GtkCellRendererSpin,
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
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererSpin {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CellRendererSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererSpin")
    }
}
