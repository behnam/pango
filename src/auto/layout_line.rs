// This file was generated by gir (18e6807) from gir-files (71d73f0)
// DO NOT EDIT

use Rectangle;
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct LayoutLine(Shared<ffi::PangoLayoutLine>);

    match fn {
        ref => |ptr| ffi::pango_layout_line_ref(ptr),
        unref => |ptr| ffi::pango_layout_line_unref(ptr),
    }
}

impl LayoutLine {
    pub fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_extents(self.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_line_get_pixel_extents(self.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    //pub fn get_x_ranges(&self, start_index: i32, end_index: i32, ranges: /*Unimplemented*/CArray TypeId { ns_id: 0, id: 14 }) -> i32 {
    //    unsafe { TODO: call ffi::pango_layout_line_get_x_ranges() }
    //}

    pub fn index_to_x(&self, index_: i32, trailing: bool) -> i32 {
        unsafe {
            let mut x_pos = mem::uninitialized();
            ffi::pango_layout_line_index_to_x(self.to_glib_none().0, index_, trailing.to_glib(), &mut x_pos);
            x_pos
        }
    }

    pub fn x_to_index(&self, x_pos: i32) -> Option<(i32, i32)> {
        unsafe {
            let mut index_ = mem::uninitialized();
            let mut trailing = mem::uninitialized();
            let ret = from_glib(ffi::pango_layout_line_x_to_index(self.to_glib_none().0, x_pos, &mut index_, &mut trailing));
            if ret { Some((index_, trailing)) } else { None }
        }
    }
}
