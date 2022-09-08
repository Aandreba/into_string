#![feature(min_specialization)]
#![no_std]

pub(crate) extern crate alloc;
use alloc::{borrow::Cow, string::{String, ToString}};

/// A similar trait to [`ToString`], but avoiding an extra allocation when applied to a [`String`]
pub trait IntoString {
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

impl<'a> IntoCowStr<'a> for &'a str {
    #[inline(always)]
    fn into_cow_str (self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}