use std::str::FromStr;

use crate::{
    error::InquireResult,
    selected_option::{FromSelectedOption, SelectedOption},
};

pub trait InquireEnumVariants {
    fn get_variants() -> &'static [&'static str];
}

#[cfg(feature = "strum")]
impl<T> InquireEnumVariants for T
where
    T: strum::VariantNames,
{
    fn get_variants() -> &'static [&'static str] {
        T::VARIANTS
    }
}
