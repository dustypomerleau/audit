use leptos::prelude::use_context;
use sqlx::Pool;
use sqlx::Postgres;

use crate::error::AppError;
use crate::state::AppState;

pub async fn db() -> Result<Pool<Postgres>, AppError> {
    let pool = if let Some(state) = use_context::<AppState>() {
        // The sqlx::Pool is cheap to clone because its inner field is Arc<PoolInner<DB>>
        state.db.clone()
    } else {
        return Err(AppError::Db(
            "AppState is not present in context".to_string(),
        ));
    };

    Ok(pool)
}

#[cfg(test)]
mod tests {}
