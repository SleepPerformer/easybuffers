#[macro_use]
extern crate easybuffers;
extern crate time;

use easybuffers::helper::{ Table, HyperHelper };

#[derive(PartialEq,Clone,Default,Debug)]
struct TestMessage {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<String>,
    field_4: Option<bool>,
    field_5: Option<String>,
    field_6: Option<String>,
    field_7: Option<u32>,
    field_8: Option<String>,
    field_9: Option<String>,
    field_10: Option<bool>
}
impl TestMessage {
    fn print(&mut self) {
        let field_0 = &self.field_0;
        let field_1 = &self.field_1;
        let field_2 = &self.field_2;
        let field_3 = &self.field_3;
        let field_4 = &self.field_4;
        let field_5 = &self.field_5;
        let field_6 = &self.field_6;
        let field_7 = &self.field_7;
        let field_8 = &self.field_8;
        let field_9 = &self.field_9;
        let field_10 = &self.field_10;
        println!(" field_0:{:?},field_1:{:?},field_2:{:?},field_3:{:?},field_4:{:?},field_5:{:?},field_6:{:?},field_7:{:?},field_8:{:?},field_9:{:?},field_10:{:?}", 
                    field_0, field_1, field_2, field_3, field_4, field_5, field_6, field_7, field_8, field_9, field_10);
    }
    fn instance() -> TestMessage {
        Default::default()
    }
    fn init() -> TestMessage {
        TestMessage {
            field_0 : None,
            field_1 : Some(String::from("message_0")),
            field_2 : Some(false),
            field_3 : Some(String::from("in")),
            field_4 : Some(true),
            field_5 : Some(String::from("Rust")),
            field_6 : None,
            field_7 : Some(700000u32),
            field_8 : Some(String::from("message_1")),
            field_9 : Some(String::from("without Option")),
            field_10 : Some(true)
        }
    }
}
realize_table! {
    11, TestMessage { 
        field_0 : Option,
        field_1 : Option,
        field_2 : Option,
        field_3 : Option,
        field_4 : Option,
        field_5 : Option,
        field_6 : Option,
        field_7 : Option,
        field_8 : Option,
        field_9 : Option,
        field_10 : Option
    }
}
fn main() {
    let helper = HyperHelper::new(5);
    let mut bytes = Vec::with_capacity(1024);
    let test = TestMessage::init();
    let start = time::get_time();
    for i in 0..1000000 {
        let mut instance = TestMessage::init();
        instance.serialize(&mut bytes, 0, 0, &helper);
        HyperHelper::push_pivot(11 ,&mut bytes, &helper);
        // assert_eq!(bytes, vec![255, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 12, 0, 0, 0, 0, 14, 0, 0, 0, 0, 17, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 29, 0, 0, 0, 0, 39, 0, 0, 0, 0, 54, 0, 0, 0, 0, 54, 0, 0, 0, 0, 11, 255, 109, 101, 115, 115, 97, 103, 101, 95, 48, 255, 0, 255, 105, 110, 255, 1, 255, 82, 117, 115, 116, 255, 96, 174, 10, 0, 255, 109, 101, 115, 115, 97, 103, 101, 95, 49, 255, 119, 105, 116, 104, 111, 117, 116, 32, 79, 112, 116, 105, 111, 110, 255, 1, 61]);
        bytes.clear();      
    }
    let end = time::get_time();
    println!("序列化 {:?}", (end - start)/1000000);

    let start = time::get_time();
    for i in 0..1000000 {
        let mut data = vec![255, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 12, 0, 0, 0, 0, 14, 0, 0, 0, 0, 17, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 29, 0, 0, 0, 0, 39, 0, 0, 0, 0, 54, 0, 0, 0, 0, 54, 0, 0, 0, 0, 11, 255, 109, 101, 115, 115, 97, 103, 101, 95, 48, 255, 0, 255, 105, 110, 255, 1, 255, 82, 117, 115, 116, 255, 96, 174, 10, 0, 255, 109, 101, 115, 115, 97, 103, 101, 95, 49, 255, 119, 105, 116, 104, 111, 117, 116, 32, 79, 112, 116, 105, 111, 110, 255, 1, 61];
        let pivot = data.pop().unwrap() as usize;
        let de_instance = TestMessage::deserialize(&data, pivot, pivot, 0, &helper);
        assert_eq!(de_instance.field_7.unwrap(), 700000u32);
    }
    let end = time::get_time();
    println!("反序列化 10 {:?}", (end - start)/1000000);
}