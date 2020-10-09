// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
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
use TreeListRow;

glib_wrapper! {
    pub struct TreeListModel(Object<gtk_sys::GtkTreeListModel, gtk_sys::GtkTreeListModelClass, TreeListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || gtk_sys::gtk_tree_list_model_get_type(),
    }
}

impl TreeListModel {
    pub fn new<P: IsA<gio::ListModel>, Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static>(
        root: &P,
        passthrough: bool,
        autoexpand: bool,
        create_func: Q,
    ) -> TreeListModel {
        assert_initialized_main_thread!();
        let create_func_data: Box_<Q> = Box_::new(create_func);
        unsafe extern "C" fn create_func_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static,
        >(
            item: *mut gobject_sys::GObject,
            user_data: glib_sys::gpointer,
        ) -> *mut gio_sys::GListModel {
            let item = from_glib_borrow(item);
            let callback: &Q = &*(user_data as *mut _);
            let res = (*callback)(&item);
            res.to_glib_full()
        }
        let create_func = Some(create_func_func::<P, Q> as _);
        unsafe extern "C" fn user_destroy_func<
            P: IsA<gio::ListModel>,
            Q: Fn(&glib::Object) -> Option<gio::ListModel> + 'static,
        >(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
        }
        let destroy_call5 = Some(user_destroy_func::<P, Q> as _);
        let super_callback0: Box_<Q> = create_func_data;
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_model_new(
                root.as_ref().to_glib_full(),
                passthrough.to_glib(),
                autoexpand.to_glib(),
                create_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call5,
            ))
        }
    }
}

pub const NONE_TREE_LIST_MODEL: Option<&TreeListModel> = None;

pub trait TreeListModelExt: 'static {
    fn get_autoexpand(&self) -> bool;

    fn get_child_row(&self, position: u32) -> Option<TreeListRow>;

    fn get_model(&self) -> Option<gio::ListModel>;

    fn get_passthrough(&self) -> bool;

    fn get_row(&self, position: u32) -> Option<TreeListRow>;

    fn set_autoexpand(&self, autoexpand: bool);

    fn connect_property_autoexpand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeListModel>> TreeListModelExt for O {
    fn get_autoexpand(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_list_model_get_autoexpand(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_child_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_model_get_child_row(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_list_model_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_passthrough(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_list_model_get_passthrough(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_row(&self, position: u32) -> Option<TreeListRow> {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_list_model_get_row(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn set_autoexpand(&self, autoexpand: bool) {
        unsafe {
            gtk_sys::gtk_tree_list_model_set_autoexpand(
                self.as_ref().to_glib_none().0,
                autoexpand.to_glib(),
            );
        }
    }

    fn connect_property_autoexpand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoexpand_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::autoexpand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoexpand_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkTreeListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TreeListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&TreeListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TreeListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeListModel")
    }
}
