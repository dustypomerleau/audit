use polars::prelude::{
    JsonFormat, JsonWriter, LazyCsvReader, LazyFileListReader, PolarsError, SerWriter,
};
use std::{io::Write, path::Path};

#[derive(Clone, Debug)]
pub struct WriteString(pub String);

impl WriteString {
    pub fn new_from_csv(path: &Path) -> Result<Self, PolarsError> {
        let mut df = LazyCsvReader::new(path).finish()?.collect()?;
        let mut ws = WriteString(String::new());

        JsonWriter::new(&mut ws)
            .with_json_format(JsonFormat::Json)
            .finish(&mut df)?;

        Ok(ws)
    }
}

impl Write for WriteString {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        buf.iter().for_each(|c| self.0.push(*c as char));
        Ok(buf.len())
    }

    // This satisfies rustc, but I'm not actually sure how to implement `flush`.
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use crate::{csv::WriteString, flatcase::FlatCase};
    use polars::{
        io::{
            json::{JsonFormat, JsonWriter},
            SerWriter,
        },
        lazy::frame::{LazyCsvReader, LazyFileListReader},
    };

    #[test]
    fn reads_csv() {
        let mut df = LazyCsvReader::new("test/test.csv")
            .finish()
            .unwrap()
            .collect()
            .unwrap();

        println!("{df}");

        let mut ws = WriteString(String::new());

        JsonWriter::new(&mut ws)
            .with_json_format(JsonFormat::Json)
            .finish(&mut df)
            .unwrap();

        let json = &ws.0[..];
        println!("{json}\n\n");
        println!("{json:?}\n\n");

        let fc: Vec<FlatCase> = serde_json::from_str(json).unwrap();
        println!("{fc:?}\n\n");
    }
}
