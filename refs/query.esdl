# todo: how do we surface errors to the app on insert, for example if there is already a surgeon with that email and we try to insert it?
# in practice we need to design the query to check first and then link if it's already there, insert if it's not

# insert a new SurgeonCas
with
    surgeon := global cur_surgeon,
    urn := 

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

