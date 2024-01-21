# __source__ references the current object
# __subject__ references the current value

# todo: check exclusive constraints throughout, consider carefully any length constraints on text fields

module default {
    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

### enums

    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    scalar type Axis extending int32 { constraint min_value(0); constraint max_value(179); }
    scalar type Formula extending enum<Barrett, Kane>; # add more to this, should it just be an object? probably yes, even if you keep it as an enum in Rust (allows comparing thick lens formulas as a group, etc. because you can have a thick_lens: bool field, etc.)
    scalar type Side extending enum<Right, Left>;
    scalar type Urn extending str { constraint max_len_value(36); } # 36 is the max length of UUID, not that anyone is likely to use that...

### abstract object types

    abstract type Sca {
        required sph: float32;
        cyl: float32;
        axis: Axis;

        constraint expression on (
            (exists(.cyl) and exists(.axis))
            or (not exists(.cyl) and not exists(.axis))
        );
    }

    abstract type SoftCreate {
        required created_at: datetime {
            default := datetime_current();
            readonly := true;
        }
    }

### object types

    # # todo:
    # type Biometry extending SoftCreate {
    #     flat_k: K;
    #     steep_k: K;
    #     al: (constrained)
    #     acd: (constrained)
    #     lt: (constrained)
    #     cct: (constrained)
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
        target: Target;
        required date: cal::local_date;
        site: str;
        sia: Sia;
        iol: OpIol;
        adverse: Adverse;
        required va: OpVa;
        required refraction: OpRefraction;
    }

    type Iol extending SoftCreate {
        required model: str { constraint exclusive; }
        required name: str;
        required toric: bool { default := false; }
    }

    type OpIol extending Sca, SoftCreate {
        required iol: Iol;
        constraint expression on (.sph >= -20.0 and .sph <= 60.0 and .sph % 0.25 = 0.0);
        constraint expression on (.cyl >= 1.0 and .cyl <= 20.0 and .sph % 0.25 = 0.0);
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
        constraint expression on (.sph >= -20.0 and .sph <= 20.0 and .sph % 0.25 = 0.0);
        constraint expression on (.cyl >= -10.0 and .cyl <= 10.0 and .sph % 0.25 = 0.0);
    }

    type Sia extending SoftCreate {}

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
        num: float32 { constraint min_ex_value(0.0); constraint max_value(20.0); }
        den: float32 { constraint min_ex_value(0.0); }
    }
}
