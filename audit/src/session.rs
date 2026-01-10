use std::collections::HashMap;

use async_trait::async_trait;
use axum_login::tower_sessions::ExpiredDeletion;
use axum_login::tower_sessions::SessionStore;
use axum_login::tower_sessions::session::Id;
use axum_login::tower_sessions::session::Record;
use axum_login::tower_sessions::session_store;
use serde_json::Value;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::query;
use sqlx::query_as;
use sqlx::types::Json;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::db;

// These traits exist so that we can use a UUID v7 as the primary key in the DB, and have it map
// exactly to the bytes in [`Record::id`], which is required to be a newtyped [`i128`].
pub trait ToUuid {
    fn to_uuid(&self) -> Uuid;
}

pub trait ToId {
    fn to_id(&self) -> Id;
}

impl ToUuid for Id {
    fn to_uuid(&self) -> Uuid { Uuid::from_bytes(self.0.to_be_bytes()) }
}

impl ToId for Uuid {
    fn to_id(&self) -> Id { Id(i128::from_be_bytes(*self.as_bytes())) }
}

#[derive(Clone, Debug)]
pub struct SesStore {
    db: Pool<Postgres>,
}

#[async_trait]
impl ExpiredDeletion for SesStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        query!(r#"delete from session where expiry < now();"#)
            .execute(&self.db)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()));

        Ok(())
    }
}

pub struct QueryRecord {
    id: Uuid,
    expiry_date: OffsetDateTime,
    data: Option<Json<HashMap<String, Value>>>,
}

impl From<QueryRecord> for Record {
    fn from(qr: QueryRecord) -> Self {
        Self {
            id: qr.id.to_id(),
            data: qr.data.unwrap_or_default().0,
            expiry_date: qr.expiry_date,
        }
    }
}

#[async_trait]
impl SessionStore for SesStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        // Rather than checking for a collision, we replace the i128 in the [`Id`] with
        // the bytes of a new v7 UUID, which will be used as the `id` in the DB.
        let uuid = Uuid::now_v7();
        record.id = uuid.to_id();

        if record.expiry_date.to_utc() < OffsetDateTime::now_utc() {
            return Err(session_store::Error::Backend(format!(
                "the supplied Record is already expired - the given`expiry_date` was: {}",
                &record.expiry_date
            )));
        }

        query!(
            r#"insert into session (id, expiry, data) values ($1, $2, $3::jsonb)"#,
            uuid,
            record.expiry_date,
            Json(record.data.clone()) as _
        );

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        if record.expiry_date.to_utc() < OffsetDateTime::now_utc() {
            return Err(session_store::Error::Backend(format!(
                "the supplied Record is already expired - the given`expiry_date` was: {}",
                &record.expiry_date
            )));
        }

        query!(
            r#"update session set expiry = $1, data = $2::jsonb where id = $3;"#,
            record.expiry_date,
            Json(record.data.clone()) as _,
            record.id.to_uuid()
        );

        Ok(())
    }

    async fn load(&self, id: &Id) -> session_store::Result<Option<Record>> {
        let qr = query_as!(
            QueryRecord,
            r#"
select
    id,
    expiry as expiry_date,
    data as "data: Json<HashMap<String, Value>>"
from session
where id = $1;
            "#,
            id.to_uuid()
        )
        .fetch_optional(&db().await?)
        .await
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        Ok(qr.map(Into::<Record>::into))
    }

    async fn delete(&self, id: &Id) -> session_store::Result<()> {
        query!(r#"delete from session where id = $1"#, id.to_uuid());

        Ok(())
    }
}
