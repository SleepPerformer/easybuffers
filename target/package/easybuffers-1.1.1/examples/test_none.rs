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
impl Table for TestMessage {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> TestMessage {
        let mut instance:TestMessage = Default::default();
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
                instance.field_3 = bool::deserialize(bytes, child_pivot, pivot, 3, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 4, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_4 = String::deserialize(bytes, child_pivot, pivot, 4, helper);
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
                instance.field_8 = bool::deserialize(bytes, child_pivot, pivot, 8, helper);
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
        match HyperHelper::child_pivot(bytes, pivot, 11, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_11 = Option::deserialize(bytes, child_pivot, pivot, 11, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 12, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_12 = Option::deserialize(bytes, child_pivot, pivot, 12, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 13, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_13 = Option::deserialize(bytes, child_pivot, pivot, 13, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 14, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_14 = bool::deserialize(bytes, child_pivot, pivot, 14, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 15, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_15 = Option::deserialize(bytes, child_pivot, pivot, 15, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 16, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_16 = Vec::deserialize(bytes, child_pivot, pivot, 16, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 17, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_17 = Vec::deserialize(bytes, child_pivot, pivot, 17, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 18, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_18 = bool::deserialize(bytes, child_pivot, pivot, 18, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 19, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_19 = Option::deserialize(bytes, child_pivot, pivot, 19, helper);
            } 
        }
        match HyperHelper::child_pivot(bytes, pivot, 20, helper) {
            None => (), // 不做任何操作
            Some(child_pivot) => {
                instance.field_20 = Option::deserialize(bytes, child_pivot, pivot, 20, helper);
            } 
        }
        instance
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let field_num = 21usize; // 需要外部传入
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
        self.field_11.serialize(table, child_pivot_index, 11, helper);
        self.field_12.serialize(table, child_pivot_index, 12, helper);
        self.field_13.serialize(table, child_pivot_index, 13, helper);
        self.field_14.serialize(table, child_pivot_index, 14, helper);
        self.field_15.serialize(table, child_pivot_index, 15, helper);
        self.field_16.serialize(table, child_pivot_index, 16, helper);
        self.field_17.serialize(table, child_pivot_index, 17, helper);
        self.field_18.serialize(table, child_pivot_index, 18, helper);
        self.field_19.serialize(table, child_pivot_index, 19, helper);
        self.field_20.serialize(table, child_pivot_index, 20, helper);
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
    let test_instance:TestMessage = Default::default();
    let start = time::get_time();
    for i in 0..1000000 {
        let mut instance:TestMessage = Default::default();
        instance.serialize(&mut bytes, 0, 0, &helper);
        HyperHelper::push_pivot(21 ,&mut bytes, &helper);
        assert_eq!(bytes, vec![255, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 10, 0, 14, 0, 16, 0, 0, 0, 0, 0, 16, 0, 21, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 45]);
        bytes.clear();      
    }
    let end = time::get_time();
    println!("序列化 {:?}", (end - start)/1000000);

    let start = time::get_time();
    for i in 0..1000000 {
        let mut data = vec![255, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 10, 0, 14, 0, 16, 0, 0, 0, 0, 0, 16, 0, 21, 255, 0, 255, 0, 255, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 45];
        let pivot = data.pop().unwrap() as usize;
        let de_instance = TestMessage::deserialize(&data, pivot, pivot, 0, &helper);
        // assert_eq!(test_instance, de_instance);
    }
    let end = time::get_time();
    println!("反序列化1000000 {:?}", (end - start)/1000000); 
}