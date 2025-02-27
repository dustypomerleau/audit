# __source__ references the current object
# __subject__ references the current value

### extensions

using extension auth;

module default {

### globals

global cur_surgeon := (assert_single(
    (select Surgeon filter .identity = global ext::auth::ClientTokenIdentity)
));

### scalars

    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    
    scalar type Axis extending int32 {
        constraint min_value(0);
        constraint max_value(179);
    }

    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

    scalar type Focus extending enum<Mono, Edof, Multi>;

    scalar type Formula extending enum<
        Barrett,
        BarrettTrueK,
        Haigis,
        HofferQ,
        Holladay1,
        Holladay2,
        Kane,
        Olsen,
        SrkT
    >;

    scalar type Side extending enum<Right, Left>;

### abstract objects

    abstract type Cyl {
        required power: int32;
        required axis: Axis;
    }

    abstract type SoftCreate {
        required created_at: datetime {
            default := datetime_current();
            readonly := true;
        }
    }

### objects

    # # todo:
    # # you should probably be quite generous with the constraints here, just confirming that the order of magnitude is correct
    # type Biometry extending SoftCreate {
    #     flat_k: K; (30-65)
    #     steep_k: K; (30-65)
    #     al: (constrained) (kane uses 18-35, you can probably do like 10-40)
    #     acd: (constrained) (1.5-5.0)
    #     lt: (constrained) (2.5-8.0)
    #     cct: (constrained) (350-650)
    #     wtw: (constrained)
    # }
    # type K extending SoftCreate {
    #     value: (constrained)
    #     axis: Axis; # meridian
    # }

    type AfterVa extending SoftCreate {
        best: Va;
        required raw: Va;
    }
    
    type BeforeVa extending SoftCreate {
        required best: Va;
        raw: Va;
    }

    # case is a reserved keyword
    type Cas extending SoftCreate {
        # biometry: Biometry # eventually
        required side: Side;
        target: Target;
        required year: int32 { default := <int32>datetime_get(datetime_current(), "year"); }
        sia: Sia; # form can prepopulate Surgeon.sia (todo: make required and default to inserting Sia of 0?
        opiol: OpIol;
        adverse: Adverse;
        required va: OpVa;
        required refraction: OpRefraction;
    }

    type Constant extending SoftCreate {
        # unconstrained for now (barrett factor -2.0-5.0 (-200 to 500), Kane A 110-125 (11000 to 12500))
        required value: int32;
        required formula: Formula;
    }

    # The IOLs are provided by the DB, including default constants.
    type Iol extending SoftCreate {
        required model: str { constraint exclusive; }
        required name: str;
        required company: str;
        required focus: Focus { default := Focus.Mono; }
        required toric: bool { default := false; }
        required multi constants: Constant;
    }

    type IolCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= 100 and .power <= 2000 and .power % 25 = 0);
    }

    type OpIol extending SoftCreate {
        required iol: Iol;

        required se: int32 { 
            constraint min_value(-2000);
            constraint max_value(6000);
            constraint expression on (__subject__ % 25 = 0);
        }

        cyl: IolCyl;
    }

    type OpRefraction extending SoftCreate {
        required before: Refraction;
        required after: Refraction;
    }
    
    type OpVa extending SoftCreate {
        required before: BeforeVa;
        required after: AfterVa;
    }

    type Refraction extending SoftCreate {
        required sph: int32 { 
            constraint min_value(-2000);
            constraint max_value(2000);
            constraint expression on (__subject__ % 25 = 0);
        }

        cyl: RefractionCyl;
    }

    type RefractionCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= -1000 and .power <= 1000 and .power % 25 = 0);
    }

    type Sia extending Cyl, SoftCreate {
        constraint expression on (.power >= 0 and .power <= 200);
    }

    type Site extending SoftCreate {
        required name: str { constraint exclusive; }
    }

    type Surgeon extending SoftCreate {
        required identity: ext::auth::Identity;
        required email: EmailType { constraint exclusive; }
        first_name: str;
        last_name: str;
        sia: SurgeonSia;
        default_site: Site;
        cases := .<surgeon[is SurgeonCas];
        constants := .<surgeon[is SurgeonConstant];
    }

    type SurgeonCas extending SoftCreate {
        required surgeon: Surgeon;
        required urn: str;
        required date: cal::local_date;
        required cas: Cas { constraint exclusive; }
        site: Site; # form can prepopulate Surgeon.default_site
        
        # When creating a plot, the surgeon access their own `SurgeonCas`es (which are 
        # restricted) but accesses others' `Cas`es (which are unrestricted) for comparison.
        access policy surgeon_full_access
            allow all using (.surgeon ?= global cur_surgeon) {
                errmessage := "Only the surgeon has access to their cases."
            };
    }

    type SurgeonConstant extending SoftCreate {
        required surgeon: Surgeon;
        required iol: Iol;
        required constant: Constant;
    }

    type SurgeonSia extending SoftCreate {
        required right: Sia;
        required left: Sia;
    }

    type Target extending SoftCreate {
        constant: Constant;
        
        required se: int32 {
            constraint min_value(-600);
            constraint max_value(200);
        }

        cyl: TargetCyl;
    }

    type TargetCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= 0 and .power <= 600);
    }

    type Va extending SoftCreate {
        required num: int32 { constraint min_value(0); constraint max_value(2000); }
        required den: int32 { constraint min_ex_value(0); }
    }
}
