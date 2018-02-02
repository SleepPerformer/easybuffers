#[macro_use]
extern crate easybuffers;

use std::collections::HashMap;
use easybuffers::helper::{ Table, HyperHelper };

#[derive(PartialEq,Clone,Default,Debug)]
struct TestMap {
    author: String,
    map: HashMap<isize, String>,
    boolean: bool
}
realize_table! {
    3, TestMap { 
        author: String,
        map: HashMap,
        boolean: bool
    }
}
fn main() {
    let mut map = HashMap::new();
    let helper = HyperHelper::new(2); // 设置2字节表示数据偏移量
    map.insert(-100, String::from("Value"));
    map.insert(122222222222, String::from("Rust"));
    let mut instance = TestMap {
        author: String::from("SleepPerformer"),
        map: map,
        boolean: true
    };
    let mut bytes = Vec::with_capacity(1024);
    instance.serialize(&mut bytes,0,0,&helper);
    HyperHelper::push_pivot(3 ,&mut bytes,&helper); 
    let mut data = bytes;
    let pivot = data.pop().unwrap() as usize;
    let de_instance = TestMap::deserialize(&data, pivot, pivot, 0, &helper);
    println!("map is {:?}", de_instance.map);
}