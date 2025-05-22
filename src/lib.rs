#![feature(lock_value_accessors)]
#![feature(new_range_api)]
#![feature(string_remove_matches)]
#![feature(type_changing_struct_update)]

//! The [`audit`](self) library provides tools for analyzing and plotting the results of cataract
//! surgery. Powers that would typically be represented in diopters (1 m e-1) (refractions, IOLs,
//! refractive targets) are instead represented in centidiopters (1 hm^-1) to allow integer math.
//!
//! By convention, we use leading zeros for values less than 100 centidiopters, as a reminder that
//! their diopter representations are < 1. This means that most representations of power values
//! will be either 3 or 4 digits.

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

            impl Bounded<$type> for $name {
                fn range() -> impl RangeBounds<$type> {
                    $range
                }

                fn inner(&self) -> $type {
                    self.0
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.inner())
                }
            }

            impl $name {
                pub fn new(value: $type) -> Result<Self, crate::case::BoundsError> {
                    if ($range).contains(&value) $(&& value % $rem == 0)? {
                        Ok($name(value))
                    } else {
                        Err(crate::case::BoundsError(format!("{value:?}")))
                    }
                }

            }
        )+
    )
}

macro_rules! some_or_empty {
    ($($id:ident),+) => (let ($($id),+) = ($(some_or_empty($id),)+);)
}

// #[cfg(feature = "ssr")] pub mod fileserv;
#[cfg(feature = "ssr")] pub mod auth;
pub mod biometry;
pub mod bounded;
pub mod case;
pub mod components;
pub mod cyl;
pub mod db;
pub mod email;
pub mod iol;
// pub mod plots;
pub mod refraction;
pub mod routes;
pub mod sca;
pub mod sia;
#[cfg(feature = "ssr")] pub mod state;
pub mod surgeon;
pub mod target;
pub mod va;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::routes::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
