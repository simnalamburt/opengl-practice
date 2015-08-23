extern crate obj;
extern crate rustc_serialize;
extern crate time;
extern crate bincode;

use std::fs::File;
use std::io::BufReader;
use obj::*;
use rustc_serialize::json;
use bincode::rustc_serialize as bcode;
use bincode::SizeLimit;
use time::now;

fn main() {
    let input = BufReader::new(File::open("rilakkuma.obj").unwrap());
    let t0 = now();

    let dome: Obj = load_obj(input).unwrap();
    let t = now();
    println!("obj 로딩 : {}ms", (t - t0).num_milliseconds());
    let t0 = t;

    println!("");

    let encoded = json::encode(&dome).unwrap();
    let t = now();
    println!("json 인코딩 : {}ms", (t - t0).num_milliseconds());
    let t0 = t;

    let _decoded: Obj = json::decode(&encoded).unwrap();
    let t = now();
    println!("json 디코딩 : {}ms", (t - t0).num_milliseconds());
    let t0 = t;

    println!("");

    let encoded = bcode::encode(&dome, SizeLimit::Infinite).unwrap();
    let t = now();
    println!("bincode 인코딩 : {}ms", (t - t0).num_milliseconds());
    let t0 = t;

    let _decoded: Obj = bcode::decode(&encoded).unwrap();
    let t = now();
    println!("bincode 디코딩 : {}ms", (t - t0).num_milliseconds());
    let t0 = t;

    let _ = t0;
}
