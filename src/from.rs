use core::str::FromStr;
use alloc::string::String;

pub trait FromString: Sized {
    type Err;
    fn from_string (s: String) -> Result<Self, Self::Err>;
}

impl<T: FromStr> FromString for T {
    type Err = <T as FromStr>::Err;

    #[inline]
    default fn from_string (s: String) -> Result<Self, Self::Err> {
        return FromStr::from_str(&s)
    }
}

impl FromString for String {
    #[inline]
    fn from_string (s: String) -> Result<Self, Self::Err> {
        return Ok(s)
    }
}