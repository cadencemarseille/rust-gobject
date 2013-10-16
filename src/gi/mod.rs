// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "gi", vers = "0.1")];
#[crate_type = "lib"];

extern mod glib;
extern mod gobject;

use std::ptr;

mod detail;

pub struct Repository {
    priv ptr: *detail::GIRepository,
    priv owns: bool
}

impl Repository {
    pub fn default() -> Repository {
        Repository {
            ptr: detail::g_irepository_get_default() as *detail::GIRepository,
            owns: false
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        if self.owns {
            gobject::detail::g_object_unref(self.ptr as *mut detail::GIRepository as glib::gpointer);
        }
        self.ptr = ptr::null();
    }
}
