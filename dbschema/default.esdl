# __source__ references the current object
# __subject__ references the current value

module default {
    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

### enums

    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    scalar type Formula extending enum<Barrett, Kane>;
    scalar type Side extending enum<Right, Left>;
    scalar type Urn extending str { constraint max_len_value(36); } # 36 is the max length of UUID

### abstract object types

    abstract type Sca {
        required sph: float32;
        cyl: float32;
        axis: int32 { constraint min_value(0); constraint max_value(179); }

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

    # case is a reserved keyword
    type Cas extending SoftCreate {
        required surgeon: Surgeon;
        required urn: Urn;
        required side: Side;
        target: Target;
        required date: cal::local_date;
        site: str;
        sia: Sia;
        iol: str;
        adverse: Adverse;
        required va: OpVa;
        required refraction: OpRefraction;
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

    type Target extending SoftCreate {}

    type Va extending SoftCreate {
        num: float32 { constraint min_ex_value(0.0); constraint max_value(20.0); }
        den: float32 { constraint min_ex_value(0.0); }
    }
}
