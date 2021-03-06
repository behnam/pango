// This file was generated by gir (18e6807) from gir-files (71d73f0)
// DO NOT EDIT

use Alignment;
use Context;
use EllipsizeMode;
use FontDescription;
use LayoutIter;
use LayoutLine;
use Rectangle;
use WrapMode;
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Layout(Object<ffi::PangoLayout>);

    match fn {
        get_type => || ffi::pango_layout_get_type(),
    }
}

impl Layout {
    pub fn new(context: &Context) -> Layout {
        unsafe {
            from_glib_full(ffi::pango_layout_new(context.to_glib_none().0))
        }
    }

    pub fn context_changed(&self) {
        unsafe {
            ffi::pango_layout_context_changed(self.to_glib_none().0);
        }
    }

    pub fn copy(&self) -> Option<Layout> {
        unsafe {
            from_glib_full(ffi::pango_layout_copy(self.to_glib_none().0))
        }
    }

    pub fn get_alignment(&self) -> Alignment {
        unsafe {
            from_glib(ffi::pango_layout_get_alignment(self.to_glib_none().0))
        }
    }

    //pub fn get_attributes(&self) -> /*Ignored*/Option<AttrList> {
    //    unsafe { TODO: call ffi::pango_layout_get_attributes() }
    //}

    pub fn get_auto_dir(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_auto_dir(self.to_glib_none().0))
        }
    }

    pub fn get_baseline(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_baseline(self.to_glib_none().0)
        }
    }

    pub fn get_character_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_character_count(self.to_glib_none().0)
        }
    }

    pub fn get_context(&self) -> Option<Context> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_context(self.to_glib_none().0))
        }
    }

    pub fn get_cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            ffi::pango_layout_get_cursor_pos(self.to_glib_none().0, index_, strong_pos.to_glib_none_mut().0, weak_pos.to_glib_none_mut().0);
            (strong_pos, weak_pos)
        }
    }

    pub fn get_ellipsize(&self) -> EllipsizeMode {
        unsafe {
            from_glib(ffi::pango_layout_get_ellipsize(self.to_glib_none().0))
        }
    }

    pub fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_extents(self.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_font_description(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_indent(self.to_glib_none().0)
        }
    }

    pub fn get_iter(&self) -> Option<LayoutIter> {
        unsafe {
            from_glib_full(ffi::pango_layout_get_iter(self.to_glib_none().0))
        }
    }

    pub fn get_justify(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_justify(self.to_glib_none().0))
        }
    }

    pub fn get_line(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line(self.to_glib_none().0, line))
        }
    }

    pub fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_line_count(self.to_glib_none().0)
        }
    }

    pub fn get_line_readonly(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_line_readonly(self.to_glib_none().0, line))
        }
    }

    pub fn get_lines(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines(self.to_glib_none().0))
        }
    }

    pub fn get_lines_readonly(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::pango_layout_get_lines_readonly(self.to_glib_none().0))
        }
    }

    //pub fn get_log_attrs(&self, attrs: /*Unimplemented*/Vec<LogAttr>) -> i32 {
    //    unsafe { TODO: call ffi::pango_layout_get_log_attrs() }
    //}

    //pub fn get_log_attrs_readonly(&self) -> (/*Ignored*/Vec<LogAttr>, i32) {
    //    unsafe { TODO: call ffi::pango_layout_get_log_attrs_readonly() }
    //}

    pub fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_get_pixel_extents(self.to_glib_none().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_pixel_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::pango_layout_get_pixel_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    #[cfg(feature = "v1_32_4")]
    pub fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_layout_get_serial(self.to_glib_none().0)
        }
    }

    pub fn get_single_paragraph_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_get_single_paragraph_mode(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::pango_layout_get_size(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_spacing(self.to_glib_none().0)
        }
    }

    //pub fn get_tabs(&self) -> /*Ignored*/Option<TabArray> {
    //    unsafe { TODO: call ffi::pango_layout_get_tabs() }
    //}

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_layout_get_text(self.to_glib_none().0))
        }
    }

    pub fn get_unknown_glyphs_count(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_unknown_glyphs_count(self.to_glib_none().0)
        }
    }

    pub fn get_width(&self) -> i32 {
        unsafe {
            ffi::pango_layout_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_wrap(&self) -> WrapMode {
        unsafe {
            from_glib(ffi::pango_layout_get_wrap(self.to_glib_none().0))
        }
    }

    pub fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32) {
        unsafe {
            let mut line = mem::uninitialized();
            let mut x_pos = mem::uninitialized();
            ffi::pango_layout_index_to_line_x(self.to_glib_none().0, index_, trailing.to_glib(), &mut line, &mut x_pos);
            (line, x_pos)
        }
    }

    pub fn index_to_pos(&self, index_: i32) -> Rectangle {
        unsafe {
            let mut pos = Rectangle::uninitialized();
            ffi::pango_layout_index_to_pos(self.to_glib_none().0, index_, pos.to_glib_none_mut().0);
            pos
        }
    }

    pub fn is_ellipsized(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_is_ellipsized(self.to_glib_none().0))
        }
    }

    pub fn is_wrapped(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_is_wrapped(self.to_glib_none().0))
        }
    }

    pub fn move_cursor_visually(&self, strong: bool, old_index: i32, old_trailing: i32, direction: i32) -> (i32, i32) {
        unsafe {
            let mut new_index = mem::uninitialized();
            let mut new_trailing = mem::uninitialized();
            ffi::pango_layout_move_cursor_visually(self.to_glib_none().0, strong.to_glib(), old_index, old_trailing, direction, &mut new_index, &mut new_trailing);
            (new_index, new_trailing)
        }
    }

    pub fn set_alignment(&self, alignment: Alignment) {
        unsafe {
            ffi::pango_layout_set_alignment(self.to_glib_none().0, alignment.to_glib());
        }
    }

    //pub fn set_attributes(&self, attrs: /*Ignored*/Option<&AttrList>) {
    //    unsafe { TODO: call ffi::pango_layout_set_attributes() }
    //}

    pub fn set_auto_dir(&self, auto_dir: bool) {
        unsafe {
            ffi::pango_layout_set_auto_dir(self.to_glib_none().0, auto_dir.to_glib());
        }
    }

    pub fn set_ellipsize(&self, ellipsize: EllipsizeMode) {
        unsafe {
            ffi::pango_layout_set_ellipsize(self.to_glib_none().0, ellipsize.to_glib());
        }
    }

    pub fn set_font_description(&self, desc: Option<&FontDescription>) {
        unsafe {
            ffi::pango_layout_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    pub fn set_height(&self, height: i32) {
        unsafe {
            ffi::pango_layout_set_height(self.to_glib_none().0, height);
        }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe {
            ffi::pango_layout_set_indent(self.to_glib_none().0, indent);
        }
    }

    pub fn set_justify(&self, justify: bool) {
        unsafe {
            ffi::pango_layout_set_justify(self.to_glib_none().0, justify.to_glib());
        }
    }

    pub fn set_markup(&self, markup: &str, length: i32) {
        unsafe {
            ffi::pango_layout_set_markup(self.to_glib_none().0, markup.to_glib_none().0, length);
        }
    }

    pub fn set_markup_with_accel(&self, markup: &str, length: i32, accel_marker: char) -> char {
        unsafe {
            let mut accel_char = mem::uninitialized();
            ffi::pango_layout_set_markup_with_accel(self.to_glib_none().0, markup.to_glib_none().0, length, accel_marker.to_glib(), &mut accel_char);
            from_glib(accel_char)
        }
    }

    pub fn set_single_paragraph_mode(&self, setting: bool) {
        unsafe {
            ffi::pango_layout_set_single_paragraph_mode(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::pango_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    //pub fn set_tabs(&self, tabs: /*Ignored*/Option<&mut TabArray>) {
    //    unsafe { TODO: call ffi::pango_layout_set_tabs() }
    //}

    pub fn set_text(&self, text: &str, length: i32) {
        unsafe {
            ffi::pango_layout_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    pub fn set_width(&self, width: i32) {
        unsafe {
            ffi::pango_layout_set_width(self.to_glib_none().0, width);
        }
    }

    pub fn set_wrap(&self, wrap: WrapMode) {
        unsafe {
            ffi::pango_layout_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    pub fn xy_to_index(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        unsafe {
            let mut index_ = mem::uninitialized();
            let mut trailing = mem::uninitialized();
            let ret = from_glib(ffi::pango_layout_xy_to_index(self.to_glib_none().0, x, y, &mut index_, &mut trailing));
            if ret { Some((index_, trailing)) } else { None }
        }
    }
}
