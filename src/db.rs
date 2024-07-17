use crate::{
    case::{Adverse, Case, Side},
    iol::Iol,
    target::Constant,
    va::{BeforeVaSet, Va},
};
use chrono::NaiveDate;
use edgedb_tokio::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbCyl {
    power: f32,
    axis: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSurgeonSia {
    pub right: DbCyl,
    pub left: DbCyl,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSurgeon {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub site: Option<String>,
    pub sia: Option<DbSurgeonSia>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbTarget {
    pub constant: Option<Constant>,
    pub se: f32,
    pub cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpIol {
    pub iol: Iol,
    pub se: f32,
    pub cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbAfterVaSet {
    pub best_far: Option<Va>,
    pub raw_far: Va,
    pub raw_near: Option<Va>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpVa {
    pub before: BeforeVaSet,
    pub after: DbAfterVaSet,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbRefraction {
    sph: f32,
    cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpRefraction {
    before: DbRefraction,
    after: DbRefraction,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbCase {
    pub surgeon: DbSurgeon,
    /// A unique value provided by the surgeon, such that deanonymization may only be performed by
    /// the surgeon.
    pub urn: String,
    pub side: Side,
    /// The surgeon's intended refractive target, based on the formula of their choice.
    pub target: Option<DbTarget>,
    pub date: NaiveDate,
    /// The institution where surgery was performed.
    pub site: Option<String>,
    // If no SIA is provided at the case level, the surgeon's defaults will be used.
    pub sia: Option<DbCyl>,
    pub iol: Option<DbOpIol>,
    pub adverse: Option<Adverse>,
    pub va: DbOpVa,
    pub refraction: DbOpRefraction,
}

impl From<Case> for DbCase {
    fn from(value: Case) -> Self {
        todo!()
    }
}
