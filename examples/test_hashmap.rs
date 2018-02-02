#[macro_use]
extern crate easybuffers;
extern crate time;

use std::collections::HashMap;
use easybuffers::helper::{ Table, HyperHelper };

#[derive(PartialEq,Clone,Default,Debug)]
struct TestMap {
    field_0: String,
    field_1: HashMap<isize, String>,
    field_2: bool
}
realize_table! {
    3, TestMap { 
        field_0: String,
        field_1: HashMap,
        field_2: bool
    }
}
fn main() {
    let mut map = HashMap::new();
    let helper = HyperHelper::new(2); // 设置2字节表示数据偏移量
    map.insert(-100, String::from("Map"));
    map.insert(122222222222, String::from(""));
    let mut instance = TestMap {
        field_0: String::from(""),
        field_1: map,
        field_2: true
    };
    let mut bytes = Vec::with_capacity(1024);
    instance.serialize(&mut bytes,0,0,&helper);
    HyperHelper::push_pivot(3 ,&mut bytes,&helper); 
    // MAP 是无序的，但是不会影响结果 应该是RandomState 的结果                           
    // assert_eq!(bytes, vec![255, 0, 0, 8, 0, 34, 0, 34, 0, 3, 255, 8, 0, 22, 0, 24, 0, 2, 255, 2, 0, 4, 0, 6, 0, 2, 255, 48, 255, 77, 97, 112, 255, 2, 0, 0, 0, 2, 0, 2, 255, 49, 255, 1, 9]);
    let mut data = bytes;
    let pivot = data.pop().unwrap() as usize;
    let de_instance = TestMap::deserialize(&data, pivot, pivot, 0, &helper);
    println!("map is {:?}", de_instance.field_1);
}