#[allow(deprecated)]
use std::hash::{Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

static SPACE: u8 = 0xff;
#[derive(Debug)]
pub struct HyperHelper {
    slot_size: usize // 暂时只支持 2bytes 版本
}
impl HyperHelper {
    pub fn new(size: usize) -> HyperHelper {
        HyperHelper {
            slot_size: size
        }
    }
    pub fn set_slot_size(&mut self, num: usize) {
        self.slot_size = num;
    }
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    pub fn update_vtable(&self, table: &mut Vec<u8>, pivot_index: usize, position: usize) {
        let max = table[pivot_index] as usize;
        let offset = table.len() - pivot_index + 1;
        // 更新vtable
        for i in 0..self.slot_size {
            table[pivot_index - self.slot_size*(max - position + 1) + i] = ((offset >> i*8) & 0xff) as u8;
        }
    }
}
impl HyperHelper {
    pub fn child_pivot(bytes: &Vec<u8>, pivot: usize, child: usize, helper: &HyperHelper) -> Option<usize> {
        let slot_size = helper.slot_size();

        let mut offset = 0usize;
        let mut scale = 1usize;
        let slot_num = bytes[pivot] as usize;
        for i in 0..slot_size {
            // 计算对应的slot上的值(偏移量)
            offset += (bytes[pivot + i - (slot_num - child + 1) * slot_size] as usize) * scale;
            scale *= 256; 
        }
        // println!("string 的 offst {}", offset);
        match offset {
            0 => return None,
            _ => return Some(pivot+offset),
        };
    }
    pub fn push_pivot(filds: u8, table:&mut Vec<u8>, helper: &HyperHelper) {
        let slot_size = helper.slot_size() as u8;
        table.push((1+filds)*slot_size + 1);
    }
    // 专门用来获取某一字段的内容
    // 返回 (pivot, help_pivot, position) 用做serialize的参数
    pub fn any_field(fields: &mut Vec<usize>, table: & Vec<u8>, root: usize, helper: &HyperHelper) -> Option<(usize, usize, usize)> {
        let slot_size = helper.slot_size();
        let mut help_pivot = root;
        let mut pivot = root;
        let len = fields.len();
        for i in 0..len {
            // 先判断是否是基本类型
            if table[pivot - 1] == 255 {
                return Some((pivot, help_pivot, fields[i] as usize));
            }

            let offset = table[(pivot - slot_size*((table[pivot] as usize)-fields[i] +1))] as usize;
            if offset == 0 {
                return None; // 直接说明已经是None了
            }
            help_pivot = pivot;
            pivot += offset; 
        }
        Some((pivot, help_pivot, fields[len - 1] as usize))
    }
    // 先定位到修改字段，然后分离出来 获取相关内容修改
    // 
    // pub fn modify(bytes:&mut Vec<u8>, )
}

pub trait Table {
    /**
     * bytes      : 完整的hypertable bytes
     * pivot      : 字段的pivot在bytes中的索引 (是上一级 hypertable的一个字段)
     * help pivot : 上一级 hypertable 的pivot 在bytes中的索引
     * position   : 字段 在上一级 hypertable 的 slot 索引 好像没什么用
     * renturn      序列化后的结果
     */
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Self;
    /**
     * position 从0开始
     * return       bytes + pivot
    */
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper);
}
impl Table for String {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> String {
        let slot_size = helper.slot_size(); 
        let father_slot_num = bytes[help_pivot] as usize;
        let mut next_child_pivot = None;
        for child_index in position+1..father_slot_num {
            match HyperHelper::child_pivot(bytes, help_pivot, child_index, helper) {
                Some(pivot) => { next_child_pivot = Some(pivot); break;},
                None => (),
            };
        }
        match next_child_pivot {
            None => {
                // 之后的slot均为 None，字段的结束是help_pivot 的最后部分
                // 通过循环获取到 offset

                let mut offset = 0;
                let mut scale = 1;
                for i in 0..slot_size {
                    offset += (bytes[help_pivot - slot_size + i] as usize)*scale;
                    scale *= 256;
                }
                return String::from_utf8(bytes[pivot..help_pivot+offset+1].to_vec()).unwrap();
            },
            Some(n) => {
                // 是某一个同级的pivot 获取该字段的start
                if bytes[n - 1] == 0xff {
                    return String::from_utf8(bytes[pivot..n-1].to_vec()).unwrap();
                } else {
                    let end = n - (bytes[n] as usize + 1)*slot_size - 1; 
                    return String::from_utf8(bytes[pivot..end].to_vec()).unwrap();
                }
            }
        }
    }
    // 调用 ser 前都要先添加 vtable 的占位空间
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        // 先判断是否需要更新
        if self.len() != 0 {
            
            // 获取偏移量 len + 1
            // let offset = table.len() - pivot_index + 1;
            // // 先更新vtable
            // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
            // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
            helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
            // 添加信息
            table.push(SPACE);
            table.append(unsafe{self.as_mut_vec()});
        }
        // 如果是最后一个字段需要 再更新len
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for bool {
    /**
     * bytes      : 完整的hypertable bytes
     * pivot      : bool类型字段的pivot在bytes中的索引 (是上一级 hypertable的一个字段)
     * help pivot : 上一级 hypertable 的pivot 在bytes中的索引
     * position   : bool字段 在上一级 hypertable 的 slot 索引
     */
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> bool {
        // 直接根据pivot的值可以直接判断
        if bytes[pivot] == 1u8 {
            return true;
        } else if bytes[pivot] == 0u8 {
            return false;
        } else {
            panic!("{} is not a bool type", bytes[pivot]);
        }
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        if *self == true {
            table.push(1u8);
        } else {
            table.push(0u8);
        }
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for u8 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> u8 {
        bytes[pivot]
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        table.push(*self);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for u16 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> u16 {
        bytes[pivot] as u16 + (bytes[pivot+1] as u16) *256 
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        let b1:u8 = ((*self >> 8) & 0xff) as u8;
        table.push(b1);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for u32 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> u32 {
        let mut add = 0u32;
        let mut scale = 1u32;
        for i in 0..4 {
            add += (bytes[pivot + i] as u32)*scale;
            scale *= 256; 
        }
        add
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..4 {
            let b:u8 = ((*self >> i*8) & 0xff) as u8;
            table.push(b);
        }
        // println!(" ser {} {} {} {}", b0, b1, b2, b3);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
            // table[pivot_index - slot_size] = (len & 0xff) as u8;
            // table[pivot_index - slot_size + 1] = ((len >> 8) & 0xff) as u8;
        }
    }
}
impl Table for u64 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> u64 {
        let mut add = 0u64;
        let mut scale = 1u64;
        for i in 0..8 {
            add += (bytes[pivot + i] as u64)*scale;
            scale *= 256; 
        }
        add
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..8 {
            let b:u8 = ((*self >> i*8) & 0xff) as u8;
            table.push(b);
        }
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for usize {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> usize {
        // 调用 u64 的方法，不知道具体大小 性能可优化
        u64::deserialize(bytes, pivot, help_pivot, position, helper) as usize
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let self_as_u64 = *self as u64;
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (self_as_u64 & 0xff) as u8;
        table.push(b0);
        for i in 1..8 {
            let b:u8 = ((self_as_u64 >> i*8) & 0xff) as u8;
            table.push(b);
        }
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for i8 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> i8 {
        bytes[pivot] as i8
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let self_as_u8 = *self as u8;
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        table.push(self_as_u8);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for i16 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> i16 {
        (bytes[pivot] as u16 + (bytes[pivot+1] as u16) *256) as i16 
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let self_as_u16 = *self as u16;
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (self_as_u16 & 0xff) as u8;
        table.push(b0);
        let b1:u8 = ((self_as_u16 >> 8) & 0xff) as u8;
        table.push(b1);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for i32 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> i32 {
        let mut add = 0u32;
        let mut scale = 1u32;
        for i in 0..4 {
            add += (bytes[pivot + i] as u32)*scale;
            scale *= 256; 
        }
        add as i32
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..4 {
            let b:u8 = ((*self >> i*8) & 0xff) as u8;
            table.push(b);
        }
        // println!(" ser {} {} {} {}", b0, b1, b2, b3);
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for i64 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> i64 {
        let mut add = 0u64;
        let mut scale = 1u64;
        for i in 0..8 {
            add += (bytes[pivot + i] as u64)*scale;
            scale *= 256; 
        }
        add as i64
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..8 {
            let b:u8 = ((*self >> i*8) & 0xff) as u8;
            table.push(b);
        }
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
impl Table for isize {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> isize {
        i64::deserialize(bytes, pivot, help_pivot, position, helper) as isize
    } 
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let self_as_u64 = *self as u64;
        let slot_size = helper.slot_size();
        let max = table[pivot_index] as usize;
        
        // 获取偏移量 len + 1
        // let offset = table.len() - pivot_index + 1;
        // // 先更新vtable
        // table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
        // table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        // let b0:u8 = (self_as_u64 & 0xff) as u8;
        // table.push(b0);
        for i in 0..8 {
            let b:u8 = ((self_as_u64 >> i*8) & 0xff) as u8;
            table.push(b);
        }
        
        // 如果是最后一个字段需要 再更新end
        if max-1 == position {
            let len = table.len() - pivot_index - 1;
            // table[pivot_index - slot_size] = (len & 0xff) as u8;
            // table[pivot_index - slot_size + 1] = ((len >> 8) & 0xff) as u8;
            for i in 0..slot_size {
                table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
            }
        }
    }
}
// impl Table for u128 {
//     fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize) -> u128 {
//         let mut add = 0u128;
//         let mut scale = 1usize;
//         for i in 0..16 {
//             add += (bytes[pivot + i] as u128)*scale;
//             scale *= 256; 
//         }
//         add
//     }
//     fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize) {
//         let slot_size = 2usize;
//         let max = table[pivot_index] as usize;
        
//         // 获取偏移量 len + 1
//         let offset = table.len() - pivot_index + 1;
//         // 先更新vtable
//         table[pivot_index - slot_size*(max - position + 1)] = (offset & 0xff) as u8;
//         table[pivot_index - slot_size*(max - position + 1) + 1] = ((offset >> 8) & 0xff) as u8;
//         // 添加信息
//         table.push(SPACE);
//         let b0:u8 = (*self & 0xff) as u8;
//         table.push(b0);
//         for i in 1..16 {
//             let b:u8 = ((*self >> i*8) & 0xff) as u8;
//             table.push(b);
//         }
        
//         // 如果是最后一个字段需要 再更新end
//         if max-1 == position {
//             let len = table.len() - pivot_index - 1;
//             table[pivot_index - slot_size] = (len & 0xff) as u8;
//             table[pivot_index - slot_size + 1] = ((len >> 8) & 0xff) as u8;
//         }
//     }
// }
impl<T> Table for Option<T> where T: Table {
    // 必须有值 为空不会走这个函数
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Option<T> {
        Some(T::deserialize(bytes, pivot, help_pivot, position, helper))
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        // 判断是否为None
        match *self {
            None => {
                // 有可能是最后一个内容，需要去更新len
                let slot_size = helper.slot_size();
                if pivot_index != 0 {
                let max = table[pivot_index] as usize;
                    if position == max - 1 {
                        // 要更新father的len
                        let len = table.len() - 1 - pivot_index;
                        for i in 0..slot_size {
                            table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
                        }
                        
                    }
                }
            }
            // 这里的调用可以不用添加vtable部分
            Some(ref mut t) => t.serialize(table, pivot_index, position, helper),
        }
    }
}
impl<T> Table for Vec<T> where T: Table {
    // 必须有值 为空不会走这个函数
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Vec<T> {
        let slot_size = helper.slot_size();
        let mut vec = Vec::new();
        // 通过查看slot来获取
        let e_num = bytes[pivot] as usize;
        for i in 0..e_num {
            // 获取 offset
            let mut offset = 0;
            let mut scale = 1;
            for j in 0..slot_size {
                offset += (bytes[j + pivot - slot_size*(e_num - i+ 1)] as usize)*scale;
                scale *= 256;
            }
            vec.push(T::deserialize(bytes, pivot+offset, pivot, i, helper));
        }
        vec
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        
        let slot_size = helper.slot_size();
        table.push(SPACE);
        // 先将 属于数组部分的vtable 部分添加上
        let len = self.len();
        let mut vtable = vec![0u8;(len + 1) * slot_size + 1];
        table.append(&mut vtable);
        let child_pivot_index = table.len() - 1;
        table[child_pivot_index] = len as u8;
        // println!("第一个 pivot {:?} position {}", table[pivot_index], position);
        // println!("child pivot {:?} value {}", child_pivot_index, table[child_pivot_index]);
        // 和之前一样的步骤 只是在添加数据部分使用循环
        for i in 0..len {
            self[i].serialize(table, child_pivot_index, i, helper);
        }
        if pivot_index != 0 {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            for i in 0..slot_size {
                table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
            }
            // table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            // table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                for i in 0..slot_size {
                    table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
                }
            }
        }
    }
}

impl<K, V> Table for HashMap<K, V, RandomState> // 无序的 map
where K: Eq + Hash + Table + Clone, V: Table + Default {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> HashMap<K, V, RandomState> {
        // 将对象解析出来，然后放入 hashmap中
        let slot_size = helper.slot_size();
        let mut map = HashMap::new();
        let len = bytes[pivot] as usize; // 键值对的个数
        for i in 0..len {
            let mut offset = 0;
            let mut scale = 1;
            for j in 0..slot_size {
                offset += bytes[pivot - slot_size*(len - i + 1)+ j] as usize *scale;
                scale *= 256;
            }
            // let offset = bytes[pivot - slot_size*(len - i + 1)] as usize 
            //             + 256*(bytes[pivot - slot_size*(len - i + 1)+1] as usize);
            let inner_pivot = pivot + offset;

            let mut inner_offset_0 = 0;
            let mut inner_offset_1 = 0;
            let mut scale = 1;
            for j in 0..slot_size {
                inner_offset_0 += bytes[inner_pivot - slot_size*(2 - 0 + 1)+ j] as usize*scale;
                inner_offset_1 += bytes[inner_pivot - slot_size*(2 - 1 + 1)+ j] as usize*scale;
                scale *= 256;
            }
            // let inner_offset_0 = bytes[inner_pivot - slot_size*3] as usize 
            //                 + (bytes[inner_pivot - slot_size*3+1] as usize)*256;
            // println!("map 中 key字段的offset is {:?}",inner_offset_0);
            let child_pivot_0 = inner_pivot + inner_offset_0;
            let key = K::deserialize(bytes, child_pivot_0, inner_pivot, 0, helper);

            // let inner_offset_1 = bytes[inner_pivot - slot_size*2] as usize + (bytes[inner_pivot - slot_size*2+1] as usize)*256;
            if inner_offset_1 != 0 {
                let child_pivot_1 = inner_pivot + inner_offset_1;
                let value = V::deserialize(bytes, child_pivot_1, inner_pivot, 1, helper);
                map.insert(key, value);
            } else {
                map.insert(key, Default::default());
            }
            
            // value 可能为空 使用默认值
            
        } 
        map
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
        let slot_size = helper.slot_size();
        table.push(SPACE);
        let len = self.len(); // 键值对
        let mut vtable = vec![0u8;(len + 1) * slot_size + 1];
        table.append(&mut vtable);
        let child_pivot_index = table.len() - 1;
        table[child_pivot_index] = len as u8;
        let mut i = 0;
        for (key,mut value) in self.iter_mut() {
            // 这里要手动建立一个 2字段的结构体，并且 key 要进行clone
            table.push(SPACE);
            let inner_len = 2; // 键值对 只有2个字段
            let mut vtable = vec![0u8;(inner_len + 1) * slot_size + 1];
            table.append(&mut vtable);
            let inner_child_pivot_index = table.len() - 1;
            table[inner_child_pivot_index] = inner_len as u8;
            let offset = inner_child_pivot_index - child_pivot_index;
            // 可以更新外部第一个位置
            for j in 0..slot_size {
                table[child_pivot_index - slot_size*(len-i+1)+j] = ((offset >> j*8) & 0xff) as u8;
            }
            // table[child_pivot_index - slot_size*(len-i+1)] = (offset & 0xff) as u8;
            // table[child_pivot_index - slot_size*(len-i+1)+1] = ((offset >> 8) & 0xff) as u8;
            i += 1;
            // 对key进行转化
            let mut key = (*key).clone();// 可以优化
            key.serialize(table, inner_child_pivot_index, 0, helper);
            // 对value进行转化
            value.serialize(table, inner_child_pivot_index, 1, helper);
            // 对 child_pivot_index 所在table进行更新
        }
        // 把len 部分更新了
        let len = table.len() - 1 - child_pivot_index;
        for i in 0..slot_size {
            table[child_pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
        }
        // table[child_pivot_index - 2] = (len & 0xff) as u8;
        // table[child_pivot_index - 1] = ((len >> 8) & 0xff) as u8;
        // 更新上级的 table
        if pivot_index != 0 {
            // 更新father的vtable
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            
            let offset = child_pivot_index - pivot_index;
            for i in 0..slot_size {
                table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
            }
            // table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
            // table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
            if position == max - 1 {
                // 要更新father的len
                let len = table.len() - 1 - pivot_index;
                for i in 0..slot_size {
                    table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
                }
                // table[pivot_index - 2] = (len & 0xff) as u8;
                // table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
            }
        }
    }
}

// impl<K, V> Table for BTreeMap<K, V>
// where K:
#[macro_export]
macro_rules! realize_table {
    ( $num:expr, $name:ident { $( $fname:ident : $ftype:ident),* } ) => {
        impl Table for $name {

            fn deserialize (bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> $name {
                let mut instance:$name = Default::default();
                let mut index = 0;
                $(  
                    // println!("marco index {:?}", index);
                    match HyperHelper::child_pivot(bytes, pivot, index, helper) {
                        None => (), // 不做任何操作
                        Some(child_pivot) => {
                            instance.$fname = $ftype::deserialize(bytes, child_pivot, pivot, index, helper);// 传引用，这样省时间，到需要转换基本数据 才调用 to_vec
                        } 
                    }
                    index += 1;
                )*
                instance
            }
            fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
                let field_num = $num; // 需要外部传入
                // println!("marco field_num {:?}", field_num);
                let slot_size = helper.slot_size(); // 需要全局定义
                table.push(255u8); 
                table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
                let child_pivot_index = table.len() - 1;
                table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
                // 更新每个字段
                let mut index = 0;
                $(
                    self.$fname.serialize(table, child_pivot_index, index, helper);
                    // println!("marco ser index {:?}", index);
                    index += 1;
                )*
                if pivot_index != 0 {
                    // 更新father的vtable
                    // 算出 child 和 pivot 的距离
                    let max = table[pivot_index] as usize;
                    let offset = child_pivot_index - pivot_index;
                    for i in 0..slot_size {
                        table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
                    }
                    if position == max - 1 {
                        // 要更新father的len
                        let len = table.len() - 1 - pivot_index;
                        // println!("child1 更新长度 {:?}", len);
                        for i in 0..slot_size {
                            table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
                        }
                    }
                }
                
            }
        }

    }
}