use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::collections::HashMap;

const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoiSm9obiBEb2UiLCJpYXQiOjE1MTYyMzkwMjIsImV4cCI6OTU3NTg3ODM1NywiYXVkIjoiZm9vIn0.lOd828g8wv0Fc8e44GzSn0vkOMGJR4PIbX5baJl9UnE";

type Object = HashMap<String, serde_json::Value>;

fn main() -> Result<()> {
    let secret = DecodingKey::from_secret(b"your-256-bit-secret");
    let mut validation = Validation::default();
    validation.set_audience(&["foo"]);

    let result = decode::<Object>(TOKEN,&secret, &validation)?;

    println!("{result:#?}");

    Ok(())
}
