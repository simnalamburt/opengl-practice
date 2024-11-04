use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use time::Instant;

fn main() {
    let input = BufReader::new(File::open("rilakkuma.obj").unwrap());
    let t0 = Instant::now();

    let dome: Obj = load_obj(input).unwrap();
    let t = Instant::now();
    println!("obj 로딩 : {}ms", (t - t0).whole_milliseconds());
    let t0 = t;

    println!();

    let encoded = serde_json::to_string(&dome).unwrap();
    let t = Instant::now();
    println!("serde_json 인코딩 : {}ms", (t - t0).whole_milliseconds());
    let t0 = t;

    let _decoded: Obj = serde_json::from_str(&encoded).unwrap();
    let t = Instant::now();
    println!("serde_json 디코딩 : {}ms", (t - t0).whole_milliseconds());
    let t0 = t;

    println!();

    let encoded = bincode::serialize(&dome).unwrap();
    let t = Instant::now();
    println!("bincode 인코딩 : {}ms", (t - t0).whole_milliseconds());
    let t0 = t;

    let _decoded: Obj = bincode::deserialize(&encoded).unwrap();
    let t = Instant::now();
    println!("bincode 디코딩 : {}ms", (t - t0).whole_milliseconds());
    let t0 = t;

    let _ = t0;
}
