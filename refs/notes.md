# Notes

## Routes

- sign in
- register
- add case
- list all cases
- view and generate reports

## Auth

Consider [EdgeDB Auth](https://docs.edgedb.com/guides/auth)

## Types

### Separating Far and Near values

You have several options for structuring far and near separation (on `Va`, `Refraction`, etc.):

1. Use an enum:

```rust
enum Va {
    Far { num: f32, den: f32 },
    Near { num: f32, den: f32 }
}
```

This is the best solution for ease of use, but it doesn't allow you to constrain a field on `Case`, for example, to take only a `Va::Far`.

1. Use a field indicating the `Distance`:

```rust
enum Distance { Far, Near }

struct Va { distance: Distance, num: f32, den: f32 }
```

Again, you can pattern match on `distance`, but you can't enforce the value in the field's type on `Case`.

1. Use a wrapper struct:

```rust
struct Far<T>(T);

struct Va { num: f32, den: f32 }

struct Case { far_va: Option<Far<Va>> }
```

This allows separation of far and near, but now it's hard to write a function that operates on the values of `far_va` and `near_va`.

1. Use a trait:

```rust
trait Distance<T> {}

struct Far<T>(T);

impl<T> Distance<T> for Far<T> {}
```

This leads to type erasure, and then after you pass the type into a function, you can't pattern match on its subtype (far versus near).
It also forces you to use dynamic dispatch, which means requirements for `Sized`, boxing, etc.

Ultimately, the question is: How does wrapping `Va` in a `Far` really prevent you from screwing up? In the end, it's just a reminder about which type to put there.
And if it doesn't, why not just go back to using enums?
Or, to be more _extreme_ why not just skip this one at the level of the type system, and use the field name?

[this is a link](http://test.com)

1. There are only 2 variants (far and near), and only 2 situations where they are needed (`Refraction`, and `Va`), a final option would be dedicated types:

---

### Dealing with migrations that change a type with values inside

1. Create new temporary type & migrate
2. Move data to new temp type & any respective links
3. Delete data from original, make abstract, & migrate
4. Extend new abstract type as necessary, moving data from temporary type where applicable & respective links
5. Delete temp type & migrate

if I'm understanding correctly?
[2:52 AM]m10zeus: As long as moving links from one type to another is straight forward, I should be good to go!
[2:56 AM]Elvis: I would do it this way:

1. Extend the base type
2. Move data to the new subtypes
3. Make base abstract

You can actually do it all in a single migration if you edit the generated migration file and add the data moving statements. Migrations support arbitrary EdgeQL in them, not just DDL.
