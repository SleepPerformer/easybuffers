#[macro_use]
extern crate easybuffers;
extern crate time;

use easybuffers::helper::{ Table, HyperHelper };

#[derive(PartialEq,Clone,Default,Debug)]
pub struct TestMessage_0 {
    field_0: Option<Vec<TestMessageChild_1>>, // 7 // 字段强制订为field 并且定义 set get 方法
    field_1: Option<String>, // 1
    field_2: Option<bool>, // 2
    field_3: Option<TestMessageChild_1>, // 4
    field_4: Option<String>, // 1
    field_5: Option<bool>, // 2
    field_6: Option<TestMessageChild_2>, // 5
    field_7: Option<Vec<String>>,// 6
    field_8: Option<bool>, // 2
    field_9: Option<String>, // 1
    field_10: Option<TestMessageChild_0> //3
}
impl TestMessage_0 {
    fn instance() -> TestMessage_0 {
        Default::default()
    }
    fn new( field_0: Vec<TestMessageChild_1>,
            field_4: &str,
            field_6: Option<TestMessageChild_2>,
            field_7: Option<Vec<String>>,
            field_10: Option<TestMessageChild_0>)
             -> TestMessage_0 {
        TestMessage_0 {
            field_0: Some(field_0),
            field_1: Some(String::from("field_100")),
            field_2: None,
            field_3: None,
            field_4: Some(String::from(field_4)),
            field_5: Some(false),
            field_6: field_6,
            field_7:field_7,
            field_8: Some(true),
            field_9: None,
            field_10:field_10  
        }
    }
}
// 使用宏好像会慢一点
realize_table! {
    11, TestMessage_0 { 
        field_0: Option, 
        field_1: Option, 
        field_2: Option,
        field_3: Option, 
        field_4: Option, 
        field_5: Option, 
        field_6: Option, 
        field_7: Option,
        field_8: Option,
        field_9: Option, 
        field_10: Option
    }
}
realize_table! {
    5, TestMessageChild_0 { 
        field_0: Option,
        field_1: Option,
        field_2: Option,
        field_3: Option,
        field_4: Option
    }
}
realize_table! {
    4, TestMessageChild_1 { 
        field_0: Option,
        field_1: Option,
        field_2: Option,
        field_3: Option
    }
}
realize_table! {
    3, TestMessageChild_2 { 
        field_0: Option,
        field_1: Option,
        field_2: Option
    }
}
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TestMessageChild_0 {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<TestMessageChild_2>,
    field_4: Option<Vec<Vec<bool>>>
}
impl TestMessageChild_0 {
    fn instance() -> TestMessageChild_0 {
        Default::default()
    }
    fn new( field_0: &str,
            field_3: TestMessageChild_2)
             -> TestMessageChild_0 {
        TestMessageChild_0 {
            field_0: Some(String::from(field_0)),
            field_1: Some(String::from("TestMessageChild_0")),
            field_2: Some(false),
            field_3: Some(field_3),
            field_4: Some(vec![vec![true, false, false], vec![false, true]])
        }
    }
}
#[derive(PartialEq,Clone,Default,Debug)]
struct TestMessageChild_1 {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<String>
}
impl TestMessageChild_1 {
    fn instance() -> TestMessageChild_1 {
        Default::default()
    }
    fn new(field_2: Option<bool>) -> TestMessageChild_1 {
        TestMessageChild_1 {
            field_0: Some(String::from("field_0")),
            field_1: Some(String::from("TestMessageChild_1")),
            field_2: field_2,
            field_3: None
        }
    }

}
#[derive(PartialEq,Clone,Default,Debug)]
struct TestMessageChild_2 {
    field_0: Option<String>,
    field_1: Option<bool>,
    field_2: Option<TestMessageChild_1>
}
impl TestMessageChild_2 {
    fn instance() -> TestMessageChild_2 {
        Default::default()
    }
    fn new() -> TestMessageChild_2 {
        TestMessageChild_2 {
            field_0: Some(String::from("TestMessageChild_2")),
            field_1: Some(true),
            field_2: None
        }
    }
}

fn init_child_2() -> TestMessageChild_2 {
    TestMessageChild_2::new()
}
fn init_child_0(message: &str) -> TestMessageChild_0 {
    TestMessageChild_0::new(message, init_child_2())
}
fn init_child_1(bool_value: Option<bool>) -> TestMessageChild_1 {
    TestMessageChild_1::new(bool_value)
}
fn init_father( field_0: Vec<TestMessageChild_1>,
                field_4: &str,
                field_6: Option<TestMessageChild_2>,
                field_7: Option<&mut Vec<&str>>,
                field_10: Option<TestMessageChild_0> )
                 -> TestMessage_0
    {
        let string_vec = match field_7 {
            Some(field_7) => {
                let mut string_vec = Vec::new();
                let len = field_7.len();
                for i in 0..len {
                    let string = field_7.pop().unwrap();
                    string_vec.push(String::from(string));
                }
                string_vec.reverse();
                Some(string_vec)
            },
            None => None,
        };
        TestMessage_0::new(field_0, field_4, field_6, string_vec, field_10)
}
fn father_instance() -> TestMessage_0 {
    let child_1_0 = init_child_1(Some(false));
    let child_1_1 = init_child_1(None);
    let child_1_2 = init_child_1(Some(true));
    let mut field_0 = vec![child_1_0, child_1_1, child_1_2];
    let mut field_7 = vec!["Use", "buffers", "in", "Rust"];
    let father = init_father(field_0, "Father", Some(TestMessageChild_2::new()), Some(&mut field_7), Some(init_child_0("Last field")));
    father
}
fn main() {
    let mut bytes = Vec::with_capacity(1024);
    let test_instance = father_instance();
    let start = time::get_time();
    for i in 0..1000000 {
        let mut instance = father_instance();
        instance.serialize(&mut bytes,0,0);
        HyperHelper::push_pivot(11 ,&mut bytes);
        // assert_eq!(bytes, vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25]);
        bytes.clear();      
    }
    let end = time::get_time();
    println!("序列化 {:?}", (end - start)/1000000);
    
    let start = time::get_time();
    for i in 0..1000000 {
        let mut data = vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25];
        let pivot = data.pop().unwrap() as usize;
        let de_instance = TestMessage_0::deserialize(&data, pivot, pivot, 0);
        // assert_eq!(test_instance, de_instance);
    }
    let end = time::get_time();
    println!("反序列化1000000 {:?}", (end - start)/1000000); 
}
