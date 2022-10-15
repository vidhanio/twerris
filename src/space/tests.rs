use super::*;

#[test]
fn deserialize_samples() {
    let samples = include_str!("tests/samples.json");

    serde_json::from_str::<Vec<Space>>(samples).expect("should parse samples");
}
