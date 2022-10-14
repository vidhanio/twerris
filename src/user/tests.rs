use super::*;

#[test]
fn deserialize_sample_users() {
    let sample_users = include_str!("tests/sample_users.json");

    serde_json::from_str::<Vec<User>>(sample_users).expect("should parse sample users");
}
