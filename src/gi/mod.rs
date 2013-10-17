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

use glib::strfuncs::Strdupv;
use std::ptr;
use std::str;

mod detail;

pub struct Repository {
    priv ptr: *mut detail::GIRepository,
    priv owns: bool
}

impl Repository {
    pub fn default() -> Repository {
        unsafe {
            Repository {
                ptr: detail::g_irepository_get_default(),
                owns: false
            }
        }
    }

    pub fn loaded_namespaces(&self) -> ~[~str] {
        unsafe {
            let mut res: ~[~str] = ~[];
            let mut namespaces_strdupv = Strdupv::new(detail::g_irepository_get_loaded_namespaces(self.ptr as *detail::GIRepository));
            do namespaces_strdupv.with_mut_ptr |namespaces_array| {
                let mut p = namespaces_array;
                loop {
                    let namespace_c_str = ptr::read_ptr(p);
                    if ptr::is_null(namespace_c_str) {
                        break;
                    }
                    res.push(str::raw::from_c_str(namespace_c_str as *glib::gchar));
                    p = ptr::mut_offset(p, 1);
                }
            };
            res
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        unsafe {
            if self.owns {
                gobject::detail::g_object_unref(self.ptr as glib::gpointer);
            }
            self.ptr = ptr::mut_null();
            self.owns = false;
        }
    }
}
