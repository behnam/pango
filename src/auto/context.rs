// This file was generated by gir (18e6807) from gir-files (71d73f0)
// DO NOT EDIT

use Direction;
use FontDescription;
use FontMap;
use Gravity;
use GravityHint;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Context(Object<ffi::PangoContext>);

    match fn {
        get_type => || ffi::pango_context_get_type(),
    }
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            from_glib_full(ffi::pango_context_new())
        }
    }

    #[cfg(feature = "v1_32_4")]
    pub fn changed(&self) {
        unsafe {
            ffi::pango_context_changed(self.to_glib_none().0);
        }
    }

    pub fn get_base_dir(&self) -> Direction {
        unsafe {
            from_glib(ffi::pango_context_get_base_dir(self.to_glib_none().0))
        }
    }

    pub fn get_base_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_base_gravity(self.to_glib_none().0))
        }
    }

    pub fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_description(self.to_glib_none().0))
        }
    }

    pub fn get_font_map(&self) -> Option<FontMap> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_map(self.to_glib_none().0))
        }
    }

    pub fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_gravity(self.to_glib_none().0))
        }
    }

    pub fn get_gravity_hint(&self) -> GravityHint {
        unsafe {
            from_glib(ffi::pango_context_get_gravity_hint(self.to_glib_none().0))
        }
    }

    //pub fn get_language(&self) -> /*Ignored*/Option<Language> {
    //    unsafe { TODO: call ffi::pango_context_get_language() }
    //}

    //pub fn get_matrix(&self) -> /*Ignored*/Option<Matrix> {
    //    unsafe { TODO: call ffi::pango_context_get_matrix() }
    //}

    //pub fn get_metrics(&self, desc: Option<&FontDescription>, language: /*Ignored*/Option<&mut Language>) -> /*Ignored*/Option<FontMetrics> {
    //    unsafe { TODO: call ffi::pango_context_get_metrics() }
    //}

    #[cfg(feature = "v1_32_4")]
    pub fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_context_get_serial(self.to_glib_none().0)
        }
    }

    //pub fn list_families(&self, families: /*Unimplemented*/Vec<FontFamily>) -> i32 {
    //    unsafe { TODO: call ffi::pango_context_list_families() }
    //}

    //pub fn load_font(&self, desc: &FontDescription) -> /*Ignored*/Option<Font> {
    //    unsafe { TODO: call ffi::pango_context_load_font() }
    //}

    //pub fn load_fontset(&self, desc: &FontDescription, language: /*Ignored*/&mut Language) -> /*Ignored*/Option<Fontset> {
    //    unsafe { TODO: call ffi::pango_context_load_fontset() }
    //}

    pub fn set_base_dir(&self, direction: Direction) {
        unsafe {
            ffi::pango_context_set_base_dir(self.to_glib_none().0, direction.to_glib());
        }
    }

    pub fn set_base_gravity(&self, gravity: Gravity) {
        unsafe {
            ffi::pango_context_set_base_gravity(self.to_glib_none().0, gravity.to_glib());
        }
    }

    pub fn set_font_description(&self, desc: &FontDescription) {
        unsafe {
            ffi::pango_context_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    pub fn set_font_map(&self, font_map: &FontMap) {
        unsafe {
            ffi::pango_context_set_font_map(self.to_glib_none().0, font_map.to_glib_none().0);
        }
    }

    pub fn set_gravity_hint(&self, hint: GravityHint) {
        unsafe {
            ffi::pango_context_set_gravity_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    //pub fn set_language(&self, language: /*Ignored*/&mut Language) {
    //    unsafe { TODO: call ffi::pango_context_set_language() }
    //}

    //pub fn set_matrix(&self, matrix: /*Ignored*/Option<&Matrix>) {
    //    unsafe { TODO: call ffi::pango_context_set_matrix() }
    //}
}
