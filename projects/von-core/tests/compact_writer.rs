use serde::Serializer;
use von::CompactWriter;

#[test]
fn test() {
    let mut buffer = String::new();
    let writer = CompactWriter::new(&mut buffer);
    writer.serialize_bool(false).unwrap();
    println!("{}", buffer)
}