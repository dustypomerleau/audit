use std::io::Write;

#[derive(Clone, Debug)]
pub struct WriteString(pub String);

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

        println!("{}", ws.0);
    }
}
