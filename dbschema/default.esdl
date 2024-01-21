# __source__ references the current object
# __subject__ references the current value

module default {
    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

### enums

    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    scalar type Axis extending int32 { constraint min_value(0); constraint max_value(179); }
    scalar type Distance extending enum<Far, Near>;
    scalar type Focal extending enum<Mono, Edof, Multi>;
    scalar type Lens extending enum<Thick, Thin>;
    scalar type Side extending enum<Right, Left>;
    scalar type Urn extending str { constraint max_len_value(36); } # 36 is the max length of UUID, not that anyone is likely to use that...

### abstract object types

    abstract type Cyl {
        required power: float32;
        required axis: Axis;
    }

    abstract type Sca {
        required sph: float32;
        cyl: Cyl;
    }

    abstract type SoftCreate {
        required created_at: datetime {
            default := datetime_current();
            readonly := true;
        }
    }

### object types

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
    #     meridian: Axis;
    # }
    
    # case is a reserved keyword
    type Cas extending SoftCreate {
        required surgeon: Surgeon;
        required urn: Urn;
        required side: Side;
        # biometry: Biometry # eventually
        target: Target;
        required date: cal::local_date;
        site: str;
        sia: Sia;
        iol: OpIol;
        adverse: Adverse;
        required va: OpVa;
        required refraction: OpRefraction;
    }

    type Constant extending SoftCreate {
        required value: float32; # unconstrained for now (barrett factor -2.0-5.0, A 112-125, Kane A 110-125)
        required formula: Formula;
    }

    type Formula extending SoftCreate {
        required name: str { constraint exclusive; }
        required lens: Lens;
    }

    type Iol extending SoftCreate {
        required model: str { constraint exclusive; }
        required name: str;
        required type: Focal { default := Focal.Mono; }
        required toric: bool { default := false; }
        required multi constants: Constant;
    }

    type OpIol extending Sca, SoftCreate {
        required iol: Iol;
        constraint expression on (.sph >= -20.0 and .sph <= 60.0 and .sph % 0.25 = 0.0);
        constraint expression on (.cyl.power >= 1.0 and .cyl.power <= 20.0 and .cyl.power % 0.25 = 0.0);
    }

    type OpRefraction extending SoftCreate {
        before: Refraction;
        after: Refraction;
    }
    
    type OpVa extending SoftCreate {
        before: Va;
        after: Va;
    }

    type Refraction extending Sca, SoftCreate {
        required distance: Distance { default := Distance.Far; }

        constraint expression on (.sph >= -20.0 and .sph <= 20.0 and .sph % 0.25 = 0.0);
        constraint expression on (.cyl.power >= -10.0 and .cyl.power <= 10.0 and .cyl.power % 0.25 = 0.0);
    }

    type Sia extending SoftCreate {
        value: float32 { constraint min_value(0.0); constraint max_value(2.0) }
        meridian: Axis;
    }

    type Surgeon extending SoftCreate {
        required email: EmailType;
        first_name: str;
        last_name: str;
        site: str;
        handed: Side;
        sia_right: Sia;
        sia_left: Sia;
        multi cases := .<surgeon[is Cas];
    }

    type Target extending Sca, SoftCreate {
        formula: Formula;
        constraint expression on (.sph >= -6.0 and .sph <= 2.0);
        constraint expression on (.cyl >= 0.0 and .cyl <= 6.0);
    }

    type Va extending SoftCreate {
        required distance: Distance { default := Distance.Far; }
        num: float32 { constraint min_ex_value(0.0); constraint max_value(20.0); }
        den: float32 { constraint min_ex_value(0.0); }
    }
}
