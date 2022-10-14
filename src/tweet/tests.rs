use super::*;

#[test]
fn sample_tweets() {
    let sample_tweets = include_str!("tests/sample_tweets.json");

    serde_json::from_str::<Vec<Tweet>>(sample_tweets).unwrap();
}
