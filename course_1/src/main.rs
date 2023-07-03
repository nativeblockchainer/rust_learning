mod helper;
fn main() {
    // 大于'a'到'Z'之间的字符
    helper::print_ascii::print_ascii_by_while('a','Z');
    // 大于'A'到'z'之间的字符
    helper::print_ascii::print_ascii_by_while('A','z');
    
    // 大于'a'到'Z'之间的字符
    helper::print_ascii::print_ascii_by_for('a','Z');
    // 大于'A'到'z'之间的字符
    helper::print_ascii::print_ascii_by_for('A','z');
}
