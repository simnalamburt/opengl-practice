use obj::{load_obj, Obj};
use std::{fs::File, io::BufReader, time::Instant};

fn main() {
    let input = BufReader::new(File::open("rilakkuma.obj").unwrap());

    let t = Instant::now();
    let dome: Obj = load_obj(input).unwrap();
    println!("obj 로딩 : {:?}", Instant::now() - t);
    println!();

    let t = Instant::now();
    let encoded = serde_json::to_string(&dome).unwrap();
    println!("serde_json 인코딩 : {:?}", Instant::now() - t);

    let t = Instant::now();
    let _decoded: Obj = serde_json::from_str(&encoded).unwrap();
    println!("serde_json 디코딩 : {:?}", Instant::now() - t);
    println!();

    let t0 = Instant::now();
    let encoded = bincode::serialize(&dome).unwrap();
    println!("bincode 인코딩 : {:?}", Instant::now() - t0);

    let t0 = Instant::now();
    let _decoded: Obj = bincode::deserialize(&encoded).unwrap();
    println!("bincode 디코딩 : {:?}", Instant::now() - t0);
}
