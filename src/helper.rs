#[allow(deprecated)]
use std::hash::{Hash, Hasher};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

static SPACE: u8 = 0xff;
/// 与 flatbuffers 思路一致， 但是仅使用 table，primitive 表示所有类型
/// table => SPACE + vtable + data_len + pivot + data。 适用 T, Vec<T>, Option<T>, HashMap<K, V> 类型
/// primitive => SPACE + origin [u8]。 使用Rust内置的基本类型
#[derive(PartialEq,Clone,Default,Debug)]
/// HyperHelper 主要存储序列化和反序列化的全局信息 
/// 暂时只存储 vtable 中每一个slot的大小（bytes）
// 封装Vec<u8> 方便序列化
pub struct VecU8 {
    inner: Vec<u8>
}
impl VecU8 {
    pub fn init_with_vec_u8(data: Vec<u8>) -> VecU8 {
        VecU8 {
            inner: data
        }
    }
    pub fn len(&self) -> usize{
        self.inner.len()
    }
    pub fn to_vec_u8(self) -> Vec<u8> {
        self.inner
    }
}
// 暂时还没实现，看情况添加
pub struct VecBool {
    inner: Vec<bool>
}
impl VecBool {
    pub fn init_with_vec_bool(data: Vec<bool>) -> VecBool {
        VecBool {
            inner: data
        }
    }
    pub fn to_vec_bool(self) -> Vec<bool> {
        self.inner
    }
}
pub struct HyperHelper {
    slot_size: usize 
}
impl HyperHelper {
    /// 创建一个HyperHelper实例 参数是 序列化和反序列的 slot size
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate easybuffers;
    /// use easybuffers::helper::{ Table, HyperHelper };
    /// 
    /// let helper = HyperHelper::new(2);
    /// ```
    pub fn new(size: usize) -> HyperHelper {
        HyperHelper {
            slot_size: size
        }
    }
    /// 设置 slot size 的大小
    pub fn set_slot_size(&mut self, num: usize) {
        self.slot_size = num;
    }
    /// 获取 slot size 的大小
    pub fn slot_size(&self) -> usize {
        self.slot_size
    }
    #[inline]
    fn update_vtable(&self, table: &mut Vec<u8>, pivot_index: usize, position: usize) {
        let max = table[pivot_index] as usize;
        let offset = table.len() - pivot_index + 1;
        for i in 0..self.slot_size {
            table[pivot_index - self.slot_size*(max - position + 1) + i] = ((offset >> i*8) & 0xff) as u8;
        }
    }
}
impl HyperHelper {
    /// 通过 二进制数组、pivot位置、字段索引 获取目标字段的pivot相对pivot的偏移量
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate easybuffers;
    /// use easybuffers::helper::{ Table, HyperHelper };
    /// 
    /// let helper = HyperHelper::new(2);
    /// let bytes = vec![255u8, 2, 0, 4, 0, 4, 0, 2, 255, 1, 255, 0]; // 2个字段 field_0, field_1
    /// let field_1_index = HyperHelper::child_pivot(&bytes, 7, 1, &helper).unwrap();
    /// assert_eq!(field_1_index, 11);
    /// ```
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
        match offset {
            0 => return None,
            _ => return Some(pivot+offset),
        };
    }
    /// 将最高层的pivot添加入最终参与网络传输的二进制数组的末尾
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate easybuffers;
    /// use easybuffers::helper::{ Table, HyperHelper };
    /// 
    /// let helper = HyperHelper::new(2);
    /// let mut bytes = vec![255u8, 2, 0, 4, 0, 4, 0, 2, 255, 1, 255, 0]; // 2个字段 field_0, field_1
    /// HyperHelper::push_pivot(2, vec![255u8, 2, 0, 4, 0, 4, 0, 2, 255, 1, 255, 0, 7]);
    /// assert_eq!(bytes, 11);
    /// ```
    pub fn push_pivot(filds: u8, table:&mut Vec<u8>, helper: &HyperHelper) {
        let slot_size = helper.slot_size() as u8;
        table.push((1+filds)*slot_size + 1);
    }
    /// 专门用来获取某一字段的内容 vec存储按层顺序的字段索引
    /// 返回 (pivot, help_pivot, position) 用做serialize的参数
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate easybuffers;
    /// use easybuffers::helper::{ Table, HyperHelper };
    /// 
    /// let helper = HyperHelper::new(2);
    /// let mut bool_vec = vec![true, true, false, false, true];
    /// let mut bytes = Vec::with_capacity(1024);
    /// bool_vec.serialize(&mut bytes, 0, 0, &helper);
    /// let mut fields = vec![2;1];
    /// let (pivot, help_pivot, pisition) = HyperHelper::any_field(&mut fields, &mut bytes, 13, &helper).unwrap();
    /// let res = bool::deserialize(&data, child, help, position, &helper);
    /// assert_eq!(res, false);
    /// ```
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
}

pub trait Table {
    /// /**
    ///  * bytes      : 完整的hypertable bytes
    ///  * pivot      : 字段的pivot在bytes中的索引 (是上一级 hypertable的一个字段)
    ///  * help pivot : 上一级 hypertable 的pivot 在bytes中的索引
    ///  * position   : 字段 在上一级 hypertable 的 slot 索引 好像没什么用
    ///  * helper     : 存储全局信息
    ///  * renturn      反序列化后的结果
    ///  */
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Self;
    /// /**
    ///  * bytes      : 完整的hypertable bytes
    ///  * pivot      : 字段的pivot在bytes中的索引 (是上一级 hypertable的一个字段)
    ///  * help pivot : 上一级 hypertable 的pivot 在bytes中的索引
    ///  * position   : 字段 在上一级 hypertable 的 slot 索引 
    ///  * helper     : 存储全局信息
    ///  */
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
                if bytes[n - 1] == SPACE {
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
            helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
            table.push(SPACE);
            table.append(unsafe{self.as_mut_vec()}); // delete unsafe
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
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..4 {
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
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        let b0:u8 = (*self & 0xff) as u8;
        table.push(b0);
        for i in 1..4 {
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
        helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
        // 添加信息
        table.push(SPACE);
        for i in 0..8 {
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
impl Table for VecU8 {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> VecU8 {
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
                let mut offset = 0;
                let mut scale = 1;
                for i in 0..slot_size {
                    offset += (bytes[help_pivot - slot_size + i] as usize)*scale;
                    scale *= 256;
                }
                return VecU8::init_with_vec_u8(bytes[pivot..help_pivot+offset+1].to_vec());
            },
            Some(n) => {
                // 是某一个同级的pivot 获取该字段的start
                if bytes[n - 1] == SPACE {
                    return VecU8::init_with_vec_u8(bytes[pivot..n-1].to_vec());
                } else {
                    let end = n - (bytes[n] as usize + 1)*slot_size - 1; 
                    return VecU8::init_with_vec_u8(bytes[pivot..end].to_vec());
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
            helper.update_vtable(table, pivot_index, position); // 有一步多余操作，但影响不大
            table.push(SPACE);
            // let mut vec = self.inner;
            table.append(&mut self.inner); 
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
impl<T> Table for Option<T> where T: Table {
    // 必须有值 为空不会走这个函数
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Option<T> {
        Some(T::deserialize(bytes, pivot, help_pivot, position, helper))
    }
    fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
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
            Some(ref mut t) => t.serialize(table, pivot_index, position, helper),
        }
    }
}

impl<T> Table for Vec<T> where T: Table {
    // 必须有值 为空不会走这个函数
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Vec<T> {
        let slot_size = helper.slot_size();
        let mut vec = Vec::new();
        let e_num = bytes[pivot] as usize;
        for i in 0..e_num {
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
        // println!("Vec序列化");
        let slot_size = helper.slot_size();
        table.push(SPACE);
        // 先将 属于数组部分的vtable 部分添加上
        let len = self.len();
        let mut vtable = vec![0u8;(len + 1) * slot_size + 1];
        
        table.append(&mut vtable);
        
        let child_pivot_index = table.len() - 1;
        table[child_pivot_index] = len as u8;
        println!("Vec序列化");
        for i in 0..len {
            self[i].serialize(table, child_pivot_index, i, helper);
        }
        if pivot_index != 0 {
            // 更新father的vtable
            let max = table[pivot_index] as usize;
            let offset = child_pivot_index - pivot_index;
            for i in 0..slot_size {
                table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
            }
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
/// 
/// # Examples
/// 
/// ```
/// #[macro_use]
/// extern crate easybuffers;
/// 
/// use std::collections::HashMap;
/// use easybuffers::helper::{ Table, HyperHelper };
/// 
/// #[derive(PartialEq,Clone,Default,Debug)]
/// struct TestMap {
///     author: String,
///    map: HashMap<isize, String>,
///     boolean: bool
/// }
/// realize_table! {
///     3, TestMap { 
///         author: String,
///         map: HashMap,
///         boolean: bool
///     }
/// }
/// fn main() {
///     let mut map = HashMap::new();
///     let helper = HyperHelper::new(2); // 设置2字节表示数据偏移量
///     map.insert(-100, String::from("Value"));
///     map.insert(122222222222, String::from("Rust"));
///     let mut instance = TestMap {
///         author: String::from("SleepPerformer"),
///         map: map,
///         boolean: true
///     };
///     let mut bytes = Vec::with_capacity(1024);
///     instance.serialize(&mut bytes,0,0,&helper);
///     HyperHelper::push_pivot(3 ,&mut bytes,&helper); 
///     let mut data = bytes;
///     let pivot = data.pop().unwrap() as usize;
///     let de_instance = TestMap::deserialize(&data, pivot, pivot, 0, &helper);
///     println!("map is {:?}", de_instance.map);
/// }
/// ```
impl<K, V> Table for HashMap<K, V, RandomState> // 无序的 map
where K: Eq + Hash + Table + Clone, V: Table + Default {
    fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> HashMap<K, V, RandomState> {
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
            let inner_pivot = pivot + offset;
            let mut inner_offset_0 = 0;
            let mut inner_offset_1 = 0;
            let mut scale = 1;
            for j in 0..slot_size {
                inner_offset_0 += bytes[inner_pivot - slot_size*(2 - 0 + 1)+ j] as usize*scale;
                inner_offset_1 += bytes[inner_pivot - slot_size*(2 - 1 + 1)+ j] as usize*scale;
                scale *= 256;
            }
            let child_pivot_0 = inner_pivot + inner_offset_0;
            let key = K::deserialize(bytes, child_pivot_0, inner_pivot, 0, helper);

            if inner_offset_1 != 0 {
                let child_pivot_1 = inner_pivot + inner_offset_1;
                let value = V::deserialize(bytes, child_pivot_1, inner_pivot, 1, helper);
                map.insert(key, value);
            } else {
                map.insert(key, Default::default());
            }
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
            i += 1;
            // 对key进行转化
            let mut key = (*key).clone();// 可以优化
            key.serialize(table, inner_child_pivot_index, 0, helper);
            value.serialize(table, inner_child_pivot_index, 1, helper);
        }
        // 把len 部分更新了
        let len = table.len() - 1 - child_pivot_index;
        for i in 0..slot_size {
            table[child_pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
        }
        // 更新上级的 table
        if pivot_index != 0 {
            // 算出 child 和 pivot 的距离
            let max = table[pivot_index] as usize;
            
            let offset = child_pivot_index - pivot_index;
            for i in 0..slot_size {
                table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
            }
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
/// 自动实现 `Table` trait 按要求输入字段总数，每个字段名以及类型
/// 
/// # Examples
/// 
/// ```
/// #[macro_use]
/// extern crate easybuffers;
/// extern crate time;
/// 
/// use easybuffers::helper::{ Table, HyperHelper };
/// 
/// #[derive(PartialEq,Clone,Default,Debug)]
/// struct TestMessage {
///     field_0: Option<String>,
///     field_1: Option<String>,
///     field_2: Option<bool>,
///     field_3: Option<String>,
///     field_4: Option<bool>,
///     field_5: Option<String>,
///     field_6: Option<String>,
///     field_7: Option<u32>,
///     field_8: Option<String>,
///     field_9: Option<String>,
///     field_10: Option<bool>
/// }
/// realize_table! {
///     11, TestMessage { 
///         field_0 : Option,
///         field_1 : Option,
///         field_2 : Option,
///         field_3 : Option,
///         field_4 : Option,
///         field_5 : Option,
///         field_6 : Option,
///         field_7 : Option,
///         field_8 : Option,
///         field_9 : Option,
///         field_10 : Option
///     }
/// }
/// ```
#[macro_export]
macro_rules! realize_table {
    ( $num:expr, $name:ident { $( $fname:ident : $ftype:ident),* } ) => {
        impl Table for $name {

            fn deserialize (bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> $name {
                let mut instance:$name = Default::default();
                let mut index = 0;
                $(  
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
                let slot_size = helper.slot_size(); // 需要全局定义
                table.push(255u8); 
                table.append(&mut vec![0u8;(field_num+1)*slot_size+1]);
                let child_pivot_index = table.len() - 1;
                table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
                // 更新每个字段
                let mut index = 0;
                $(
                    self.$fname.serialize(table, child_pivot_index, index, helper);
                    index += 1;
                )*
                if pivot_index != 0 {
                    let max = table[pivot_index] as usize;
                    let offset = child_pivot_index - pivot_index;
                    for i in 0..slot_size {
                        table[pivot_index - slot_size*(1+max - position)+i] = ((offset >> i*8) & 0xff) as u8;
                    }
                    if position == max - 1 {
                        let len = table.len() - 1 - pivot_index;
                        for i in 0..slot_size {
                            table[pivot_index - slot_size + i] = ((len >> i*8) & 0xff) as u8;
                        }
                    }
                }
            }
        }
    }
}