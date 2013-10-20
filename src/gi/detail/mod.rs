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
use std::ptr;

mod native;

pub struct GIAttributeIter {
    priv data: glib::gpointer,
    priv data2: glib::gpointer,
    priv data3: glib::gpointer,
    priv data4: glib::gpointer
}

impl GIAttributeIter {
    pub fn new() -> GIAttributeIter {
        GIAttributeIter {
            data: ptr::mut_null(),
            data2: ptr::mut_null(),
            data3: ptr::mut_null(),
            data4: ptr::mut_null()
        }
    }
}

pub type GIInfoType = libc::c_int;

pub struct GIBaseInfo;

pub unsafe fn g_base_info_ref(info: *mut ::detail::GIBaseInfo) -> *mut ::detail::GIBaseInfo {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_ref(info)
}

pub unsafe fn g_base_info_unref(info: *mut ::detail::GIBaseInfo) {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_unref(info)
}

pub unsafe fn g_base_info_get_type(info: *mut ::detail::GIBaseInfo) -> ::InfoType {
    #[fixed_stack_segment]; #[inline(never)];
    match native::g_base_info_get_type(info) {
        0 => ::Invalid,
        1 => ::Function,
        2 => ::Callback,
        3 => ::Struct,
        4 => ::Boxed,
        5 => ::Enum,
        6 => ::Flags,
        7 => ::Object,
        8 => ::Interface,
        9 => ::Constant,
        10 => ::Invalid,
        11 => ::Union,
        12 => ::Value,
        13 => ::Signal,
        14 => ::Vfunc,
        15 => ::Property,
        16 => ::Field,
        17 => ::Arg,
        18 => ::Type,
        19 => ::Unresolved,
        v => fail2!("unknown GIInfoType {}", v)
    }
}

pub unsafe fn g_base_info_get_name(info: *mut ::detail::GIBaseInfo) -> *glib::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_get_name(info)
}

pub unsafe fn g_base_info_get_namespace(info: *mut ::detail::GIBaseInfo) -> *glib::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_get_namespace(info)
}

pub unsafe fn g_base_info_is_deprecated(info: *mut ::detail::GIBaseInfo) -> bool {
    #[fixed_stack_segment]; #[inline(never)];
    if native::g_base_info_is_deprecated(info) == 0 {
        false
    } else {
        true
    }
}

pub unsafe fn g_base_info_get_attribute(info: *mut ::detail::GIBaseInfo, name: *glib::gchar) -> *glib::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_get_attribute(info, name)
}

pub unsafe fn g_base_info_iterate_attributes(info: *mut ::detail::GIBaseInfo, iterator: *mut ::detail::GIAttributeIter, name: *mut *libc::c_char, value: *mut *libc::c_char) -> bool {
    #[fixed_stack_segment]; #[inline(never)];
    if native::g_base_info_iterate_attributes(info, iterator, name, value) == 0 {
        false
    } else {
        true
    }
}

pub unsafe fn g_base_info_get_container(info: *mut ::detail::GIBaseInfo) -> *mut ::detail::GIBaseInfo {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_get_container(info)
}

pub unsafe fn g_base_info_get_typelib(info: *mut ::detail::GIBaseInfo) -> *mut ::detail::GITypelib {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_base_info_get_typelib(info)
}

pub unsafe fn g_base_info_equal(info1: *mut ::detail::GIBaseInfo, info2: *mut ::detail::GIBaseInfo) -> bool {
    #[fixed_stack_segment]; #[inline(never)];
    if native::g_base_info_equal(info1, info2) == 0 {
        false
    } else {
        true
    }
}

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

pub unsafe fn g_irepository_require(repository: *mut ::detail::GIRepository, namespace: *glib::gchar, version: *glib::gchar, flags: &[::RepositoryLoadFlag], error: *mut *mut GError) -> *mut GITypelib {
    #[fixed_stack_segment]; #[inline(never)];
    let converted_flags = flags.iter().fold(0, |converted_flags, &flag| converted_flags | (flag as GIRepositoryLoadFlags));
    native::g_irepository_require(repository, namespace, version, converted_flags, error)
}

pub unsafe fn g_irepository_get_loaded_namespaces(repository: *mut GIRepository) -> *mut *mut glib::gchar {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_loaded_namespaces(repository)
}

pub unsafe fn g_irepository_get_n_infos(repository: *mut ::detail::GIRepository, namespace: *glib::gchar) -> glib::gint {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_n_infos(repository, namespace)
}

pub unsafe fn g_irepository_get_info(repository: *mut ::detail::GIRepository, namespace: *glib::gchar, index: glib::gint) -> *mut ::detail::GIBaseInfo {
    #[fixed_stack_segment]; #[inline(never)];
    native::g_irepository_get_info(repository, namespace, index)
}
