use dotenvy::{EnvLoader, EnvSequence};

fn main() {
    EnvLoader::new()
        .sequence(EnvSequence::EnvOnly)
        .load()
        .unwrap(); // Not panics
    EnvLoader::new()
        .sequence(EnvSequence::EnvThenInput)
        .load()
        .unwrap(); // Panics
    EnvLoader::new()
        .sequence(EnvSequence::InputThenEnv)
        .load()
        .unwrap(); // Panics
    EnvLoader::new()
        .sequence(EnvSequence::InputOnly)
        .load()
        .unwrap(); // Panics
    println!("Hello, world!");
}
