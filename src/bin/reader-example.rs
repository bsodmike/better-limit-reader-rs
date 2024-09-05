use better_limit_reader::prelude::*;

extern crate better_limit_reader;

fn main() -> anyhow::Result<()> {
    let mut limit_reader = LimitReader::new();
    limit_reader.limit(24);
    let read_size = limit_reader.read_limited("./README.md".into())?;

    let data = limit_reader.buffer();
    let text = String::from_utf8(data[..read_size].to_vec()).unwrap();

    println!("First line from README: {}", &text);

    Ok(())
}
