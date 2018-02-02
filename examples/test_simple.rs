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
impl Table for TestMessage {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> TestMessage {
        let mut instance = TestMessage::instance();
        match HyperHelper::child_pivot(bytes, pivot, 0, helper) {
            // List 先不考虑
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_0 = Option::deserialize(bytes, child_pivot, pivot, 0, helper);// 传引用，这样省时间，到需要转换基本数据 才调用 to_vec
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 1, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_1 = Option::deserialize(bytes, child_pivot, pivot, 1, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 2, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_2 = Option::deserialize(bytes, child_pivot, pivot, 2, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 3, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_3 = Option::deserialize(bytes, child_pivot, pivot, 3, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 4, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_4 = Option::deserialize(bytes, child_pivot, pivot, 4, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 5, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_5 = Option::deserialize(bytes, child_pivot, pivot, 5, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 6, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_6 = Option::deserialize(bytes, child_pivot, pivot, 6, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 7, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_7 = Option::deserialize(bytes, child_pivot, pivot, 7, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 8, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_8 = Option::deserialize(bytes, child_pivot, pivot, 8, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 9, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_9 = Option::deserialize(bytes, child_pivot, pivot, 9, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 10, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_10 = Option::deserialize(bytes, child_pivot, pivot, 10, helper);
            } 
        }
        instance
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let field_num = 11usize; // 需要外部传入
        let slot_size = 2; // 需要全局定义
        table.push(255u8); 
        table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
        let child_pivot_index = table.len()  - 1;
        table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
        // 更新每个字段

        self.field_0.serialize(table, child_pivot_index, 0, helper);
        self.field_1.serialize(table, child_pivot_index, 1, helper);
        self.field_2.serialize(table, child_pivot_index, 2, helper);
        self.field_3.serialize(table, child_pivot_index, 3, helper);
        self.field_4.serialize(table, child_pivot_index, 4, helper);
        self.field_5.serialize(table, child_pivot_index, 5, helper);
        self.field_6.serialize(table, child_pivot_index, 6, helper);
        self.field_7.serialize(table, child_pivot_index, 7, helper);
        self.field_8.serialize(table, child_pivot_index, 8, helper);
        self.field_9.serialize(table, child_pivot_index, 9, helper);
        self.field_10.serialize(table, child_pivot_index, 10, helper);
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
fn main() {
    let helper = HyperHelper::new(2);
    let mut bytes = Vec::with_capacity(1024);
    let test = TestMessage::init();
    let start = time::get_time();
    for i in 0..1000000 {
        let mut instance = TestMessage::init();
        instance.serialize(&mut bytes, 0, 0, &helper);
        HyperHelper::push_pivot(11 ,&mut bytes, &helper);
        assert_eq!(bytes, vec![255, 0, 0, 2, 0, 12, 0, 14, 0, 17, 0, 19, 0, 0, 0, 24, 0, 29, 0, 39, 0, 54, 0, 54, 0, 11, 255, 109, 101, 115, 115, 97, 103, 101, 95, 48, 255, 0, 255, 105, 110, 255, 1, 255, 82, 117, 115, 116, 255, 96, 174, 10, 0, 255, 109, 101, 115, 115, 97, 103, 101, 95, 49, 255, 119, 105, 116, 104, 111, 117, 116, 32, 79, 112, 116, 105, 111, 110, 255, 1, 25]);
        bytes.clear();      
    }
    let end = time::get_time();
    println!("序列化 {:?}", (end - start)/1000000);

    let start = time::get_time();
    for i in 0..1000000 {
        let mut data = vec![255, 0, 0, 2, 0, 12, 0, 14, 0, 17, 0, 19, 0, 0, 0, 24, 0, 29, 0, 39, 0, 54, 0, 54, 0, 11, 255, 109, 101, 115, 115, 97, 103, 101, 95, 48, 255, 0, 255, 105, 110, 255, 1, 255, 82, 117, 115, 116, 255, 96, 174, 10, 0, 255, 109, 101, 115, 115, 97, 103, 101, 95, 49, 255, 119, 105, 116, 104, 111, 117, 116, 32, 79, 112, 116, 105, 111, 110, 255, 1, 25];
        let pivot = data.pop().unwrap() as usize;
        let de_instance = TestMessage::deserialize(&data, pivot, pivot, 0, &helper);
        assert_eq!(de_instance.field_7.unwrap(), 700000u32);
    }
    let end = time::get_time();
    println!("反序列化 10 {:?}", (end - start)/1000000);
}
