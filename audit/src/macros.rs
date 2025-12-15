#[macro_export]
macro_rules! some_or_empty {
    ($($id:ident),+) => (let ($($id),+) = ($($crate::db::some_or_empty($id),)+);)
}

#[cfg(test)]
mod tests {
    use audit_macro::RangeBounded;

    use crate::bounded::Bounded;

    #[test]
    fn derives_range_bounded() {
        #[derive(RangeBounded)]
        #[bounded(range = 0..50, rem = 5)]
        struct TestStruct(u32);

        TestStruct::new(25).unwrap();
    }
}
