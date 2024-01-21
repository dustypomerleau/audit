# __source__ references the current object
# __subject__ references the current value

module default {

### scalars

    scalar type Adverse extending enum<Rhexis, Pc, Zonule, Other>;
    
    scalar type Axis extending int32 {
        constraint min_value(0);
        constraint max_value(179);
    }

    scalar type Distance extending enum<Far, Near>;

    scalar type EmailType extending str {
        # HTML5 allows dotless domains, but ICANN doesn't, so prohibit here
        constraint regexp("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
    }

    scalar type Focus extending enum<Mono, Edof, Multi>;
    scalar type Lens extending enum<Thick, Thin>;
    scalar type Side extending enum<Right, Left>;

### abstract objects

    abstract type Cyl {
        required power: float32;
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
    #     meridian: Axis;
    # }
    
    # case is a reserved keyword
    type Cas extending SoftCreate {
        required surgeon: Surgeon;
        required urn: str;
        required side: Side;
        # biometry: Biometry # eventually
        target: Target;
        required date: cal::local_date;
        site: str; # if present, overrides surgeon default
        sia: Sia; # if present, overrides surgeon default
        iol: OpIol;
        adverse: Adverse;
        required va: OpVa;
        required refraction: OpRefraction;
    }

    type Constant extending SoftCreate {
        # unconstrained for now (barrett factor -2.0-5.0, Kane A 110-125)
        required value: float32;
        required formula: Formula;
    }

    type Formula extending SoftCreate {
        required name: str { constraint exclusive; }
        required lens: Lens;
    }

    type Iol extending SoftCreate {
        required model: str { constraint exclusive; }
        required name: str;
        required focus: Focus { default := Focus.Mono; }
        required toric: bool { default := false; }
        required multi constants: Constant;
    }

    type IolCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= 1.0 and .power <= 20.0 and .power % 0.25 = 0.0);
    }

    type OpIol extending SoftCreate {
        required iol: Iol;

        required se: float32 { 
            constraint min_value(-20.0);
            constraint max_value(60.0);
            constraint expression on (__subject__ % 0.25 = 0.0);
        }

        cyl: IolCyl;
    }

    # for now these values are only far refraction
    type OpRefraction extending SoftCreate {
        required before: Refraction;
        required after: Refraction;
    }
    
    # for now, these values are only far BCVA
    type OpVa extending SoftCreate {
        required before: Va;
        required after: Va;
    }

    type Refraction extending SoftCreate {
        required distance: Distance { default := Distance.Far; }

        required sph: float32 { 
            constraint min_value(-20.0);
            constraint max_value(20.0);
            constraint expression on (__subject__ % 0.25 = 0.0);
        }

        cyl: RefCyl;
    }

    type RefCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= -10.0 and .power <= 10.0 and .power % 0.25 = 0.0);
    }

    type Sia extending Cyl, SoftCreate {
        constraint expression on (.power >= 0.0 and .power <= 2.0);
    }

    type Surgeon extending SoftCreate {
        required email: EmailType { constraint exclusive; }
        first_name: str;
        last_name: str;
        site: str;
        handed: Side;
        sia: SurgeonSia;
        multi cases := .<surgeon[is Cas];
    }

    type SurgeonSia extending SoftCreate {
        required right: Sia;
        required left: Sia;
    }

    type Target extending SoftCreate {
        formula: Formula;
        
        required se: float32 {
            constraint min_value(-6.0);
            constraint max_value(2.0);
        }

        cyl: TargetCyl;
    }

    type TargetCyl extending Cyl, SoftCreate {
        constraint expression on (.power >= 0.0 and .power <= 6.0);
    }

    type Va extending SoftCreate {
        required distance: Distance { default := Distance.Far; }
        required num: float32 { constraint min_ex_value(0.0); constraint max_value(20.0); }
        required den: float32 { constraint min_ex_value(0.0); }
    }
}
