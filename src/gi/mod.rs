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

use glib::detail::error::GError;
use glib::strfuncs::Strdupv;
use std::c_str::CString;
use std::cast;
use std::libc;
use std::option::Option;
use std::ptr;
use std::result::Result;
use std::str;
use std::unstable::raw::Slice;

mod detail;

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

pub struct AttributeIter {
    priv ptr: *mut detail::GIBaseInfo,
    priv it: GIAttributeIter
}

impl Drop for AttributeIter {
    fn drop(&mut self) {
        unsafe {
            detail::g_base_info_unref(self.ptr);
            self.ptr = ptr::mut_null();
        }
    }
}

impl Iterator<(~str, ~str)> for AttributeIter {
    fn next(&mut self) -> Option<(~str, ~str)> {
        unsafe {
            let mut name_c_str: *libc::c_char = ptr::null();
            let mut value_c_str: *libc::c_char = ptr::null();
            if !detail::g_base_info_iterate_attributes(self.ptr, &mut self.it, &mut name_c_str, &mut value_c_str) {
                None
            } else {
                Some((str::raw::from_c_str(name_c_str), str::raw::from_c_str(value_c_str)))
            }
        }
    }
}

#[deriving(ToStr)]
pub enum InfoType {
    Invalid = 0,
    Function = 1,
    Callback = 2,
    Struct = 3,
    Boxed = 4,
    Enum = 5,
    Flags = 6,
    Object = 7,
    Interface = 8,
    Constant = 9,
    // Invalid0
    Union = 11,
    Value = 12,
    Signal = 13,
    Vfunc = 14,
    Property = 15,
    Field = 16,
    Arg = 17,
    Type = 18,
    Unresolved = 19
}

pub struct BaseInfo {
    priv ptr: *mut detail::GIBaseInfo,
    priv owns: bool
}

impl BaseInfo {
    pub fn type_(&self) -> InfoType {
        unsafe {
            detail::g_base_info_get_type(self.ptr)
        }
    }

    pub fn name(&self) -> Option<~str> {
        unsafe {
            let name_c_str = detail::g_base_info_get_name(self.ptr);
            if ptr::is_null(name_c_str) {
                None
            } else {
                Some(str::raw::from_c_str(name_c_str))
            }
        }
    }

    #[inline]
    pub unsafe fn raw_name(&self) -> Option<&str> {
        let name_c_str = detail::g_base_info_get_name(self.ptr);
        if ptr::is_null(name_c_str) {
            None
        } else {
            let mut name_len = 0u;
            while *ptr::offset(name_c_str, name_len as int) != 0 {
                name_len += 1;
            }
            Some(cast::transmute(Slice {
                data: name_c_str,
                len: name_len
            }))
        }
    }

    pub fn namespace(&self) -> ~str {
        unsafe {
            let namespace_c_str = detail::g_base_info_get_namespace(self.ptr);
            str::raw::from_c_str(namespace_c_str)
        }
    }

    pub fn is_deprecated(&self) -> bool {
        unsafe {
            detail::g_base_info_is_deprecated(self.ptr)
        }
    }

    pub fn attribute(&self, name: &str) -> Option<~str> {
        unsafe {
            do name.with_c_str |name_c_str| {
                let attribute_c_str = detail::g_base_info_get_attribute(self.ptr, name_c_str);
                if ptr::is_null(attribute_c_str) {
                    None
                } else {
                    Some(str::raw::from_c_str(attribute_c_str))
                }
            }
        }
    }

    pub fn attribute_iter(&self) -> AttributeIter {
        unsafe {
            AttributeIter {
                ptr: { detail::g_base_info_ref(self.ptr); self.ptr },
                it: GIAttributeIter::new()
            }
        }
    }
}

impl Clone for BaseInfo {
    fn clone(&self) -> BaseInfo {
        unsafe {
            BaseInfo {
                ptr: detail::g_base_info_ref(self.ptr),
                owns: true
            }
        }
    }
}

impl Drop for BaseInfo {
    fn drop(&mut self) {
        unsafe {
            if self.owns {
                detail::g_base_info_unref(self.ptr);
            }
            self.ptr = ptr::mut_null();
            self.owns = false;
        }
    }
}

impl Eq for BaseInfo {
    fn eq(&self, other: &BaseInfo) -> bool {
        unsafe {
            detail::g_base_info_equal(self.ptr, other.ptr)
        }
    }
}

pub struct Typelib {
    priv ptr: *mut detail::GITypelib,
    priv owns: bool
}

impl Drop for Typelib {
    fn drop(&mut self) {
        unsafe {
            if self.owns {
                detail::g_typelib_free(self.ptr);
            }
            self.ptr = ptr::mut_null();
            self.owns = false;
        }
    }
}

pub struct Repository {
    priv ptr: *mut detail::GIRepository,
    priv owns: bool
}

#[deriving(ToStr)]
pub enum RepositoryLoadFlag {
    Lazy = 1
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

    pub fn info(&self, namespace_cstring: &CString, index: glib::gint) -> BaseInfo {
        unsafe {
            do namespace_cstring.with_ref |namespace_c_str| {
                BaseInfo {
                    ptr: detail::g_irepository_get_info(self.ptr, namespace_c_str, index),
                    owns: true
                }
            }
        }
    }

    pub fn loaded_namespaces(&self) -> ~[~str] {
        unsafe {
            let mut res: ~[~str] = ~[];
            let mut namespaces_strdupv = Strdupv::new(detail::g_irepository_get_loaded_namespaces(self.ptr));
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

    pub fn n_infos(&self, namespace_cstring: &CString) -> glib::gint {
        unsafe {
            do namespace_cstring.with_ref |namespace_c_str| {
                detail::g_irepository_get_n_infos(self.ptr, namespace_c_str)
            }
        }
    }

    pub fn require(&self, namespace: &str, opt_version: Option<&str>, flags: &[RepositoryLoadFlag]) -> Result<Typelib, glib::Error> {
        unsafe {
            do namespace.with_c_str |namespace_c_str| {
                let mut error: *mut GError = ptr::mut_null();
                let typelib_ptr = match opt_version {
                    None => detail::g_irepository_require(self.ptr, namespace_c_str, ptr::null(), flags, &mut error),
                    Some(version) => {
                        do version.with_c_str |version_c_str| {
                            detail::g_irepository_require(self.ptr, namespace_c_str, version_c_str, flags, &mut error)
                        }
                    }
                };
                if ptr::is_null(typelib_ptr) {
                    Err(glib::Error::new(error))
                } else {
                    Ok(Typelib {
                        ptr: typelib_ptr,
                        owns: false
                    })
                }
            }
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
