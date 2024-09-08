use better_limit_reader::prelude::*;

extern crate better_limit_reader;

fn main() -> LimitReaderResult<()> {
    let mut limit_reader = LimitReader::new();
    limit_reader.limit(24);

    let output = limit_reader.read_limited("./README.md".into())?;
    let bytes_read = output.bytes_read();
    println!(
        "LimitReaderOutput: {}, Bytes remaining: {}",
        &output,
        &output.bytes_remaining()
    );

    let data = limit_reader.buffer();
    let text = String::from_utf8(data[..(bytes_read as usize)].to_vec())?;

    println!("First line from README: {}", &text);

    Ok(())
}
