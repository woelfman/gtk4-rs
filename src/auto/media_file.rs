// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gio;
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
use MediaStream;

glib_wrapper! {
    pub struct MediaFile(Object<gtk_sys::GtkMediaFile, gtk_sys::GtkMediaFileClass, MediaFileClass>) @extends MediaStream, @implements gdk::Paintable;

    match fn {
        get_type => || gtk_sys::gtk_media_file_get_type(),
    }
}

impl MediaFile {
    pub fn new() -> MediaFile {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_media_file_new()) }
    }

    pub fn new_for_file<P: IsA<gio::File>>(file: &P) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_media_file_new_for_file(
                file.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn new_for_filename(filename: &str) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_media_file_new_for_filename(
                filename.to_glib_none().0,
            ))
        }
    }

    pub fn new_for_input_stream<P: IsA<gio::InputStream>>(stream: &P) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_media_file_new_for_input_stream(
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn new_for_resource(resource_path: &str) -> MediaFile {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_media_file_new_for_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }
}

impl Default for MediaFile {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MEDIA_FILE: Option<&MediaFile> = None;

pub trait MediaFileExt: 'static {
    fn clear(&self);

    fn get_file(&self) -> Option<gio::File>;

    fn get_input_stream(&self) -> Option<gio::InputStream>;

    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>);

    fn set_filename(&self, filename: Option<&str>);

    fn set_input_stream<P: IsA<gio::InputStream>>(&self, stream: Option<&P>);

    fn set_resource(&self, resource_path: Option<&str>);

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<MediaFile>> MediaFileExt for O {
    fn clear(&self) {
        unsafe {
            gtk_sys::gtk_media_file_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_sys::gtk_media_file_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_input_stream(&self) -> Option<gio::InputStream> {
        unsafe {
            from_glib_none(gtk_sys::gtk_media_file_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>) {
        unsafe {
            gtk_sys::gtk_media_file_set_file(
                self.as_ref().to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_filename(&self, filename: Option<&str>) {
        unsafe {
            gtk_sys::gtk_media_file_set_filename(
                self.as_ref().to_glib_none().0,
                filename.to_glib_none().0,
            );
        }
    }

    fn set_input_stream<P: IsA<gio::InputStream>>(&self, stream: Option<&P>) {
        unsafe {
            gtk_sys::gtk_media_file_set_input_stream(
                self.as_ref().to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            gtk_sys::gtk_media_file_set_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMediaFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MediaFile>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_stream_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMediaFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MediaFile>,
        {
            let f: &F = &*(f as *const F);
            f(&MediaFile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-stream\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_stream_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MediaFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MediaFile")
    }
}
