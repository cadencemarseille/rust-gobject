// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gobject::detail::GObject;

mod native;

pub struct GIRepositoryPrivate;

pub struct GIRepository {
    parent: GObject,

    priv priv_: *mut GIRepositoryPrivate
}

pub unsafe fn g_irepository_get_default() -> *mut GIRepository {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_default()
}
