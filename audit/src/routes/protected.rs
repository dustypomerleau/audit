use leptos::either::Either;
use leptos::prelude::IntoView;
use leptos::prelude::Resource;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Suspend;
use leptos::prelude::Suspense;
use leptos::prelude::component;
use leptos::prelude::provide_context;
use leptos::prelude::server;
#[cfg(feature = "ssr")] use leptos::prelude::use_context;
use leptos::prelude::view;
#[cfg(feature = "ssr")] use leptos_axum::redirect;
use leptos_router::components::Outlet;
#[cfg(feature = "ssr")] use sqlx::query_as;

#[cfg(feature = "ssr")] use crate::auth::get_jwt_cookie;
use crate::components::SignedOut;
#[cfg(feature = "ssr")] use crate::db::db;
use crate::error::AppError;
use crate::model::Axis;
use crate::model::Email;
use crate::model::Focus;
use crate::model::Formula;
use crate::model::Main;
use crate::model::QuerySurgeon;
use crate::model::SiaPower;
use crate::model::Surgeon;
use crate::model::ToricPower;
#[cfg(feature = "ssr")] use crate::state::AppState;

#[component]
pub fn Protected() -> impl IntoView {
    let current_surgeon = RwSignal::<Option<Surgeon>>::new(None);
    let surgeon_resource = Resource::new_blocking(|| (), |_| get_authorized_surgeon());

    let outlet_if_authorized = Suspend::new(async move {
        if let Ok(Some(surgeon)) = surgeon_resource.await {
            current_surgeon.set(Some(surgeon));
            provide_context(current_surgeon);

            Either::Left(view! { <Outlet /> })
        } else {
            Either::Right(
                view! {"debug info: this signed out is the one inside the `outlet if authorized` `Suspend`" <br/> <SignedOut /> },
            )
        }
    });

    view! {
        <Suspense fallback=move || {
            view! { "Checking authorization for the current surgeon..." }
        }>{outlet_if_authorized}</Suspense>
    }
}

#[server]
pub async fn get_authorized_surgeon() -> Result<Option<Surgeon>, AppError> {
    let state = if let Some(state) = use_context::<AppState>() {
        state
    } else {
        return Err(AppError::State(
            "the call to `use_context::<AppState>()` in `get_authorized_surgeon()` returned `None`"
                .to_string(),
        ));
    };

    let auth_token = if let Ok(Some(auth_token)) = get_jwt_cookie().await {
        auth_token
    } else {
        return Err(AppError::Auth(
            "the call to `get_jwt_cookie` in `get_authorized_surgeon` returned `None`".to_string(),
        ));
    };

    // `create_client()` errors if a connection can't immediately be established, so it
    // isn't necessary to call `ensure_connection()` if the client was created through this
    // convenience method.
    //     let client = create_client()
    //         .await?
    //         .with_globals_fn(|client| client.set("ext::auth::client_token", &auth_token));
    //
    //     state.db.set(client)?;
    //
    //     let query = r#"
    // select global cur_surgeon {
    //     email,
    //     terms,
    //     full_name,
    //     preferred_name,
    //
    //     defaults: {
    //         site: { name },
    //         iol: { model, name, company, focus, toric },
    //         formula,
    //         custom_constant,
    //         main
    //     },
    //
    //     sia: { right: { power, axis }, left: { power, axis } }
    // };
    //         "#;

    let surgeon = query_as!(
        QuerySurgeon,
        r#"
with s as (
    select
        email,
        terms,
        full_name,
        preferred_name,
        default_site_id,
        default_iol_id,
        default_formula,
        default_custom_constant,
        default_main,
        default_sia_power,
        default_sia_axis_right,
        default_sia_axis_left
    from surgeon
    where email = $1
    limit 1
)

select
    s.email as "email: Email",
    s.terms,
    s.full_name,
    s.preferred_name,
    site.name as "default_site_name: String",
    iol.model as "default_iol_model: String",
    iol.name as "default_iol_name: String",
    iol.company as "default_iol_company: String",
    iol.focus as "default_iol_focus: Focus",
    iol.toric as "default_iol_toric: ToricPower",
    s.default_formula as "default_formula: Formula",
    s.default_custom_constant,
    s.default_main as "default_main: Main",
    s.default_sia_power as "default_sia_power: SiaPower",
    s.default_sia_axis_right as "default_sia_axis_right: Axis",
    s.default_sia_axis_left as "default_sia_axis_left: Axis"

from s
left join site on s.default_site_id = site.id
left join iol on s.default_iol_id = iol.id; 
        "#,
        "todo@todo.com"
    )
    .fetch_one(&db().await?)
    .await;

    if let Ok(surgeon) = surgeon {
        let surgeon: Surgeon = surgeon.into();

        if surgeon.terms.is_some() {
            state.surgeon.set(Some(surgeon.clone()))?;

            Ok(Some(surgeon))
        } else {
            redirect("/terms");

            Ok(None)
        }
    } else {
        redirect("/signup");

        Ok(None)
    }
}
