#[macro_use]
extern crate easybuffers;
extern crate time;

use easybuffers::helper::{ Table, HyperHelper };
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TestMessage {
    field_0: Option<String>, 
    field_1: Option<String>, // 1
    field_2: Option<bool>, // 2
    field_3: bool, // 4
    field_4: String, // 1
    field_5: Option<bool>, // 2
    field_6: Option<Vec<bool>>, // 5
    field_7: Option<Vec<String>>,// 6
    field_8: bool, // 2
    field_9: Option<String>, // 1
    field_10: Option<Vec<String>>,
    field_11: Option<String>, 
    field_12: Option<String>, // 1
    field_13: Option<bool>, // 2
    field_14: bool, // 4
    field_15: Option<bool>, // 2
    field_16: Vec<bool>, // 5
    field_17: Vec<String>,// 6
    field_18: bool, // 2
    field_19: Option<String>, // 1
    field_20: Option<Vec<String>>
}
realize_table! {
    21, TestMessage { 
        field_0: Option, 
        field_1: Option, // 1
        field_2: Option, // 2
        field_3: bool, // 4
        field_4: String, // 1
        field_5: Option, // 2
        field_6: Option, // 5
        field_7: Option,// 6
        field_8: bool, // 2
        field_9: Option, // 1
        field_10: Option,
        field_11: Option, 
        field_12: Option, // 1
        field_13: Option, // 2
        field_14: bool, // 4
        field_15: Option, // 2
        field_16: Vec, // 5
        field_17: Vec,// 6
        field_18: bool, // 2
        field_19: Option, // 1
        field_20: Option
    }
}
fn main() {
    let mut bytes = Vec::with_capacity(1024);
    let test_instance:TestMessage = Default::default();
    let start = time::get_time();
    for i in 0..1000000 {
        let mut instance:TestMessage = Default::default();
        instance.serialize(&mut bytes,0,0);
        HyperHelper::push_pivot(21 ,&mut bytes);
        assert_eq!(bytes, vec![255, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 10, 0, 14, 0, 16, 0, 0, 0, 0, 0, 16, 0, 21, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 45]);
        bytes.clear();      
    }
    let end = time::get_time();
    println!("序列化 {:?}", (end - start)/1000000);
 
    let start = time::get_time();
    for i in 0..1000000 {
        let mut data = vec![255, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 10, 0, 14, 0, 16, 0, 0, 0, 0, 0, 16, 0, 21, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 45];
        let pivot = data.pop().unwrap() as usize;
        let de_instance = TestMessage::deserialize(&data, pivot, pivot, 0);
        // assert_eq!(test_instance, de_instance);
    }
    let end = time::get_time();
    println!("反序列化1000000 {:?}", (end - start)/1000000); 
}