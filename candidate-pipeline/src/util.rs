
pub fn short_type_name(full_name: &str) -> &str {
    // 找到最后一个 "::" 的起始索引
    if let Some(pos) = full_name.rfind("::") {
        // 返回双冒号之后的部分
        &full_name[pos + 2..]
    } else {
        // 如果没有双冒号（比如原始类型 i32），则返回原字符串
        full_name
    }
}