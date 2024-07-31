use crate::{
    bounds_check::Checked,
    case::{Adverse, Case, Side},
    cyl::Cyl,
    iol::{Iol, OpIol},
    refraction::{OpRefraction, Refraction},
    sia::Sia,
    surgeon::{Surgeon, SurgeonSia},
    target::{Constant, Formula, Target},
    va::{AfterVa, BeforeVa, OpVa, Va},
};
use chrono::NaiveDate;
use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

pub fn opt_into<T, U: From<T>>(from: Option<T>) -> Option<U> {
    if let Some(t) = from {
        Some(t.into())
    } else {
        None
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbAfterVa {
    pub best: Option<DbVa>,
    pub raw: DbVa,
}

impl From<AfterVa> for DbAfterVa {
    fn from(va: AfterVa) -> Self {
        Self {
            best: opt_into(va.best),
            raw: va.raw.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbBeforeVa {
    pub best: DbVa,
    pub raw: Option<DbVa>,
}

impl From<BeforeVa> for DbBeforeVa {
    fn from(va: BeforeVa) -> Self {
        Self {
            best: va.best.into(),
            raw: opt_into(va.raw),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbCase {
    pub surgeon: DbSurgeon,
    pub urn: String,
    pub side: Side,
    pub target: Option<DbTarget>,
    pub date: NaiveDate,
    pub site: Option<String>,
    pub sia: Option<DbSia>,
    pub iol: Option<DbOpIol>,
    pub adverse: Option<Adverse>,
    pub va: DbOpVa,
    pub refraction: DbOpRefraction,
}

impl From<Case> for DbCase {
    fn from(case: Case) -> Self {
        let Case {
            surgeon,
            urn,
            side,
            target,
            date,
            site,
            sia,
            iol,
            adverse,
            va,
            refraction,
        } = case;

        Self {
            surgeon: surgeon.into(),
            urn,
            side,
            target: opt_into(target),
            date,
            site,
            sia: opt_into(sia),
            iol: opt_into(iol),
            adverse,
            va: va.into(),
            refraction: refraction.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbConstant {
    pub value: i32,
    pub formula: Formula,
}

impl From<Constant> for DbConstant {
    fn from(constant: Constant) -> Self {
        Self {
            value: constant.value as i32,
            formula: constant.formula,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbCyl {
    power: i32,
    axis: i32,
}

impl From<Cyl> for DbCyl {
    fn from(cyl: Cyl) -> Self {
        Self {
            power: cyl.power,
            axis: cyl.axis as i32,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpIol {
    pub iol: Iol,
    pub se: i32,
    pub cyl: Option<DbCyl>,
}

impl From<OpIol<Checked>> for DbOpIol {
    fn from(iol: OpIol<Checked>) -> Self {
        let OpIol { iol, se, cyl, .. } = iol;

        Self {
            iol,
            se,
            cyl: opt_into(cyl),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpRefraction {
    before: DbRefraction,
    after: DbRefraction,
}

impl From<OpRefraction> for DbOpRefraction {
    fn from(refraction: OpRefraction) -> Self {
        Self {
            before: refraction.before.into(),
            after: refraction.after.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpVa {
    pub before: DbBeforeVa,
    pub after: DbAfterVa,
}

impl From<OpVa> for DbOpVa {
    fn from(va: OpVa) -> Self {
        Self {
            before: va.before.into(),
            after: va.after.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbRefraction {
    sph: i32,
    cyl: Option<DbCyl>,
}

impl From<Refraction<Checked>> for DbRefraction {
    fn from(refraction: Refraction<Checked>) -> Self {
        Self {
            sph: refraction.sph,
            cyl: opt_into(refraction.cyl),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSia {
    power: i32,
    axis: i32,
}

impl From<Sia> for DbSia {
    fn from(sia: Sia) -> Self {
        Self {
            power: sia.power,
            axis: sia.axis as i32,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSurgeon {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub site: Option<String>,
    pub sia: Option<DbSurgeonSia>,
}

impl From<Surgeon> for DbSurgeon {
    fn from(surgeon: Surgeon) -> Self {
        Self {
            email: surgeon.email,
            first_name: surgeon.first_name,
            last_name: surgeon.last_name,
            site: surgeon.site,
            sia: opt_into(surgeon.sia),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSurgeonSia {
    pub right: DbSia,
    pub left: DbSia,
}

// are impls like this needed? since all of the values already implement into?
impl From<SurgeonSia> for DbSurgeonSia {
    fn from(sia: SurgeonSia) -> Self {
        Self {
            right: sia.right.into(),
            left: sia.left.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbTarget {
    pub constant: Option<DbConstant>,
    pub se: i32,
    pub cyl: Option<DbCyl>,
}

impl From<Target<Checked>> for DbTarget {
    fn from(target: Target<Checked>) -> Self {
        let Target {
            constant, se, cyl, ..
        } = target;

        Self {
            constant: opt_into(constant),
            se,
            cyl: opt_into(cyl),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbVa {
    num: i32,
    den: i32,
}

impl From<Va> for DbVa {
    fn from(va: Va) -> Self {
        Self {
            num: va.num as i32,
            den: va.den as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        iol::Focus,
        target::Formula,
        va::{AfterVa, BeforeVa},
    };
    use chrono::NaiveDate;

    fn dbcase() -> DbCase {
        DbCase {
            surgeon: DbSurgeon {
                email: "test@test.com".to_string(),
                first_name: Some("Paul".to_string()),
                last_name: Some("Johnson".to_string()),
                site: None,
                sia: Some(DbSurgeonSia {
                    right: DbSia {
                        power: 010,
                        axis: 100,
                    },
                    left: DbSia {
                        power: 010,
                        axis: 100,
                    },
                }),
            },

            urn: "123".to_string(),
            side: Side::Right,

            target: Some(DbTarget {
                constant: Some(DbConstant {
                    value: 11936,
                    formula: Formula::Barrett,
                }),
                se: -012,
                cyl: Some(DbCyl {
                    power: 028,
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

                se: 2450,
                cyl: Some(DbCyl {
                    power: 100,
                    axis: 90,
                }),
            }),

            adverse: None,

            va: DbOpVa {
                before: DbBeforeVa {
                    best: Va {
                        num: 600,
                        den: 1200,
                    },
                    raw: None,
                },
                after: DbAfterVa {
                    best: Some(Va { num: 600, den: 500 }),
                    raw: Va { num: 600, den: 600 },
                },
            },

            refraction: DbOpRefraction {
                before: DbRefraction {
                    sph: -825,
                    cyl: Some(DbCyl {
                        power: 050,
                        axis: 180,
                    }),
                },
                after: DbRefraction {
                    sph: -025,
                    cyl: Some(DbCyl {
                        power: 025,
                        axis: 90,
                    }),
                },
            },
        }
    }

    #[test]
    fn inserts_a_case() {
        todo!()
    }
}
