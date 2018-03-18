# Overview

需要手动遍历序列化的实例的每个字段，但是序列化需要更大空间

不需要使用Schema

类型暂时仅支持 String, bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, HasmMap, Vec, Struct

不支持&类型

直接定位解析数据位置，速度更快，详情参考 examples

# Structure

```rust
                 easybuffers bytes形式 结构示意图  
SPACE         vtable     | data_len | pivot | SPACE+data0  | SPACE+data1  | ~ |
       slot0  slot1  ~  
0xff | 2bytes 2bytes ~~~ | 2bytes   | 1byte | 0xff+n_bytes | 0xff+n_bytes | ~ |
       Pivot with offset                    | All data                        | 
```
# Future to do

支持更多基本类型的转化，针对Map的转化可以优化

实现 当修改序列化对象的部分字段 可以在已生成的bytes上修改，无需重新生成

# Issue

出现反序列化失败的情况 很有可能是计算偏移量的类型是u8 定位将其改为 usize 类型

对于usize 和 isize 类型，其实将其转为u64 和 i64类型进行操作

对于字段中多个None值的情况，性能可能不如protobuf

进行序列化的对象，将无法继续使用

*Struct* 必须满足 *Default* trait ，*HashMap* key字段必须满足*Clone*且反序列化的结果顺序可能会变化

可能需要对常用类型进行封装(极大程度的降低了序列化结果的大小) Vec<bool> => VecBool 、Vec<u16> => VecU16... 2.0版本添加了 最常用的 VecU8 

欢迎与我交流 337990443@qq.com

# How to use

```rust
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
```