// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use Action;
use Component;
use Document;
use EditableText;
use Hypertext;
use Image;
use Object;
use Selection;
use Table;
use TableCell;
use Text;
use Value;
use Window;

glib_wrapper! {
    pub struct NoOpObject(Object<atk_sys::AtkNoOpObject, atk_sys::AtkNoOpObjectClass>) @extends Object, @implements Action, Component, Document, EditableText, Hypertext, Image, Selection, Table, TableCell, Text, Value, Window;

    match fn {
        get_type => || atk_sys::atk_no_op_object_get_type(),
    }
}

impl NoOpObject {
    pub fn new<P: IsA<glib::Object>>(obj: &P) -> NoOpObject {
        assert_initialized_main_thread!();
        unsafe {
            Object::from_glib_full(atk_sys::atk_no_op_object_new(obj.as_ref().to_glib_none().0))
                .unsafe_cast()
        }
    }
}

pub const NONE_NO_OP_OBJECT: Option<&NoOpObject> = None;

impl fmt::Display for NoOpObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NoOpObject")
    }
}