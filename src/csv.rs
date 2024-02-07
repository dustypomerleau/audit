#[cfg(test)]
mod tests {
    use polars::lazy::frame::{LazyCsvReader, LazyFileListReader};

    #[test]
    fn reads_csv() {
        let lf = LazyCsvReader::new("test/test.csv")
            .finish()
            .unwrap()
            .collect();

        assert!(lf.is_ok());

        let df = lf.unwrap();
        println!("{df}");
    }
}
