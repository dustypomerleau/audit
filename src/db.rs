use crate::{
    case::{Adverse, Case, Side},
    iol::Iol,
    target::Constant,
    va::{BeforeVa, OpVa, Va},
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
pub struct DbAfterVa {
    pub best_far: Option<Va>,
    pub raw_far: Va,
    pub raw_near: Option<Va>,
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
    pub va: OpVa,
    pub refraction: DbOpRefraction,
}

impl From<Case> for DbCase {
    fn from(value: Case) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{iol::Focus, target::Formula, va::AfterVa};
    use chrono::NaiveDate;

    fn case() -> DbCase {
        DbCase {
            surgeon: DbSurgeon {
                email: "test@test.com".to_string(),
                first_name: Some("Paul".to_string()),
                last_name: Some("Johnson".to_string()),
                site: None,
                sia: Some(DbSurgeonSia {
                    right: DbCyl {
                        power: 0.1,
                        axis: 100,
                    },
                    left: DbCyl {
                        power: 0.1,
                        axis: 100,
                    },
                }),
            },

            urn: "123".to_string(),
            side: Side::Right,

            target: Some(DbTarget {
                constant: Some(Constant {
                    value: 119.36,
                    formula: Formula::Barrett,
                }),
                se: -0.12,
                cyl: Some(DbCyl {
                    power: 0.28,
                    axis: 90,
                }),
            }),

            date: NaiveDate::from_ymd_opt(2024, 7, 10).unwrap(),
            site: Some("RMH".to_string()),
            sia: None,

            iol: Some(DbOpIol {
                iol: Iol {
                    model: "zxt100".to_string(),
                    name: "Symfony".to_string(),
                    company: "Johnson and Johnson".to_string(),
                    focus: Focus::Edof,
                    toric: true,
                },

                se: 24.5,
                cyl: Some(DbCyl {
                    power: 1.0,
                    axis: 90,
                }),
            }),

            adverse: None,

            va: OpVa {
                before: BeforeVa {
                    best: Va {
                        num: 6.0,
                        den: 12.0,
                    },
                    raw: None,
                },
                after: AfterVa {
                    best: Some(Va { num: 6.0, den: 5.0 }),
                    raw: Va { num: 6.0, den: 6.0 },
                },
            },

            refraction: DbOpRefraction {
                before: DbRefraction {
                    sph: -8.25,
                    cyl: Some(DbCyl {
                        power: 0.5,
                        axis: 180,
                    }),
                },
                after: DbRefraction {
                    sph: -0.25,
                    cyl: Some(DbCyl {
                        power: 0.25,
                        axis: 90,
                    }),
                },
            },
        }
    }

    #[test]
    fn inserts_a_case() {
        let case = case();
        todo!()
    }
}

