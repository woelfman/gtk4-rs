// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use Accessible;
use Buildable;
use Justification;
use MovementStep;
use Widget;

glib_wrapper! {
    pub struct Label(Object<gtk_sys::GtkLabel, LabelClass>) @extends Widget, @implements Accessible, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_label_get_type(),
    }
}

impl Label {
    pub fn new(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_label_new(str.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn with_mnemonic(str: Option<&str>) -> Label {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_label_new_with_mnemonic(str.to_glib_none().0))
                .unsafe_cast()
        }
    }

    pub fn get_attributes(&self) -> Option<pango::AttrList> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_attributes(self.to_glib_none().0)) }
    }

    pub fn get_current_uri(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_current_uri(self.to_glib_none().0)) }
    }

    pub fn get_ellipsize(&self) -> pango::EllipsizeMode {
        unsafe { from_glib(gtk_sys::gtk_label_get_ellipsize(self.to_glib_none().0)) }
    }

    pub fn get_extra_menu(&self) -> Option<gio::MenuModel> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_extra_menu(self.to_glib_none().0)) }
    }

    pub fn get_justify(&self) -> Justification {
        unsafe { from_glib(gtk_sys::gtk_label_get_justify(self.to_glib_none().0)) }
    }

    pub fn get_label(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_label(self.to_glib_none().0)) }
    }

    pub fn get_layout(&self) -> Option<pango::Layout> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_layout(self.to_glib_none().0)) }
    }

    pub fn get_layout_offsets(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            gtk_sys::gtk_label_get_layout_offsets(
                self.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    pub fn get_lines(&self) -> i32 {
        unsafe { gtk_sys::gtk_label_get_lines(self.to_glib_none().0) }
    }

    pub fn get_max_width_chars(&self) -> i32 {
        unsafe { gtk_sys::gtk_label_get_max_width_chars(self.to_glib_none().0) }
    }

    pub fn get_mnemonic_keyval(&self) -> u32 {
        unsafe { gtk_sys::gtk_label_get_mnemonic_keyval(self.to_glib_none().0) }
    }

    pub fn get_mnemonic_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_label_get_mnemonic_widget(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_selectable(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_label_get_selectable(self.to_glib_none().0)) }
    }

    pub fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut end = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_label_get_selection_bounds(
                self.to_glib_none().0,
                start.as_mut_ptr(),
                end.as_mut_ptr(),
            ));
            let start = start.assume_init();
            let end = end.assume_init();
            if ret {
                Some((start, end))
            } else {
                None
            }
        }
    }

    pub fn get_single_line_mode(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_label_get_single_line_mode(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_text(&self) -> Option<GString> {
        unsafe { from_glib_none(gtk_sys::gtk_label_get_text(self.to_glib_none().0)) }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_label_get_use_markup(self.to_glib_none().0)) }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_label_get_use_underline(self.to_glib_none().0)) }
    }

    pub fn get_width_chars(&self) -> i32 {
        unsafe { gtk_sys::gtk_label_get_width_chars(self.to_glib_none().0) }
    }

    pub fn get_wrap(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_label_get_wrap(self.to_glib_none().0)) }
    }

    pub fn get_wrap_mode(&self) -> pango::WrapMode {
        unsafe { from_glib(gtk_sys::gtk_label_get_wrap_mode(self.to_glib_none().0)) }
    }

    pub fn get_xalign(&self) -> f32 {
        unsafe { gtk_sys::gtk_label_get_xalign(self.to_glib_none().0) }
    }

    pub fn get_yalign(&self) -> f32 {
        unsafe { gtk_sys::gtk_label_get_yalign(self.to_glib_none().0) }
    }

    pub fn select_region(&self, start_offset: i32, end_offset: i32) {
        unsafe {
            gtk_sys::gtk_label_select_region(self.to_glib_none().0, start_offset, end_offset);
        }
    }

    pub fn set_attributes(&self, attrs: Option<&pango::AttrList>) {
        unsafe {
            gtk_sys::gtk_label_set_attributes(self.to_glib_none().0, attrs.to_glib_none().0);
        }
    }

    pub fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        unsafe {
            gtk_sys::gtk_label_set_ellipsize(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn set_extra_menu<P: IsA<gio::MenuModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_label_set_extra_menu(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_justify(&self, jtype: Justification) {
        unsafe {
            gtk_sys::gtk_label_set_justify(self.to_glib_none().0, jtype.to_glib());
        }
    }

    pub fn set_label(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_label_set_label(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_lines(&self, lines: i32) {
        unsafe {
            gtk_sys::gtk_label_set_lines(self.to_glib_none().0, lines);
        }
    }

    pub fn set_markup(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_label_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_markup_with_mnemonic(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_label_set_markup_with_mnemonic(
                self.to_glib_none().0,
                str.to_glib_none().0,
            );
        }
    }

    pub fn set_max_width_chars(&self, n_chars: i32) {
        unsafe {
            gtk_sys::gtk_label_set_max_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    pub fn set_mnemonic_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_label_set_mnemonic_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_selectable(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_label_set_selectable(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_single_line_mode(&self, single_line_mode: bool) {
        unsafe {
            gtk_sys::gtk_label_set_single_line_mode(
                self.to_glib_none().0,
                single_line_mode.to_glib(),
            );
        }
    }

    pub fn set_text(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_label_set_text(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_text_with_mnemonic(&self, str: &str) {
        unsafe {
            gtk_sys::gtk_label_set_text_with_mnemonic(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    pub fn set_use_markup(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_label_set_use_markup(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_label_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_width_chars(&self, n_chars: i32) {
        unsafe {
            gtk_sys::gtk_label_set_width_chars(self.to_glib_none().0, n_chars);
        }
    }

    pub fn set_wrap(&self, wrap: bool) {
        unsafe {
            gtk_sys::gtk_label_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    pub fn set_wrap_mode(&self, wrap_mode: pango::WrapMode) {
        unsafe {
            gtk_sys::gtk_label_set_wrap_mode(self.to_glib_none().0, wrap_mode.to_glib());
        }
    }

    pub fn set_xalign(&self, xalign: f32) {
        unsafe {
            gtk_sys::gtk_label_set_xalign(self.to_glib_none().0, xalign);
        }
    }

    pub fn set_yalign(&self, yalign: f32) {
        unsafe {
            gtk_sys::gtk_label_set_yalign(self.to_glib_none().0, yalign);
        }
    }

    pub fn connect_activate_current_link<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_current_link_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-current-link\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_current_link_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate_current_link(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("activate-current-link", &[])
                .unwrap()
        };
    }

    pub fn connect_activate_link<F: Fn(&Label, &str) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_link_trampoline<
            F: Fn(&Label, &str) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut gtk_sys::GtkLabel,
            uri: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &GString::from_glib_borrow(uri)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate-link\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_link_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn copy_clipboard_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"copy-clipboard\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    copy_clipboard_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_copy_clipboard(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("copy-clipboard", &[])
                .unwrap()
        };
    }

    pub fn connect_move_cursor<F: Fn(&Label, MovementStep, i32, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_cursor_trampoline<
            F: Fn(&Label, MovementStep, i32, bool) + 'static,
        >(
            this: *mut gtk_sys::GtkLabel,
            step: gtk_sys::GtkMovementStep,
            count: libc::c_int,
            extend_selection: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(step),
                count,
                from_glib(extend_selection),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-cursor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_cursor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_move_cursor(&self, step: MovementStep, count: i32, extend_selection: bool) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
                .emit("move-cursor", &[&step, &count, &extend_selection])
                .unwrap()
        };
    }

    pub fn connect_property_attributes_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_attributes_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_attributes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_ellipsize_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ellipsize_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ellipsize_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_extra_menu_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_menu_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::extra-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_extra_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_justify_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_justify_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::justify\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_justify_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_label_notify<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_lines_notify<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lines_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::lines\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lines_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_max_width_chars_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_chars_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::max-width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_chars_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mnemonic_keyval_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonic_keyval_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::mnemonic-keyval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mnemonic_keyval_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mnemonic_widget_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonic_widget_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::mnemonic-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mnemonic_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_selectable_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_single_line_mode_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_single_line_mode_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::single-line-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_single_line_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_markup_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_underline_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_width_chars_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_chars_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_chars_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_wrap_notify<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_wrap_mode_notify<F: Fn(&Label) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_mode_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::wrap-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_xalign_notify<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_xalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_yalign_notify<F: Fn(&Label) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<F: Fn(&Label) + 'static>(
            this: *mut gtk_sys::GtkLabel,
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
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_yalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label")
    }
}
