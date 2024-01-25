# insert a new Cas
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
    surgeon := (select (insert Surgeon {
        email := "bob@test.com"
    } unless conflict on .email)),
    urn := "my urn",
    side := Side.Right,
    date := (select <cal::local_date>'2024-01-20'),
    va := (select (insert OpVa { before := before_va, after := after_va, })),
    refraction := (select (insert OpRefraction { before := before_ref, after := after_ref, }))
};

