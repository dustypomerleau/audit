/// Creates bounded integer newtypes from comma-separated tuples containing:
///
/// `(<newtype_name>, <integer_type>, <integer_range>, <optional_rem/modulo_value>)`.
///
/// The generated type implements [`Bounded<integer_type>`](crate::bounded::Bounded),
/// [`AsRef<integer_type>`], and a `Self::new()` constructor with bounds checking.
// todo: replace the bounded macro with a derive macro that takes range and rem as arguments.
macro_rules! bounded {
    ($(($name:ident, $type:ty, $range:expr $(, $rem:literal)? $(,)?)),+ $(,)?) => (
        $(
            /// A bounded integer newtype generated using the [`bounded`] macro.
            /// The generated type implements [`Bounded<integer_type>`](crate::bounded::Bounded),
            /// [`AsRef<integer_type>`], and a `Self::new()` constructor with bounds checking.
            #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
            pub struct $name($type);

            impl AsRef<$type> for $name {
                fn as_ref(&self) -> &$type {
                    &self.0
                }
            }

            impl $crate::bounded::Bounded<$type> for $name {
                fn inner(&self) -> $type {
                    self.0
                }

                fn new(value: $type) -> Result<Self, $crate::error::AppError> {
                    if ($range).contains(&value) $(&& value % $rem == 0)? {
                        Ok($name(value))
                    } else {
                        Err($crate::error::AppError::Bounds(format!("{value:?}")))
                    }
                }

                fn range() -> impl RangeBounds<$type> {
                    $range
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.inner())
                }
            }
        )+
    )
}

macro_rules! some_or_empty {
    ($($id:ident),+) => (let ($($id),+) = ($($crate::db::some_or_empty($id),)+);)
}
