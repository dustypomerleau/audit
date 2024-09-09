# todo: how do we surface errors to the app on insert, for example if there is already a surgeon with that email and we try to insert it?
# in practice we need to design the query to check first and then link if it's already there, insert if it's not

# insert a new SurgeonCas
# todo: insert logic that deals with missing optional values
# todo: paste the hardcoded version here as that's where you fixed the syntax errors
# then you can add back positional variables
with
    adverse := $pos,
    after_refraction_cyl_axis := $pos,
    after_refraction_cyl_power := $pos,
    after_refraction_sph := $pos,
    before_refraction_cyl_axis := $pos,
    before_refraction_cyl_power := $pos,
    before_refraction_sph := $pos,
    best_after_va_den := $pos,
    best_after_va_num := $pos,
    best_before_va_den := $pos,
    best_before_va_num := $pos,
    date := $pos,
    iol_cyl_axis := $pos,
    iol_cyl_power := $pos,
    iol_model := $pos,
    iol_se := $pos,
    raw_after_va_den := $pos,
    raw_after_va_num := $pos,
    raw_before_va_den := $pos,
    raw_before_va_num := $pos,
    sia_axis := $pos,
    sia_power := $pos,
    side := $pos,
    site := $pos,
    target_constant := $pos,
    target_cyl_axis := $pos,
    target_cyl_power := $pos,
    target_se := $pos,
    urn := $pos,

    surgeon := global cur_surgeon,

    target_cyl := (select (insert TargetCyl {
        power := target_cyl_power,
        axis := target_cyl_axis
    })),
    target := (select (insert Target {
        constant := target_constant,
        se := target_se,
        cyl := target_cyl
    })),
    
    iol := (assert_single(select Iol filter .model = iol_model)),
    iol_cyl := (select (insert IolCyl { power := iol_cyl_power, axis := iol_cyl_axis })),
    opiol := (select (insert OpIol { iol := iol, se := iol_se, cyl := iol_cyl })),

    sia := (select (insert Sia { power := sia_power, axis := sia_axis })),

    best_before_va := (select (insert Va {
        num := best_before_va_num,
        den := best_before_va_den
    })),
    raw_before_va := (select (insert Va {
        num := raw_before_va_num,
        den := raw_before_va_den
    })),
    best_after_va := (select (insert Va {
        num := best_after_va_num,
        den := best_after_va_den
    })),
    raw_after_va := (select (insert Va {
        num := raw_after_va_num,
        den := raw_after_va_den
    })),
    before_va := (select (insert BeforeVa { best := best_before_va, raw := raw_before_va })),
    after_va := (select (insert AfterVa { best := best_after_va, raw := raw_after_va })),
    opva := (select (insert OpVa { before := before_va, after := after_va })),

    before_refraction_cyl := (select (insert RefractionCyl {
        power := before_refraction_cyl_power,
        axis := before_refraction_cyl_axis
    })),
    before_refraction := (select (insert Refraction {
        sph := before_refraction_sph,
        cyl := before_refraction_cyl
    })),
    after_refraction_cyl := (select (insert RefractionCyl {
        power := after_refraction_cyl_power,
        axis := after_refraction_cyl_axis
    })),
    after_refraction := (select (insert Refraction {
        sph := after_refraction_sph,
        cyl := after_refraction_cyl
    })),
    oprefraction := (select (insert OpRefraction {
        before := before_refraction,
        after := after_refraction
    })),

    cas := (select (insert Cas {
        side := side,
        target := target,
        date := date,
        sia := sia,
        opiol := opiol,
        adverse := adverse,
        va := opva,
        refraction := oprefraction
    }))
insert SurgeonCas {
    surgeon := surgeon,
    urn := urn,
    cas := cas,
    site := site
};

--- 

insert Iol {
    model := "sn60wf",
    name := "AcrySof IQ",
    company := "Alcon",
    focus := Focus.Mono,
    toric := false,
    constants := (select (insert Constant { value := 11898, formula := Formula.Kane }))
};

    before_va := (select (insert Va { num := <float32>6.0, den := <float32>12.0 })),
    after_va := (select (insert Va { num := <float32>6.0, den := <float32>6.0 })),
    before_ref := (select(
        insert Refraction {
            sph := <float32>-2.25,
            cyl := (select (insert RefCyl { power := <float32>-1.0, axis := <Axis>100 }))
    })),
    after_ref := (select(
        insert Refraction {
            sph := <float32>-0.25,
            cyl := (select (insert RefCyl { power := <float32>-0.5, axis := <Axis>150 }))
    })),
insert Cas {
    surgeon := (select (
        insert Surgeon { email := "tom@test.com" } unless conflict on .email
    )),
    urn := "my urn",
    side := Side.Right,
    date := (select <cal::local_date>'2024-01-20'),
    va := (select (insert OpVa { before := before_va, after := after_va, })),
    refraction := (select (
        insert OpRefraction { before := before_ref, after := after_ref, }
    )),
};

# server function to insert case:
# remember, you need to use positional arguments, not named
with
    before_va := (select (insert Va { num := <float32>6.0, den := <float32>12.0 })),
    after_va := (select (insert Va { num := <float32>6.0, den := <float32>6.0 })),
    before_ref := (select(
        insert Refraction {
            sph := <float32>-2.25,
            cyl := (select (insert RefCyl { power := <float32>-1.0, axis := <Axis>100 }))
    })),
    after_ref := (select(
        insert Refraction {
            sph := <float32>-0.25,
            cyl := (select (insert RefCyl { power := <float32>-0.5, axis := <Axis>150 }))
    })),
insert Cas {
    surgeon := (select (
        insert Surgeon { email := "tom@test.com" } unless conflict on .email
    )),
    urn := "my urn",
    side := Side.Right,
    date := (select <cal::local_date>'2024-01-20'),
    va := (select (insert OpVa { before := before_va, after := after_va, })),
    refraction := (select (
        insert OpRefraction { before := before_ref, after := after_ref, }
    )),
};

# ---

# setting global:
set global cur_surgeon_id := (select Surgeon.id filter Surgeon.email = "tom@tom.com" limit 1);

# https://docs.edgedb.com/database/edgeql/insert#conflicts
with 
    surgeon := 
    sia := (select (insert SurgeonSia {
        right := (select (insert Sia {
            power := <int32>100,
            axis := <int32>95
        } unless conflict on ((.power, .axis)) else (select Sia))),
        left := (select (insert Sia {
            power := <int32>100,
            axis := <int32>95
        } unless conflict on ((.power, .axis)) else (select Sia))),
    })),

    site := (select (insert Site {
        name := "RMH"
    } unless conflict))
insert Surgeon {
    email := "tom@tom.com",
    first_name := "tom",
    last_name := "surname",
    site := site,
    sia := sia 
} unless conflict on .email
else (select Surgeon);

# ---

# insert a Surgeon
insert Surgeon {
    identity := (select (insert ext::auth::Identity {
        issuer := "dusty",
        subject := "test surgeon subject"
    })),
    email := "surgeon1@gmail.com",
    first_name := "Tim",
    last_name := "Smith",
    sites := { (select (insert Site {
        name := "Royal Melbourne Hospital (Parkville, AUS)" 
    }))},
    sia := (select (insert SurgeonSia {
        right := (select (insert Sia { power := <int32>100, axis := <int32>100 })),
        left := (select (insert Sia { power := <int32>100, axis := <int32>100 })),
    }))
};

set global cur_surgeon := (assert_single((select Surgeon filter .email = "surgeon1@gmail.com")));

# ---

# hardcoded values to test inserting a case
# todo: mock auth to allow inserting a surgeon's case
# todo: replace the simple components of the with statement with edgedb::protocol::named_args!
with
    adverse := Adverse.Pc,
    after_refraction_cyl_axis := 90,
    after_refraction_cyl_power := 125,
    after_refraction_sph := -50,
    before_refraction_cyl_axis := 90,
    before_refraction_cyl_power := 50,
    before_refraction_sph := 200,
    best_after_va_den := 750,
    best_after_va_num := 600,
    best_before_va_den := 1200,
    best_before_va_num := 600,
    date := <cal::local_date>"2024-09-09",
    iol_cyl_axis := 0,
    iol_cyl_power := 100,
    iol_model := "sn60wf",
    iol_se := 2250,
    raw_after_va_den := 600,
    raw_after_va_num := 600,
    raw_before_va_den := 3600,
    raw_before_va_num := 600,
    sia_axis := 100,
    sia_power := 10,
    side := Side.Left,
    site_name := "Royal Melbourne Hospital (Parkville, AUS)",
    target_constant_value := 11898,
    target_constant_formula := Formula.Kane,
    target_cyl_axis := 90,
    target_cyl_power := 100,
    target_se := -170,
    urn := "abc123",

    site := (select( insert Site {
        name := site_name
    } unless conflict on .name else Site)),

    # surgeon := global cur_surgeon,

    # workaround until you have auth, then uncomment above
    surgeon := (assert_single((select Surgeon limit 1))),

    target_constant := (select (insert Constant {
        value := target_constant_value,
        formula := target_constant_formula
    })),
    target_cyl := (select (insert TargetCyl {
        power := target_cyl_power,
        axis := target_cyl_axis
    })),
    target := (select (insert Target {
        constant := target_constant,
        se := target_se,
        cyl := target_cyl
    })),
    
    iol := (assert_single((select Iol filter .model = iol_model))),
    iol_cyl := (select (insert IolCyl { power := iol_cyl_power, axis := iol_cyl_axis })),
    opiol := (select (insert OpIol { iol := iol, se := iol_se, cyl := iol_cyl })),

    sia := (select (insert Sia { power := sia_power, axis := sia_axis })),

    best_before_va := (select (insert Va {
        num := best_before_va_num,
        den := best_before_va_den
    })),
    raw_before_va := (select (insert Va {
        num := raw_before_va_num,
        den := raw_before_va_den
    })),
    best_after_va := (select (insert Va {
        num := best_after_va_num,
        den := best_after_va_den
    })),
    raw_after_va := (select (insert Va {
        num := raw_after_va_num,
        den := raw_after_va_den
    })),
    before_va := (select (insert BeforeVa { best := best_before_va, raw := raw_before_va })),
    after_va := (select (insert AfterVa { best := best_after_va, raw := raw_after_va })),
    opva := (select (insert OpVa { before := before_va, after := after_va })),

    before_refraction_cyl := (select (insert RefractionCyl {
        power := before_refraction_cyl_power,
        axis := before_refraction_cyl_axis
    })),
    before_refraction := (select (insert Refraction {
        sph := before_refraction_sph,
        cyl := before_refraction_cyl
    })),
    after_refraction_cyl := (select (insert RefractionCyl {
        power := after_refraction_cyl_power,
        axis := after_refraction_cyl_axis
    })),
    after_refraction := (select (insert Refraction {
        sph := after_refraction_sph,
        cyl := after_refraction_cyl
    })),
    oprefraction := (select (insert OpRefraction {
        before := before_refraction,
        after := after_refraction
    })),

    cas := (select (insert Cas {
        side := side,
        target := target,
        date := date,
        sia := sia,
        opiol := opiol,
        adverse := adverse,
        va := opva,
        refraction := oprefraction
    }))
insert SurgeonCas {
    surgeon := surgeon,
    urn := urn,
    cas := cas,
    site := site
};

