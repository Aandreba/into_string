#![feature(min_specialization)]

use std::borrow::Cow;

/// A similar trait to [`ToString`], but avoiding extra allocations when applied to a [`String`]
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

/// Helper trait to turn [`ToString`]-able values into strings, avoiding an allocation if the type is already a [`String`],
/// and avoiding a new allocation if it's a [`&str`](str)
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