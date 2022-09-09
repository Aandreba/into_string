#![cfg_attr(not(feature = "std"), no_std, feature(core_c_str, alloc_c_string))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "max", feature(specialization), allow(incomplete_features))]
#![cfg_attr(not(feature = "max"), feature(min_specialization))]
#![feature(box_into_inner)]
#![doc = include_str!("../README.md")]

pub(crate) extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::ffi::{CStr, CString};
    } else {
        use core::{ffi::CStr};
        use alloc::ffi::CString;
    }
}

use alloc::{boxed::Box, borrow::Cow, string::{String, ToString}};

/// A similar trait to [`ToString`], but avoiding an extra allocation when applied to a [`String`]
pub trait IntoString {
    /// Converts value into string.
    fn into_string (self) -> String;    
}

impl<T: ToString> IntoString for &T {
    #[inline(always)]
    default fn into_string (self) -> String {
        T::to_string(self)
    }
}

impl IntoString for String {
    #[inline(always)]
    fn into_string (self) -> String {
        self
    }
}

impl IntoString for Cow<'_, str> {
    #[inline(always)]
    fn into_string (self) -> String {
        self.into_owned()
    }
}

impl IntoString for Box<str> {
    #[inline(always)]
    fn into_string (self) -> String {
        Box::<str>::into(self)
    }
}

impl<T: IntoString> IntoString for Box<T> {
    #[inline(always)]
    fn into_string (self) -> String {
        Box::into_inner(self).into_string()
    }
}

/// A similar trait to [`ToString`], but for [`CStr`]s, and avoiding an extra allocation when applied to a [`CString`]
pub trait IntoCString {
    /// Converts value into a C string.
    fn into_c_string (self) -> CString;
}

impl<T: AsRef<CStr>> IntoCString for T {
    #[inline(always)]
    default fn into_c_string (self) -> CString {
        unsafe {
            CString::from_vec_with_nul_unchecked(self.as_ref().to_bytes_with_nul().to_vec())
        }
    }
}

impl IntoCString for &CStr {
    #[inline(always)]
    fn into_c_string (self) -> CString {
        unsafe {
            CString::from_vec_with_nul_unchecked(self.to_bytes_with_nul().to_vec())
        }
    }
}

impl IntoCString for CString {
    #[inline(always)]
    fn into_c_string (self) -> CString {
        self
    }
}

impl IntoCString for Cow<'_, CStr> {
    #[inline(always)]
    fn into_c_string (self) -> CString {
        self.into_owned()
    }
}

impl IntoCString for Box<CStr> {
    #[inline(always)]
    fn into_c_string (self) -> CString {
        Box::<CStr>::into(self)
    }
}

/// A similar trait to [`ToString`], but for [`OsStr`](std::ffi::OsStr)s, and avoiding an extra allocation when applied to a [`OsString`](std::ffi::OsString).
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
pub trait IntoOsString {
    /// Converts value into an OS string.
    fn into_os_string (self) -> std::ffi::OsString;
}

#[cfg(feature = "std")]
impl<T: AsRef<std::ffi::OsStr>> IntoOsString for T {
    #[inline(always)]
    default fn into_os_string (self) -> std::ffi::OsString {
        std::ffi::OsString::from(self.as_ref())
    }
}

#[cfg(feature = "std")]
impl IntoOsString for &std::ffi::OsStr {
    #[inline(always)]
    fn into_os_string (self) -> std::ffi::OsString {
        std::ffi::OsString::from(self)
    }
}

#[cfg(feature = "std")]
impl IntoOsString for std::ffi::OsString {
    #[inline(always)]
    fn into_os_string (self) -> std::ffi::OsString {
        self
    }
}

#[cfg(feature = "std")]
impl IntoOsString for Cow<'_, std::ffi::OsStr> {
    #[inline(always)]
    fn into_os_string (self) -> std::ffi::OsString {
        self.into_owned()
    }
}

#[cfg(feature = "std")]
impl IntoOsString for Box<std::ffi::OsStr> {
    #[inline(always)]
    fn into_os_string (self) -> std::ffi::OsString {
        Box::<std::ffi::OsStr>::into(self)
    }
}

/// A similar trait to [`ToString`], but for [`Path`](std::path::Path)s, and avoiding an extra allocation when applied to a [`PathBuf`](std::path::PathBuf).
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
pub trait IntoPathBuf {
    /// Converts value into an path buffer.
    fn into_path_buf (self) -> std::path::PathBuf;
}

#[cfg(feature = "std")]
impl<T: AsRef<std::path::Path>> IntoPathBuf for T {
    #[inline(always)]
    default fn into_path_buf (self) -> std::path::PathBuf {
        std::path::PathBuf::from(self.as_ref())
    }
}

#[cfg(feature = "std")]
impl IntoPathBuf for &std::path::Path {
    fn into_path_buf (self) -> std::path::PathBuf {
        std::path::PathBuf::from(self)
    }
}

#[cfg(feature = "std")]
impl IntoPathBuf for std::path::PathBuf {
    #[inline(always)]
    fn into_path_buf (self) -> std::path::PathBuf {
        self
    }
}

#[cfg(feature = "std")]
impl IntoPathBuf for Cow<'_, std::path::Path> {
    #[inline(always)]
    fn into_path_buf (self) -> std::path::PathBuf {
        self.into_owned()
    }
}

#[cfg(feature = "std")]
impl IntoPathBuf for Box<std::path::Path> {
    #[inline(always)]
    fn into_path_buf (self) -> std::path::PathBuf {
        Box::<std::path::Path>::into(self)
    }
}

/// Helper trait to turn [`ToString`]-able values into strings, avoiding an allocation for [`String`]s and [`&str`](str)
pub trait IntoCowStr<'a> {
    fn into_cow_str (self) -> Cow<'a, str>;
}

impl<'a, T: IntoString> IntoCowStr<'a> for T {
    #[inline(always)]
    default fn into_cow_str (self) -> Cow<'a, str> {
        Cow::Owned(self.into_string())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "max")))]
#[cfg(feature = "max")]
impl<'a> IntoCowStr<'a> for Cow<'a, str> {
    #[inline(always)]
    fn into_cow_str (self) -> Cow<'a, str> {
        self
    }
}

impl<'a> IntoCowStr<'a> for &'a str {
    #[inline(always)]
    fn into_cow_str (self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}