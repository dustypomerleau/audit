use crate::{
    case::{Adverse, Case, Side},
    iol::Iol,
    surgeon::Surgeon,
    target::Constant,
    va::OpVa,
};
use chrono::NaiveDate;
use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

// todo: impl From<NormalType> for DbType, and then just call those in the impl From<Case> for
// DbCase

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbAfterVa {
    pub best: Option<DbVa>,
    pub raw: DbVa,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbBeforeVa {
    pub best: DbVa,
    pub raw: Option<DbVa>,
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

        let target = if let Some(target) = target {
            Some(DbTarget {
                constant: target.constant,
                se: target.se,
                cyl: if let Some(cyl) = target.cyl {
                    Some(DbCyl {
                        power: cyl.power,
                        axis: cyl.axis as i32,
                    })
                } else {
                    None
                },
            })
        } else {
            None
        };

        let sia = if let Some(sia) = sia {
            Some(DbSia {
                power: sia.power,
                axis: sia.axis as i32,
            })
        } else {
            None
        };

        let iol = if let Some(iol) = iol {
            Some(DbOpIol {
                iol: iol.iol,
                se: iol.se,
                cyl: if let Some(cyl) = iol.cyl {
                    Some(DbCyl {
                        power: cyl.power,
                        axis: cyl.axis as i32,
                    })
                } else {
                    None
                },
            })
        } else {
            None
        };

        let refraction = DbOpRefraction {
            before: DbRefraction {
                sph: refraction.before.sph,
                cyl: if let Some(cyl) = refraction.before.cyl {
                    Some(DbCyl {
                        power: cyl.power,
                        axis: cyl.axis as i32,
                    })
                } else {
                    None
                },
            },
            after: DbRefraction {
                sph: refraction.after.sph,
                cyl: if let Some(cyl) = refraction.after.cyl {
                    Some(DbCyl {
                        power: cyl.power,
                        axis: cyl.axis as i32,
                    })
                } else {
                    None
                },
            },
        };

        DbCase {
            // todo: do this pattern for all fields
            surgeon: surgeon.into(),
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
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbCyl {
    power: i32,
    axis: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpIol {
    pub iol: Iol,
    pub se: i32,
    pub cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpRefraction {
    before: DbRefraction,
    after: DbRefraction,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbOpVa {
    pub before: DbBeforeVa,
    pub after: DbAfterVa,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbRefraction {
    sph: i32,
    cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSia {
    power: i32,
    axis: i32,
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
        DbSurgeon {
            email: surgeon.email,
            first_name: surgeon.first_name,
            last_name: surgeon.last_name,
            site: surgeon.site,

            sia: if let Some(sia) = surgeon.sia {
                Some(DbSurgeonSia {
                    right: DbSia {
                        power: sia.right.power,
                        axis: sia.right.axis as i32,
                    },
                    left: DbSia {
                        power: sia.left.power,
                        axis: sia.left.axis as i32,
                    },
                })
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbSurgeonSia {
    pub right: DbSia,
    pub left: DbSia,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbTarget {
    pub constant: Option<Constant>,
    pub se: i32,
    pub cyl: Option<DbCyl>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct DbVa {
    num: i32,
    den: i32,
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
                    right: DbCyl {
                        power: 010,
                        axis: 100,
                    },
                    left: DbCyl {
                        power: 010,
                        axis: 100,
                    },
                }),
            },

            urn: "123".to_string(),
            side: Side::Right,

            target: Some(DbTarget {
                constant: Some(Constant {
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
        let case = case();
        todo!()
    }
}
