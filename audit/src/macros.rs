/// Creates bounded integer newtypes from comma-separated tuples containing:
///
/// `(<newtype_name>, <numeric_type>, <bounded_range>, <optional_rem/modulo_value>)`.
///
/// The generated type implements [`Bounded<integer_type>`](crate::bounded::Bounded),
/// [`AsRef<integer_type>`], and a `Self::new()` constructor with bounds checking.
// TODO: replace the bounded macro with a derive macro that takes range and rem as arguments.
#[macro_export]
macro_rules! range_bounded {
    ($(($name:ident, $type:ty, $range:expr $(, $rem:literal)? $(,)?)),+ $(,)?) => (
        $(
            /// A bounded integer newtype generated using the [`range_bounded!`] macro.
            /// The generated type implements [`Bounded<integer_type>`](crate::bounded::Bounded),
            /// [`AsRef<integer_type>`], and a `Self::new()` constructor with bounds checking. It
            /// also provides easy newtype mocking via [`Mock`](crate::mock::Mock), for testing
            /// purposes.
            #[derive(
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::core::fmt::Debug,
                ::core::default::Default,
                ::serde::Deserialize,
                ::core::cmp::PartialEq,
                ::serde::Serialize
            )]
            pub struct $name($type);

            impl ::core::convert::AsRef<$type> for $name {
                fn as_ref(&self) -> &$type {
                    &self.0
                }
            }

            impl $crate::bounded::Bounded for $name {
                type Idx = $type;

                fn inner(&self) -> Self::Idx {
                    self.0
                }

                fn new(value: Self::Idx) -> ::core::result::Result<Self, $crate::error::AppError> {
                    if ($range).contains(&value) $(&& value % $rem == 0)? {
                        Ok($name(value))
                    } else {
                        Err($crate::error::AppError::Bounds(format!("{value:?}")))
                    }
                }

                #[cfg(feature = "ssr")]
                fn range() -> impl ::std::ops::RangeBounds<$type> {
                    $range
                }
            }

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}", self.inner())
                }
            }
        )+
    )
}

#[macro_export]
macro_rules! some_or_empty {
    ($($id:ident),+) => (let ($($id),+) = ($($crate::db::some_or_empty($id),)+);)
}

#[cfg(test)]
mod tests {
    use audit_macro::RangeBounded;

    use crate::bounded::Bounded;

    #[test]
    fn derives_range_bounded() {
        #[derive(RangeBounded)]
        struct TestStruct(#[bounded(range = 0..50, rem = 5)] u32);

        TestStruct::new(25).unwrap();
    }
}
