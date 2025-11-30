use crate::bounded::Bounded;
use crate::model::Year;

pub fn query_select_compare(year: Year) -> String {
    format!(
        r#"
with
    QuerySurgeonCas := (
        select SurgeonCas filter .surgeon = global cur_surgeon and .cas.year = {year}
    ),

    QueryCohortCas := (select Cas except QuerySurgeonCas.cas filter .year = {year})

select {{
    surgeon := QuerySurgeonCas {{
        urn,
        side,
        date,
        site: {{ name }},
        cas: {{
            side,

            biometry: {{
                al,
                ks: {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
                acd,
                lt,
                cct,
                wtw
            }},
            
            target: {{
                formula,
                custom_constant,
                se,
                cyl: {{ power, axis }}
            }},
            
            year,
            main,
            sia: {{ power, axis }},
            
            iol: {{
                iol: {{ model, name, company, focus, toric }},
                se,
                axis
            }},

            adverse,

            va: {{
                before: {{ best: {{ num, den }}, raw: {{ num, den }} }},
                after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
            }},

            refraction: {{
                before: {{ sph, cyl: {{ power, axis }} }},
                after: {{ sph, cyl: {{ power, axis }} }},
            }}

        }}
    }},

    cohort := QueryCohortCas {{
        side,

        biometry: {{
            al,
            ks: {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
            acd,
            lt,
            cct,
            wtw
        }},
        
        target: {{
            formula,
            custom_constant,
            se,
            cyl: {{ power, axis }}
        }},
        
        year,
        main,
        sia: {{ power, axis }},
        
        iol: {{
            iol: {{ model, name, company, focus, toric }},
            se,
            axis
        }},

        adverse,

        va: {{
            before: {{ best: {{ num, den }}, raw: {{ num, den }} }},
            after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
        }},

        refraction: {{
            before: {{ sph, cyl: {{ power, axis }} }},
            after: {{ sph, cyl: {{ power, axis }} }},
        }}

    }}
}};
        "#
    )
}

pub fn query_select_self_compare(year: Year) -> String {
    let prior_year = year.inner() - 1;

    format!(
        r#"
with
    QuerySurgeonCas := (
        select SurgeonCas filter .surgeon = global cur_surgeon and .cas.year = {year}
    ),

    QueryCohortCas := (
        with cases := (
            select SurgeonCas
            filter .surgeon = global cur_surgeon and .cas.year = {prior_year}
        )

        select cases.cas
    )

select {{
    surgeon := QuerySurgeonCas {{
        urn,
        side,
        date,
        site: {{ name }},
        cas: {{
            side,

            biometry: {{
                al,
                ks: {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
                acd,
                lt,
                cct,
                wtw
            }},
            
            target: {{
                formula,
                custom_constant,
                se,
                cyl: {{ power, axis }}
            }},
            
            year,
            main,
            sia: {{ power, axis }},
            
            iol: {{
                iol: {{ model, name, company, focus, toric }},
                se,
                axis
            }},

            adverse,

            va: {{
                before: {{ best: {{ num, den }}, raw: {{ num, den }} }},
                after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
            }},

            refraction: {{
                before: {{ sph, cyl: {{ power, axis }} }},
                after: {{ sph, cyl: {{ power, axis }} }},
            }}

        }}
    }},

    cohort := QueryCohortCas {{
        side,

        biometry: {{
            al,
            ks: {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
            acd,
            lt,
            cct,
            wtw
        }},
        
        target: {{
            formula,
            custom_constant,
            se,
            cyl: {{ power, axis }}
        }},
        
        year,
        main,
        sia: {{ power, axis }},
        
        iol: {{
            iol: {{ model, name, company, focus, toric }},
            se,
            axis
        }},

        adverse,

        va: {{
            before: {{ best: {{ num, den }}, raw: {{ num, den }} }},
            after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
        }},

        refraction: {{
            before: {{ sph, cyl: {{ power, axis }} }},
            after: {{ sph, cyl: {{ power, axis }} }},
        }}

    }}
}};
        "#
    )
}