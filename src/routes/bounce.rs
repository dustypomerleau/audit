use leptos::prelude::{IntoView, Params, Read, component};
use leptos_axum::redirect;
use leptos_router::{
    hooks::{use_navigate, use_query},
    params::Params,
};

/// The redirects from the client to an encoded URL in the query params. For our current purposes,
/// the only encoded character that can be contained in the query string is "/" (%2F). The
/// component is useful when you want to redirect from the server, but you need client-side cookies
/// to be sent with the request (for example, to prove authentication).
#[component]
pub fn Bounce() -> impl IntoView {
    #[derive(Debug, Params, PartialEq)]
    struct RedirectQuery {
        redirect: String,
    }

    let navigate = use_navigate();

    if let Ok(redirect_query) = use_query::<RedirectQuery>().read().as_ref() {
        // This is a very simplistic parsing, but since we are only encoding one character, it
        // saves a dep. If the parsing requires more characters in future, we will have to add a
        // URL-encoding library.
        let path = redirect_query.redirect.replace("%2F", "/");
        navigate(&path, Default::default());
    } else {
        navigate("/", Default::default());
    }
}
