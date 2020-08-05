use abyss_engine::transform::{Transform};
use std::fs::{File};
use std::path::{PathBuf};
use std::io::{BufReader, Read};
use abyss_engine::config::ReadConfig;

#[test]
fn it_can_be_deserialized() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources/transform.yml");

    let input = File::open(d.as_path()).unwrap();
    let mut contents = String::new();
    let mut buffered = BufReader::new(input);
    buffered.read_to_string(&mut contents).unwrap();

    let t = Transform::read(contents.as_str()).unwrap();

    assert_eq!(0.0, t.position.x);
    assert_eq!(10.0, t.position.y);
}
