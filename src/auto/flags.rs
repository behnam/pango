// This file was generated by gir (18e6807) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub flags FontMask: u32 {
        const FONT_MASK_FAMILY = 1,
        const FONT_MASK_STYLE = 2,
        const FONT_MASK_VARIANT = 4,
        const FONT_MASK_WEIGHT = 8,
        const FONT_MASK_STRETCH = 16,
        const FONT_MASK_SIZE = 32,
        const FONT_MASK_GRAVITY = 64,
    }
}

#[doc(hidden)]
impl ToGlib for FontMask {
    type GlibType = ffi::PangoFontMask;

    fn to_glib(&self) -> ffi::PangoFontMask {
        ffi::PangoFontMask::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoFontMask> for FontMask {
    fn from_glib(value: ffi::PangoFontMask) -> FontMask {
        FontMask::from_bits_truncate(value.bits())
    }
}

