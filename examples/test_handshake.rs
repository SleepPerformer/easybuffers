#[macro_use]
extern crate easybuffers;
use easybuffers::helper::{ Table, HyperHelper, VecU8 };
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Propose {
    // message fields
    pub rand: VecU8, // 产生的随机数，用于生成 session key
    pub pubkey: VecU8, // 服务器返回证书的公钥
    pub exchanges: String, // 生成最终堆成密钥的DH算法 ECDH
    pub ciphers: String, // 使用加密的算法, 加密protocol
    pub hashes: String, // hash protocol 摘要长度，哈希算法？用于验证证书公钥是否合法
    pub serialization: String // 签名成功后进行的序列化方案选择
}
// impl Table for Propose {
//     fn deserialize(bytes: &Vec<u8>, pivot: usize, help_pivot: usize, position: usize, helper: &HyperHelper) -> Propose {
//         let mut instance:Propose = Default::default();
//         match HyperHelper::child_pivot(bytes, pivot, 0, helper) {
//             // List 先不考虑
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.rand = VecU8::deserialize(bytes, child_pivot, pivot, 0, helper);// 传引用，这样省时间，到需要转换基本数据 才调用 to_vec
//             } 
//         }
//         match HyperHelper::child_pivot(bytes, pivot, 1, helper) {
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.pubkey = VecU8::deserialize(bytes, child_pivot, pivot, 1, helper);
//             } 
//         }
//         match HyperHelper::child_pivot(bytes, pivot, 2, helper) {
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.exchanges = String::deserialize(bytes, child_pivot, pivot, 2, helper);
//             } 
//         }
//         match HyperHelper::child_pivot(bytes, pivot, 3, helper) {
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.ciphers = String::deserialize(bytes, child_pivot, pivot, 3, helper);
//             } 
//         }
//         match HyperHelper::child_pivot(bytes, pivot, 4, helper) {
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.hashes = String::deserialize(bytes, child_pivot, pivot, 4, helper);
//             } 
//         }
//         match HyperHelper::child_pivot(bytes, pivot, 5, helper) {
//             None => (), // 不做任何操作
//             Some(child_pivot) => {
//                 instance.serialization = String::deserialize(bytes, child_pivot, pivot, 5, helper);
//             } 
//         }
        
//         instance
//     }
//     fn serialize(&mut self, table: &mut Vec<u8>, pivot_index:usize, position: usize, helper: &HyperHelper) {
//         let field_num = 6usize; // 需要外部传入
//         let slot_size = 4usize; // 需要全局定义
//         table.push(255u8); 
//         let mut vec = vec![0u8;(field_num+1)*slot_size+1];
//         // println!("tem len is {}", (field_num+1)*slot_size+1);
//         table.append(&mut vec);
//         // println!("ok");
//         let child_pivot_index = table.len()  - 1;
//         table[child_pivot_index] = field_num as u8; // 这里可以好好斟酌下
//         // 更新每个字段
        
//         self.rand.serialize(table, child_pivot_index, 0, helper);
//         // println!("ok, 下一个序列化pubkey, table len is {:?}, rand len is {:?}", table.len(), self.rand.len());
//         self.pubkey.serialize(table, child_pivot_index, 1, helper);
//         // println!("ok, 下一个序列化exchanges");
//         self.exchanges.serialize(table, child_pivot_index, 2, helper);
//         self.ciphers.serialize(table, child_pivot_index, 3, helper);
//         self.hashes.serialize(table, child_pivot_index, 4, helper);
//         self.serialization.serialize(table, child_pivot_index, 5, helper);
//         // 更新最终的长度
    
//         if pivot_index != 0 {
//             // 更新father的vtable
//             // 算出 child 和 pivot 的距离
//             let max = table[pivot_index] as usize;
//             let offset = child_pivot_index - pivot_index;
//             table[pivot_index - slot_size*(1+max - position)] = (offset & 0xff) as u8;
//             table[pivot_index - slot_size*(1+max - position)+1] = ((offset >> 8) & 0xff) as u8;
            
//             if position == max - 1 {
//                 // 要更新father的len
//                 let len = table.len() - 1 - pivot_index;
//                 table[pivot_index - 2] = (len & 0xff) as u8;
//                 table[pivot_index - 1] = ((len >> 8) & 0xff) as u8;
//             }
//         }
//     }
// }
realize_table! {
    6, Propose { 
        rand: VecU8, 
        pubkey: VecU8, 
        exchanges: String,
        ciphers: String, 
        hashes: String, 
        serialization: String
    }
}
fn main() {
    let mut proposition = Propose {
        rand: VecU8::init_with_vec_u8(vec![79u8, 225, 3, 147, 227, 224, 78, 144, 139, 121, 78, 191, 88, 83, 17, 158]), 
        // rand: VecU8::init_with_vec_u8(Vec::new()), 
        pubkey: VecU8::init_with_vec_u8(vec![8u8, 0, 18, 166, 2, 48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15, 0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 179, 104, 229, 19, 162, 162, 249, 179, 37, 173, 24, 127, 77, 59, 247, 120, 253, 118, 248, 57, 176, 73, 36, 253, 243, 194, 235, 204, 0, 248, 56, 29, 244, 117, 53, 246, 134, 26, 148, 220, 181, 111, 55, 64, 202, 223, 160, 21, 211, 228, 30, 182, 83, 245, 152, 15, 47, 157, 175, 110, 98, 231, 67, 152, 6, 198, 249, 229, 240, 92, 79, 12, 157, 78, 77, 118, 52, 71, 181, 31, 56, 106, 16, 48, 128, 37, 212, 177, 33, 114, 36, 215, 201, 130, 55, 80, 21, 133, 135, 17, 180, 81, 232, 244, 3, 67, 80, 131, 93, 22, 197, 201, 30, 192, 1, 27, 163, 51, 96, 65, 164, 132, 4, 159, 158, 227, 232, 76, 213, 215, 253, 222, 20, 162, 13, 140, 176, 87, 29, 173, 128, 14, 137, 243, 72, 216, 205, 26, 169, 75, 130, 48, 28, 120, 201, 201, 192, 116, 244, 52, 180, 237, 193, 20, 77, 153, 243, 154, 214, 154, 41, 118, 246, 252, 36, 153, 43, 98, 102, 91, 165, 167, 153, 122, 147, 152, 97, 50, 163, 141, 178, 237, 18, 51, 243, 156, 84, 236, 13, 111, 236, 198, 212, 86, 165, 240, 211, 37, 186, 132, 31, 93, 70, 104, 168, 152, 171, 89, 126, 132, 219, 23, 183, 57, 194, 29, 110, 89, 145, 218, 78, 166, 154, 26, 255, 153, 105, 109, 128, 80, 0, 192, 43, 101, 69, 227, 199, 241, 218, 67, 130, 157, 233, 244, 62, 253, 2, 3, 1, 0, 1]),
        exchanges: String::from("P-256,P-384"), 
        ciphers: String::from("AES-128,AES-256"),
        hashes: String::from("SHA256,SHA512"),
        serialization: String::from("hyper")
    };
    let helper = HyperHelper::new(4); // 设置4字节表示数据偏移量 
    // let mut proposition_bytes = Vec::with_capacity(1024);
    let mut proposition_bytes = Vec::new();
    println!("准备序列化");
    proposition.serialize(&mut proposition_bytes, 0, 0, &helper);
    HyperHelper::push_pivot(6 ,&mut proposition_bytes, &helper);
    println!("bytes is {:?}", proposition_bytes);
    // deser
    let pivot = proposition_bytes.pop().unwrap() as usize;
    let de_instance = Propose::deserialize(&proposition_bytes, pivot, pivot, 0, &helper);
    assert_eq!(de_instance.pubkey.to_vec_u8(), vec![8u8, 0, 18, 166, 2, 48, 130, 1, 34, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 130, 1, 15, 0, 48, 130, 1, 10, 2, 130, 1, 1, 0, 179, 104, 229, 19, 162, 162, 249, 179, 37, 173, 24, 127, 77, 59, 247, 120, 253, 118, 248, 57, 176, 73, 36, 253, 243, 194, 235, 204, 0, 248, 56, 29, 244, 117, 53, 246, 134, 26, 148, 220, 181, 111, 55, 64, 202, 223, 160, 21, 211, 228, 30, 182, 83, 245, 152, 15, 47, 157, 175, 110, 98, 231, 67, 152, 6, 198, 249, 229, 240, 92, 79, 12, 157, 78, 77, 118, 52, 71, 181, 31, 56, 106, 16, 48, 128, 37, 212, 177, 33, 114, 36, 215, 201, 130, 55, 80, 21, 133, 135, 17, 180, 81, 232, 244, 3, 67, 80, 131, 93, 22, 197, 201, 30, 192, 1, 27, 163, 51, 96, 65, 164, 132, 4, 159, 158, 227, 232, 76, 213, 215, 253, 222, 20, 162, 13, 140, 176, 87, 29, 173, 128, 14, 137, 243, 72, 216, 205, 26, 169, 75, 130, 48, 28, 120, 201, 201, 192, 116, 244, 52, 180, 237, 193, 20, 77, 153, 243, 154, 214, 154, 41, 118, 246, 252, 36, 153, 43, 98, 102, 91, 165, 167, 153, 122, 147, 152, 97, 50, 163, 141, 178, 237, 18, 51, 243, 156, 84, 236, 13, 111, 236, 198, 212, 86, 165, 240, 211, 37, 186, 132, 31, 93, 70, 104, 168, 152, 171, 89, 126, 132, 219, 23, 183, 57, 194, 29, 110, 89, 145, 218, 78, 166, 154, 26, 255, 153, 105, 109, 128, 80, 0, 192, 43, 101, 69, 227, 199, 241, 218, 67, 130, 157, 233, 244, 62, 253, 2, 3, 1, 0, 1]);

    let helper = HyperHelper::new(4); // 设置4字节表示数据偏移量 
    let mut fields = vec![2;1];
    let root = pivot;
    let (child, help, position) = HyperHelper::any_field(&mut fields, &proposition_bytes, root, &helper).unwrap();
    println!("exchanges (child, help, position) is {:?}", (child, help, position));
    let list = String::deserialize(&proposition_bytes, child, help, position, &helper);
    println!("exanges is {:?}", list);
}