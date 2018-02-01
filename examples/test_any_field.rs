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

impl Table for TestMessage_0 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize) -> TestMessage_0 {
        let mut instance = TestMessage_0::instance();
        match HyperHelper::child_pivot(bytes, pivot, 0) {
            // List 先不考虑
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_0 = Option::deserialize(bytes, child_pivot, pivot, 0);// 传引用，这样省时间，到需要转换基本数据 才调用 to_vec
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 1) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_1 = Option::deserialize(bytes, child_pivot, pivot, 1);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 2) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_2 = Option::deserialize(bytes, child_pivot, pivot, 2);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 3) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_3 = Option::deserialize(bytes, child_pivot, pivot, 3);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 4) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_4 = Option::deserialize(bytes, child_pivot, pivot, 4);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 5) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_5 = Option::deserialize(bytes, child_pivot, pivot, 5);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 6) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_6 = Option::deserialize(bytes, child_pivot, pivot, 6);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 7) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_7 = Option::deserialize(bytes, child_pivot, pivot, 7);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 8) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_8 = Option::deserialize(bytes, child_pivot, pivot, 8);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 9) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_9 = Option::deserialize(bytes, child_pivot, pivot, 9);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 10) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_10 = Option::deserialize(bytes, child_pivot, pivot, 10);
            } 
        }
        instance
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize) {
        let field_num = 11usize; // 需要外部传入
        let slot_size = 2; // 需要全局定义
        table.push(255u8); 
        table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
        let child_pivot_index = table.len()  - 1;
        table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
        // 更新每个字段

        self.field_0.serialize(table, child_pivot_index, 0);
        self.field_1.serialize(table, child_pivot_index, 1);
        self.field_2.serialize(table, child_pivot_index, 2);
        self.field_3.serialize(table, child_pivot_index, 3);
        self.field_4.serialize(table, child_pivot_index, 4);
        self.field_5.serialize(table, child_pivot_index, 5);
        self.field_6.serialize(table, child_pivot_index, 6);
        self.field_7.serialize(table, child_pivot_index, 7);
        self.field_8.serialize(table, child_pivot_index, 8);
        self.field_9.serialize(table, child_pivot_index, 9);
        self.field_10.serialize(table, child_pivot_index, 10);
        // 更新最终的长度
    
        if pivot_index != 0 {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                table[pivot_index - 2] = (len & 0xff) as u8;
                table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
            }
        }
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
impl Table for TestMessageChild_0 {
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize) {
        let field_num = 5usize; // 需要外部传入
        let slot_size = 2; // 需要全局定义
        table.push(255u8); 
        table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
        let child_pivot_index = table.len()  - 1;
        table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
        // 更新每个字段

        self.field_0.serialize(table, child_pivot_index, 0);
        self.field_1.serialize(table, child_pivot_index, 1);
        self.field_2.serialize(table, child_pivot_index, 2);
        self.field_3.serialize(table, child_pivot_index, 3);
        self.field_4.serialize(table, child_pivot_index, 4);
    
        if pivot_index != 0 {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                table[pivot_index - 2] = (len & 0xff) as u8;
                table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
            }
        }
        
    }
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize) -> TestMessageChild_0 {
        let mut instance = TestMessageChild_0::instance();
        match HyperHelper::child_pivot(bytes, pivot, 0) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_0 = Option::deserialize(bytes, child_pivot, pivot, 0);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 1) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_1 = Option::deserialize(bytes, child_pivot, pivot, 1);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 2) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_2 = Option::deserialize(bytes, child_pivot, pivot, 2);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 3) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_3 = Option::deserialize(bytes, child_pivot, pivot, 3);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 4) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_4 = Option::deserialize(bytes, child_pivot, pivot, 4);
            } 
        }
        instance
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
impl Table for TestMessageChild_1 {
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize) {
        let field_num = 4usize; // 需要外部传入
        let slot_size = 2; // 需要全局定义
        table.push(255u8); 
        table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
        let child_pivot_index = table.len() - 1;
        table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
        // 更新每个字段

        self.field_0.serialize(table, child_pivot_index, 0);
        self.field_1.serialize(table, child_pivot_index, 1);
        self.field_2.serialize(table, child_pivot_index, 2);
        self.field_3.serialize(table, child_pivot_index, 3);
   
        if pivot_index != 0 {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                // println!("child1 更新长度 {:?}", len);
                table[pivot_index - 2] = (len & 0xff) as u8;
                table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
            }
        }
        
    }
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize) -> TestMessageChild_1 {
        let mut instance = TestMessageChild_1::instance();
        match HyperHelper::child_pivot(bytes, pivot, 0) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_0 = Option::deserialize(bytes, child_pivot, pivot, 0);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 1) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_1 = Option::deserialize(bytes, child_pivot, pivot, 1);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 2) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_2 = Option::deserialize(bytes, child_pivot, pivot, 2);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 3) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_3 = Option::deserialize(bytes, child_pivot, pivot, 3);
            } 
        }
        instance
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
impl Table for TestMessageChild_2 {
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize) {
        let field_num = 3usize; // 需要外部传入
        let slot_size = 2; // 需要全局定义
        table.push(255u8); 
        table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
        let child_pivot_index = table.len() - 1;
        table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
        // 更新每个字段
        self.field_0.serialize(table, child_pivot_index, 0);
        self.field_1.serialize(table, child_pivot_index, 1);
        self.field_2.serialize(table, child_pivot_index, 2);
        // 判断是否是要更新上一级的vtable
        if pivot_index == 0 {
            // 说明是最高一层, 不需要更新任何内容
        } else {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                table[pivot_index - 2] = (len & 0xff) as u8;
                table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
            }
        }
    }
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize) -> TestMessageChild_2 {
        let mut instance = TestMessageChild_2::instance();
        match HyperHelper::child_pivot(bytes, pivot, 0) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_0 = Option::deserialize(bytes, child_pivot, pivot, 0);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 1) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_1 = Option::deserialize(bytes, child_pivot, pivot, 1);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 2) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_2 = Option::deserialize(bytes, child_pivot, pivot, 2);
            } 
        }
        instance
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
    // let mut bytes = Vec::with_capacity(1024);
    // let start = time::get_time();
    // for i in 0..1000000 {
    //     let mut instance = father_instance();
    //     instance.serialize(&mut bytes,0,0);
    //     HyperHelper::push_pivot(11 ,&mut bytes);
    //     // assert_eq!(bytes, vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25]);
    //     bytes.clear();      
    // }
    // let end = time::get_time();
    // println!("序列化 {:?}", (end - start)/1000000);

    
    let start = time::get_time();
    let mut fields = vec![0usize, 2];
    for i in 0..1000000 {
        // let test_instance = father_instance();
        let mut data = vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25];
        let pivot = data.pop().unwrap() as usize;
        match HyperHelper::any_field(&mut fields, &mut data, pivot) {
            None => println!("This field is None"),
            Some((child, help, position)) => {
                let string = TestMessageChild_1::deserialize(&data, child, help, position);
                // assert_eq!(string, test_instance.field_0.unwrap()[2]);
            },
        };
    }
    let end = time::get_time();
    println!("获取 TestMessageChild_1 1000000 {:?}", (end - start)/1000000);

    let start = time::get_time();
    let mut fields = vec![10usize, 4, 0, 2];
    for i in 0..1000000 {
        // let test_instance = father_instance();
        let mut data = vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25];
        let pivot = data.pop().unwrap() as usize;
        match HyperHelper::any_field(&mut fields, &mut data, pivot) {
            None => println!("This field is None"),
            Some((child, help, position)) => {
                let string = bool::deserialize(&data, child, help, position);
                // assert_eq!(string, test_instance.field_10.unwrap().field_4.unwrap()[0][2]);
            },
        };
    }
    let end = time::get_time();
    println!("获取 bool 1000000 {:?}", (end - start)/1000000);

    let start = time::get_time();
    let mut fields = vec![7usize, 2];
    for i in 0..1000000 {
        // let test_instance = father_instance();
        let mut data = vec![255, 10, 0, 133, 0, 0, 0, 0, 0, 143, 0, 150, 0, 160, 0, 193, 0, 215, 0, 0, 0, 229, 0, 72, 1, 11, 255, 12, 0, 53, 0, 92, 0, 121, 0, 3, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 0, 255, 2, 0, 10, 0, 0, 0, 0, 0, 27, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 2, 0, 10, 0, 29, 0, 0, 0, 29, 0, 4, 255, 102, 105, 101, 108, 100, 95, 48, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 49, 255, 1, 255, 102, 105, 101, 108, 100, 95, 49, 48, 48, 255, 70, 97, 116, 104, 101, 114, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 2, 0, 6, 0, 14, 0, 17, 0, 20, 0, 4, 255, 85, 115, 101, 255, 98, 117, 102, 102, 101, 114, 115, 255, 105, 110, 255, 82, 117, 115, 116, 255, 1, 255, 2, 0, 13, 0, 32, 0, 42, 0, 71, 0, 99, 0, 5, 255, 76, 97, 115, 116, 32, 102, 105, 101, 108, 100, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 48, 255, 0, 255, 2, 0, 21, 0, 0, 0, 21, 0, 3, 255, 84, 101, 115, 116, 77, 101, 115, 115, 97, 103, 101, 67, 104, 105, 108, 100, 95, 50, 255, 1, 255, 10, 0, 24, 0, 28, 0, 2, 255, 2, 0, 4, 0, 6, 0, 6, 0, 3, 255, 1, 255, 0, 255, 0, 255, 2, 0, 4, 0, 4, 0, 2, 255, 0, 255, 1, 25];
        let pivot = data.pop().unwrap() as usize;
        match HyperHelper::any_field(&mut fields, &mut data, pivot) {
            None => println!("This field is None"),
            Some((child, help, position)) => {
                let string = String::deserialize(&data, child, help, position);
                // assert_eq!(string, test_instance.field_7.unwrap()[2]);
            },
        };
    }
    let end = time::get_time();
    println!("获取 String 1000000 {:?}", (end - start)/1000000);
}
// 返回需要的字段的pivot
