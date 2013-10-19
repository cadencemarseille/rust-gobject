// Copyright 2013 The rust-gobject authors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gobject::detail::GObject;

use glib;
use glib::detail::error::GError;
use std::libc;

mod native;

pub struct GITypelib;

pub unsafe fn g_typelib_free(typelib: *mut GITypelib) {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_typelib_free(typelib);
}

pub struct GIRepositoryPrivate;

pub struct GIRepository {
    parent: GObject,

    priv priv_: *mut GIRepositoryPrivate
}

pub type GIRepositoryLoadFlags = libc::c_int;

pub unsafe fn g_irepository_get_default() -> *mut GIRepository {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_default()
}

pub unsafe fn g_irepository_get_loaded_namespaces(repository: *GIRepository) -> *mut *mut glib::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_loaded_namespaces(repository)
}

pub unsafe fn g_irepository_require(repository: *mut ::detail::GIRepository, namespace_: *glib::gchar, version: *glib::gchar, flags: &[::RepositoryLoadFlag], error: *mut *mut GError) -> *mut GITypelib {
    #[fixed_stack_segment]; #[inline(never)];
    let converted_flags = flags.iter().fold(0, |converted_flags, &flag| (converted_flags | (flag as GIRepositoryLoadFlags) as GIRepositoryLoadFlags));
    native::g_irepository_require(repository, namespace_, version, converted_flags, error)
}
