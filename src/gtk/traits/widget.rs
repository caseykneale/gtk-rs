// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use libc::{c_int, c_char, self};
use std::ffi::CString;
use gtk::ffi;
use glib::{to_bool, to_gboolean};
use gdk;
use gdk_ffi;
use gtk;
use glib;
use glib::ffi::GType;

pub trait WidgetTrait: ffi::FFIWidget + gtk::GObjectTrait {
    fn show_all(&mut self) -> () {
        unsafe {
            ffi::gtk_widget_show_all(self.get_widget());
        }
    }

    fn show_now(&self) {
        unsafe { ffi::gtk_widget_show_now(self.get_widget()) }
    }

    fn hide(&self) {
        unsafe { ffi::gtk_widget_hide(self.get_widget()) }
    }

    fn map(&self) {
        unsafe { ffi::gtk_widget_map(self.get_widget()) }
    }

    fn unmap(&self) {
        unsafe { ffi::gtk_widget_unmap(self.get_widget()) }
    }

    fn realize(&self) {
        unsafe { ffi::gtk_widget_realize(self.get_widget()) }
    }

    fn unrealize(&self) {
        unsafe { ffi::gtk_widget_unrealize(self.get_widget()) }
    }

    fn queue_draw(&self) {
        unsafe { ffi::gtk_widget_queue_draw(self.get_widget()) }
    }

    fn queue_resize(&self) {
        unsafe { ffi::gtk_widget_queue_resize(self.get_widget()) }
    }

    fn queue_resize_no_redraw(&self) {
        unsafe { ffi::gtk_widget_queue_resize_no_redraw(self.get_widget()) }
    }

    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_scale_factor(self.get_widget()) }
    }

    fn activate(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_activate(self.get_widget())) }
    }

    fn reparent<T: WidgetTrait>(&self, new_parent: &T) {
        unsafe { ffi::gtk_widget_reparent(self.get_widget(), new_parent.get_widget()) }
    }

    fn is_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_focus(self.get_widget())) }
    }

    fn grab_focus(&self) {
        unsafe { ffi::gtk_widget_grab_focus(self.get_widget()) }
    }

    fn grab_default(&self) {
        unsafe { ffi::gtk_widget_grab_default(self.get_widget()) }
    }

    fn set_name(&self, name: &str) {
        let c_str = CString::from_slice(name.as_bytes());

        unsafe { ffi::gtk_widget_set_name(self.get_widget(), c_str.as_ptr()) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            let tmp = ffi::gtk_widget_get_name(self.get_widget());

            if tmp.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&tmp)).to_string())
            }
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe { ffi::gtk_widget_set_sensitive(self.get_widget(), to_gboolean(sensitive)) }
    }

    fn set_parent<T: WidgetTrait>(&self, parent: &T) {
        unsafe { ffi::gtk_widget_set_parent(self.get_widget(), parent.get_widget()) }
    }

    /*fn set_parent_window(&self, parent: &Widget) {
        unsafe { gtk_widget_set_parent_window(self.get_widget(), parent.get_widget()) }
    }*/

    fn get_toplevel(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_toplevel(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn get_ancestor(&self, widget_type: GType) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_ancestor(self.get_widget(), widget_type) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn is_ancestor<T: WidgetTrait>(&self, ancestor: &T) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_ancestor(self.get_widget(), ancestor.get_widget())) }
    }

    fn hide_on_delete(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_hide_on_delete(self.get_widget())) }
    }

    fn set_direction(&self, dir: gtk::TextDirection) {
        unsafe { ffi::gtk_widget_set_direction(self.get_widget(), dir) }
    }

    fn get_direction(&self) -> gtk::TextDirection {
        unsafe { ffi::gtk_widget_get_direction(self.get_widget()) }
    }

    fn set_default_direction(dir: gtk::TextDirection) {
        unsafe { ffi::gtk_widget_set_default_direction(dir) }
    }

    fn get_default_direction() -> gtk::TextDirection {
        unsafe { ffi::gtk_widget_get_default_direction() }
    }

    fn in_destruction(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_in_destruction(self.get_widget())) }
    }

    fn unparent(&self) {
        unsafe { ffi::gtk_widget_unparent(self.get_widget()) }
    }

    fn translate_coordinates<T: WidgetTrait>(&self, dest_widget: &T, src_x: i32, src_y: i32) -> Option<(i32, i32)> {
        let mut dest_x = 0i32;
        let mut dest_y = 0i32;

        let r = to_bool(unsafe { ffi::gtk_widget_translate_coordinates(self.get_widget(), dest_widget.get_widget(), src_x, src_y, &mut dest_x, &mut dest_y) });
        if r {
            Some((dest_x, dest_y))
        }
        else {
            None
        }
    }

    fn override_background_color(&self, state: gtk::StateFlags, color: &gdk_ffi::C_GdkRGBA) {
        unsafe { ffi::gtk_widget_override_background_color(self.get_widget(), state, color) }
    }

    fn override_color(&self, state: gtk::StateFlags, color: &gdk_ffi::C_GdkRGBA) {
        unsafe { ffi::gtk_widget_override_color(self.get_widget(), state, color) }
    }

    fn override_symbolic_color(&self, name: &str, color: &gdk_ffi::C_GdkRGBA) {
        let c_str = CString::from_slice(name.as_bytes());

        unsafe { ffi::gtk_widget_override_symbolic_color(self.get_widget(), c_str.as_ptr(), color); }
    }

    fn override_cursor(&self, cursor: &gdk_ffi::C_GdkRGBA, secondary_cursor: &gdk_ffi::C_GdkRGBA) {
        unsafe { ffi::gtk_widget_override_cursor(self.get_widget(), cursor, secondary_cursor) }
    }

    fn queue_draw_area(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_queue_draw_area(self.get_widget(), x, y, width, height) }
    }

    fn set_app_paintable(&self, app_paintable: bool) {
        unsafe { ffi::gtk_widget_set_app_paintable(self.get_widget(), to_gboolean(app_paintable)) }
    }

    fn set_double_buffered(&self, double_buffered: bool) {
        unsafe { ffi::gtk_widget_set_double_buffered(self.get_widget(), to_gboolean(double_buffered)) }
    }

    fn set_redraw_on_allocate(&self, redraw_on_allocate: bool) {
        unsafe { ffi::gtk_widget_set_redraw_on_allocate(self.get_widget(), to_gboolean(redraw_on_allocate)) }
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        unsafe { to_bool(ffi::gtk_widget_mnemonic_activate(self.get_widget(), to_gboolean(group_cycling))) }
    }

    /*fn send_expose(&self, event: &mut gdk::Event) -> i32 {
        unsafe { ffi::gtk_widget_send_expose(self.get_widget(), event) }
    }

    fn send_focus_change(&self, event: &mut gdk::Event) -> bool {
        unsafe { to_bool(ffi::gtk_widget_send_expose(self.get_widget(), event)) }
    }*/

    fn child_focus(&self, direction: gtk::DirectionType) -> bool {
        unsafe { to_bool(ffi::gtk_widget_child_focus(self.get_widget(), direction)) }
    }

    fn get_child_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_child_visible(self.get_widget())) }
    }

    fn get_parent(&self) -> Option<Self> {
        let tmp = unsafe { ffi::gtk_widget_get_parent(self.get_widget()) };

        if tmp.is_null() {
            None
        } else {
            Some(ffi::FFIWidget::wrap(tmp))
        }
    }

    fn has_screen(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_screen(self.get_widget())) }
    }

    fn get_size_request(&self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;

        unsafe { ffi::gtk_widget_get_size_request(self.get_widget(), &mut width, &mut height) };
        (width, height)
    }

    fn set_child_visible(&self, is_visible: bool) {
        unsafe { ffi::gtk_widget_set_child_visible(self.get_widget(), to_gboolean(is_visible)) }
    }

    fn set_size_request(&self, width: i32, height: i32) {
        unsafe { ffi::gtk_widget_set_size_request(self.get_widget(), width, height) }
    }

    fn set_no_show_all(&self, no_show_all: bool) {
        unsafe { ffi::gtk_widget_set_no_show_all(self.get_widget(), to_gboolean(no_show_all)) }
    }

    fn get_no_show_all(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_no_show_all(self.get_widget())) }
    }

    fn list_mnemonic_labels(&self) -> glib::List<Box<Self>> {
        let tmp = unsafe { ffi::gtk_widget_list_mnemonic_labels(self.get_widget()) };

        if tmp.is_null() {
            glib::List::new()
        } else {
            let old_list : glib::List<*mut ffi::C_GtkWidget> = glib::GlibContainer::wrap(tmp);
            let mut tmp_vec : glib::List<Box<Self>> = glib::List::new();

            for it in old_list.iter() {
                tmp_vec.append(Box::new(ffi::FFIWidget::wrap(*it)));
            }
            tmp_vec
        }
    }

    fn add_mnemonic_label<T: WidgetTrait>(&self, label: &T) {
        unsafe { ffi::gtk_widget_add_mnemonic_label(self.get_widget(), label.get_widget()) }
    }

    fn remove_mnemonic_label<T: WidgetTrait>(&self, label: &T) {
        unsafe { ffi::gtk_widget_remove_mnemonic_label(self.get_widget(), label.get_widget()) }
    }

    fn is_composited(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_composited(self.get_widget())) }
    }

    fn error_bell(&self) {
        unsafe { ffi::gtk_widget_error_bell(self.get_widget()) }
    }

    fn keynav_failed(&self, direction: gtk::DirectionType) -> bool {
        unsafe { to_bool(ffi::gtk_widget_keynav_failed(self.get_widget(), direction)) }
    }

    fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            let tmp = ffi::gtk_widget_get_tooltip_markup(self.get_widget());

            if tmp.is_null() {
                None
            } else {
                let ret = String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(tmp as *const i8))).to_string();

                libc::funcs::c95::stdlib::free(tmp as *mut libc::c_void);
                Some(ret)
            }
        }
    }

    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            let c_str = CString::from_slice(markup.as_bytes());

            ffi::gtk_widget_set_tooltip_markup(self.get_widget(), c_str.as_ptr() as *mut c_char);
        }
    }


    fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            let tmp = ffi::gtk_widget_get_tooltip_text(self.get_widget());

            if tmp.is_null() {
                None
            } else {
                let ret = String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&(tmp as *const i8))).to_string();

                libc::funcs::c95::stdlib::free(tmp as *mut libc::c_void);
                Some(ret)
            }
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());
            ffi::gtk_widget_set_tooltip_text(self.get_widget(), c_str.as_ptr() as *mut c_char);
        }
    }

    fn get_has_tooltip(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_has_tooltip(self.get_widget())) }
    }

    fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe { ffi::gtk_widget_set_has_tooltip(self.get_widget(), to_gboolean(has_tooltip)) }
    }

    fn trigger_tooltip_query(&self) {
        unsafe { ffi::gtk_widget_trigger_tooltip_query(self.get_widget()) }
    }

    fn get_allocated_baseline(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_allocated_baseline(self.get_widget()) }
    }

    fn get_app_paintable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_app_paintable(self.get_widget())) }
    }

    fn get_can_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_can_default(self.get_widget())) }
    }

    fn set_can_default(&self, can_default: bool) {
        unsafe { ffi::gtk_widget_set_can_default(self.get_widget(), to_gboolean(can_default)) }
    }

    fn get_can_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_can_focus(self.get_widget())) }
    }

    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::gtk_widget_set_can_focus(self.get_widget(), to_gboolean(can_focus)) }
    }

    fn get_double_buffered(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_double_buffered(self.get_widget())) }
    }

    fn get_has_window(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_has_window(self.get_widget())) }
    }

    fn set_has_window(&self, has_window: bool) {
        unsafe { ffi::gtk_widget_set_has_window(self.get_widget(), to_gboolean(has_window)) }
    }

    fn get_sensitive(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_sensitive(self.get_widget())) }
    }

    fn is_sensitive(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_sensitive(self.get_widget())) }
    }

    fn get_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_visible(self.get_widget())) }
    }

    fn is_visible(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_visible(self.get_widget())) }
    }

    fn set_visible(&self, visible: bool) {
        unsafe { ffi::gtk_widget_set_visible(self.get_widget(), to_gboolean(visible)) }
    }

    fn set_state_flags(&self, flags: gtk::StateFlags, clear: bool) {
        unsafe { ffi::gtk_widget_set_state_flags(self.get_widget(), flags, to_gboolean(clear)) }
    }

    fn unset_state_flags(&self, flags: gtk::StateFlags) {
        unsafe { ffi::gtk_widget_unset_state_flags(self.get_widget(), flags) }
    }

    fn get_state_flags(&self) -> gtk::StateFlags {
        unsafe { ffi::gtk_widget_get_state_flags(self.get_widget()) }
    }

    fn has_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_default(self.get_widget())) }
    }

    fn has_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_focus(self.get_widget())) }
    }

    fn has_visible_focus(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_visible_focus(self.get_widget())) }
    }

    fn has_grab(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_has_grab(self.get_widget())) }
    }

    fn is_drawable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_drawable(self.get_widget())) }
    }

    fn is_toplevel(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_is_toplevel(self.get_widget())) }
    }

    fn set_receives_default(&self, receives_default: bool) {
        unsafe { ffi::gtk_widget_set_receives_default(self.get_widget(), to_gboolean(receives_default)) }
    }

    fn get_receives_default(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_receives_default(self.get_widget())) }
    }

    fn set_support_multidevice(&self, support_multidevice: bool) {
        unsafe { ffi::gtk_widget_set_support_multidevice(self.get_widget(), to_gboolean(support_multidevice)) }
    }

    fn get_support_multidevice(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_support_multidevice(self.get_widget())) }
    }

    fn set_realized(&self, realized: bool) {
        unsafe { ffi::gtk_widget_set_realized(self.get_widget(), to_gboolean(realized)) }
    }

    fn get_realized(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_realized(self.get_widget())) }
    }

    fn set_mapped(&self, mapped: bool) {
        unsafe { ffi::gtk_widget_set_mapped(self.get_widget(), to_gboolean(mapped)) }
    }

    fn get_mapped(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_mapped(self.get_widget())) }
    }

    fn get_modifier_mask(&self, intent: gdk::ModifierIntent) -> gdk::ModifierType {
        unsafe { ffi::gtk_widget_get_modifier_mask(self.get_widget(), intent) }
    }

    fn set_opacity(&self, opacity: f64) {
        unsafe { ffi::gtk_widget_set_opacity(self.get_widget(), opacity) }
    }

    fn get_opacity(&self) -> f64 {
        unsafe { ffi::gtk_widget_get_opacity(self.get_widget()) }
    }

    fn set_margin_top(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_top(self.get_widget(), margin as c_int)
        }
    }

    fn set_margin_bottom(&mut self, margin: i32) -> () {
        unsafe {
            ffi::gtk_widget_set_margin_bottom(self.get_widget(), margin as c_int)
        }
    }

    fn get_margin_top(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_top(self.get_widget()) as i32
        }
    }

    fn get_margin_bottom(&mut self) -> i32 {
        unsafe {
            ffi::gtk_widget_get_margin_bottom(self.get_widget()) as i32
        }
    }

    fn get_allocated_width(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_width(self.get_widget()) as i32
        }
    }

    fn get_allocated_height(&self) -> i32 {
        unsafe{
            ffi::gtk_widget_get_allocated_height(self.get_widget()) as i32
        }
    }

    fn reset_style(&self) {
        unsafe { ffi::gtk_widget_reset_style(self.get_widget()) }
    }

    fn get_preferred_height(&self) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height(self.get_widget(), &mut minimum_height, &mut natural_height); }
        (minimum_height, natural_height)
    }

    fn get_preferred_width(&self) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_width(self.get_widget(), &mut minimum_width, &mut natural_width); }
        (minimum_width, natural_width)
    }

    fn get_preferred_height_for_width(&self, width: i32) -> (i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height_for_width(self.get_widget(), width, &mut minimum_height, &mut natural_height) };
        (minimum_height, natural_height)
    }

    fn get_preferred_width_for_height(&self, height: i32) -> (i32, i32) {
        let mut minimum_width = 0i32;
        let mut natural_width = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_width_for_height(self.get_widget(), height, &mut minimum_width, &mut natural_width) };
        (minimum_width, natural_width)
    }

    fn get_preferred_height_and_baseline_for_width(&self, width: i32) -> (i32, i32, i32, i32) {
        let mut minimum_height = 0i32;
        let mut natural_height = 0i32;
        let mut minimum_baseline = 0i32;
        let mut natural_baseline = 0i32;

        unsafe { ffi::gtk_widget_get_preferred_height_and_baseline_for_width(self.get_widget(), width, &mut minimum_height,
            &mut natural_height, &mut minimum_baseline, &mut natural_baseline) };

        (minimum_height, natural_height, minimum_baseline, natural_baseline)
    }

    fn get_request_mode(&self) -> gtk::SizeRequestMode {
        unsafe { ffi::gtk_widget_get_request_mode(self.get_widget()) }
    }

    fn get_halign(&self) -> gtk::Align {
        unsafe { ffi::gtk_widget_get_halign(self.get_widget()) }
    }

    fn set_halign(&self, align: gtk::Align) {
        unsafe { ffi::gtk_widget_set_halign(self.get_widget(), align) }
    }

    fn get_valign(&self) -> gtk::Align {
        unsafe { ffi::gtk_widget_get_valign(self.get_widget()) }
    }

    fn get_valign_with_baseline(&self) -> gtk::Align {
        unsafe { ffi::gtk_widget_get_valign_with_baseline(self.get_widget()) }
    }

    fn set_valign(&self, align: gtk::Align) {
        unsafe { ffi::gtk_widget_set_valign(self.get_widget(), align) }
    }

    fn get_margin_start(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_start(self.get_widget()) }
    }

    fn set_margin_start(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_start(self.get_widget(), margin) }
    }

    fn get_margin_end(&self) -> i32 {
        unsafe { ffi::gtk_widget_get_margin_end(self.get_widget()) }
    }

    fn set_margin_end(&self, margin: i32) {
        unsafe { ffi::gtk_widget_set_margin_end(self.get_widget(), margin) }
    }

    fn get_hexpand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_hexpand(self.get_widget())) }
    }

    fn set_hexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_hexpand(self.get_widget(), to_gboolean(expand)) }
    }

    fn get_hexpand_set(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_hexpand_set(self.get_widget())) }
    }

    fn set_hexpand_set(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_hexpand_set(self.get_widget(), to_gboolean(expand)) }
    }

    fn get_vexpand(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_vexpand(self.get_widget())) }
    }

    fn set_vexpand(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_vexpand(self.get_widget(), to_gboolean(expand)) }
    }

    fn get_vexpand_set(&self) -> bool {
        unsafe { to_bool(ffi::gtk_widget_get_vexpand_set(self.get_widget())) }
    }

    fn set_vexpand_set(&self, expand: bool) {
        unsafe { ffi::gtk_widget_set_vexpand_set(self.get_widget(), to_gboolean(expand)) }
    }

    fn queue_compute_expand(&self) {
        unsafe { ffi::gtk_widget_queue_compute_expand(self.get_widget()) }
    }

    fn compute_expand(&self, orientation: gtk::Orientation) -> bool {
        unsafe { to_bool(ffi::gtk_widget_compute_expand(self.get_widget(), orientation)) }
    }

    fn init_template(&self) {
        unsafe { ffi::gtk_widget_init_template(self.get_widget()) }
    }

    fn thaw_child_notify(&self) {
        unsafe { ffi::gtk_widget_thaw_child_notify(self.get_widget()) }
    }

    fn freeze_child_notify(&self) {
        unsafe { ffi::gtk_widget_freeze_child_notify(self.get_widget()) }
    }

    fn child_notify(&self, child_property: &str) {
        unsafe {
            let c_str = CString::from_slice(child_property.as_bytes());

            ffi::gtk_widget_child_notify(self.get_widget(), c_str.as_ptr())
        }
    }

    fn destroy(&self) {
        unsafe { ffi::gtk_widget_destroy(self.get_widget()) }
    }

    fn destroyed(&self, other: &Self) {
        let mut tmp = other.get_widget();

        unsafe { ffi::gtk_widget_destroyed(self.get_widget(), &mut tmp) }
    }
}
