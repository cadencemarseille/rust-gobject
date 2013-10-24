// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "gobject", vers = "0.1")];
#[crate_type = "lib"];

extern mod glib;

pub mod detail;

pub type GType = glib::gsize;

pub struct GData;

pub struct GTypeClass {
    priv g_type: GType
}

pub struct GObject {
    g_type_instance: GTypeClass,

    priv /*volatile*/ ref_count: glib::guint,
    priv qdata: *mut GData
}

pub enum Object {
    MyGObject(GObject),
    GObjectPtr {
        priv ptr: *mut GObject,
        priv owns: bool
    }
}
