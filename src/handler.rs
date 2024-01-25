use crate::{
    case::Case,
    distance::Far,
    refraction::{OpRefraction, Refraction},
    surgeon::Surgeon,
    va::{FarVaSet, OpVa},
};
use axum::Extension;
use edgedb_tokio::Client;
use leptos::{server, ServerFnError};
use uuid::Uuid;

#[server]
async fn insert_case(client: Extension<Client>, case: Case) -> Result<Uuid, ServerFnError> {
    let Case {
        surgeon:
            Surgeon {
                email,
                first_name,
                last_name,
                site,
            },
        urn,
        side,
        target,
        date,
        site,
        sia,
        iol,
        adverse,
        va:
            OpVa {
                best_far:
                    FarVaSet {
                        before: Far(Va { num, den }),
                        after: Far(Va { num, den }),
                    },
                best_near,
                raw_far,
                raw_near,
            },
        refraction:
            OpRefraction {
                before: Far(Refraction(Sca { sph, cyl })),
                after: Far(Refraction(Sca { sph, cyl })),
            },
    } = case;

    // return an error just to get this compiling so you can check the rest
    Err(ServerFnError::Args("a placeholder error".to_string()))
}
