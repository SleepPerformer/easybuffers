#使用Rust实现easybuffers

需要手动遍历序列化的实例的每个字段，但是序列化需要更大空间

类型暂时仅支持 String, bool, u8, u16, u32, u64, HasmMap, Vec， Struct

直接定位解析数据位置，速度更快，详情参考 examples

#Structure

                HyperBuffer bytes形式 结构示意图  
      
SPACE         vtable     | data_len | pivot | SPACE+data0  | SPACE+data1  | ~ |

        slot0  slot1  ~  

0xff | 2bytes 2bytes ~~~ | 2bytes   | 1byte | 0xff+n_bytes | 0xff+n_bytes | ~ | 

       Pivot with offset                    | All data                        | 

#Future to do

支持更多基本类型的转化，针对Map的转化可以优化

允许slot_size 动态设置，修改偏移量计算方式

实现 当修改序列化对象的部分字段 可以在已生成的bytes上修改，无需重新生成

#Issue

出现反序列化失败的情况 很有可能是计算偏移量的类型是u8 定位将其改为 usize 类型