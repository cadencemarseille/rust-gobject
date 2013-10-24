// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use std::ptr;

mod native;

pub unsafe fn g_object_ref(object: glib::gpointer) -> glib::gpointer {
    #[fixed_stack_segment]; #[inline(never)];
    assert!(ptr::is_not_null(object));
    native::g_object_ref(object)
}

pub unsafe fn g_object_ref_sink(object: glib::gpointer) -> glib::gpointer {
    #[fixed_stack_segment]; #[inline(never)];
    assert!(ptr::is_not_null(object));
    native::g_object_ref_sink(object)
}

pub unsafe fn g_object_unref(object: glib::gpointer) {
    #[fixed_stack_segment]; #[inline(never)];
    assert!(ptr::is_not_null(object));
    native::g_object_unref(object)
}
