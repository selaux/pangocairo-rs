use cairo;
use ffi;
use glib::translate::*;
use pango;

glib_wrapper! {
    pub struct FontMap(Object<ffi::PangoCairoFontMap>);

    match fn {
        get_type => || ffi::pango_cairo_font_map_get_type(),
    }
}

impl FontMap {
    pub fn get_font_type(&self) -> cairo::enums::FontType {
        unsafe {
            ffi::pango_cairo_font_map_get_font_type(self.to_glib_none().0)
        }
    }

    pub fn get_resolution(&self) -> f64 {
        unsafe { ffi::pango_cairo_font_map_get_resolution(self.to_glib_none().0) }
    }

    pub fn set_resolution(&self, dpi: f64) {
        unsafe { ffi::pango_cairo_font_map_set_resolution(self.to_glib_none().0, dpi) }
    }

    // fn pango_cairo_font_map_set_default(fontmap: *mut PangoCairoFontMap);
}
