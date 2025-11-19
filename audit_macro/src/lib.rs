use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;
use syn::FieldsUnnamed;
use syn::Meta;
use syn::Token;
use syn::Type;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

#[proc_macro_derive(RangeBounded, attributes(bounded))]
pub fn range_bounded(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let (mut ty, mut range, mut rem) =
        (Type::Verbatim(proc_macro2::TokenStream::new()), None, None);

    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
            ..
        }) => {
            if unnamed.len() == 1 {
                let field = unnamed
                    .first()
                    .expect("only implemented for singleton tuple structs");

                field.attrs.iter().for_each(|attr| {
                    if attr.path().is_ident("bounded") {
                        ty = field.ty.clone();

                        let nested = attr
                            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                            .expect("nested attributes should be comma-separated");

                        for meta in nested {
                            if let Meta::NameValue(ref mnv) = meta {
                                match mnv.path.get_ident().unwrap().to_string().as_str() {
                                    "range" => {
                                        range = Some(mnv.value.clone());
                                    }

                                    "rem" => {
                                        rem = Some(mnv.value.clone());
                                    }

                                    _ => unimplemented!("unrecognized key-value pair"),
                                }
                            }
                        }
                    }
                })
            }
        }

        _ => unimplemented!("only implemented for singleton tuple structs"),
    }

    let name = ast.ident;

    let predicate = match (&range, &rem) {
        (Some(range), Some(rem)) => {
            quote! { (#range).contains(&value) && value % #rem == 0 as #ty }
        }

        (Some(range), None) => {
            quote! { (#range).contains(&value)}
        }

        (None, Some(rem)) => {
            quote! { value % #rem == 0 as #ty }
        }

        (None, None) => quote! { true },
    };

    let rem = if let Some(rem) = rem {
        quote! { Some(#rem) }
    } else {
        quote! { None }
    };

    let output = quote! {
        impl crate::bounded::Bounded for #name {
            type Idx = #ty;

            fn inner(&self) -> Self::Idx {
                self.0
            }

            fn new(value: Self::Idx) -> ::core::result::Result<Self, crate::error::AppError> {
                if #predicate {
                    Ok(#name(value))
                } else {
                    Err(crate::error::AppError::Bounds(format!("{value:?}")))
                }
            }

            #[cfg(feature = "ssr")]
            fn range() -> impl ::std::ops::RangeBounds<Self::Idx> + ::rand::distr::uniform::SampleRange<Self::Idx> {
                #range
            }

            fn rem() -> Option<Self::Idx> {
                #rem
            }
        }

        impl ::core::convert::AsRef<#ty> for #name {
            fn as_ref(&self) -> &#ty {
                &self.0
            }
        }

        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.inner())
            }
        }
    };

    output.into()
}
