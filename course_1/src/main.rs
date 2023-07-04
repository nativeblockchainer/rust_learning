mod helper;
fn main() {
    // 一层module，打印'a'到'Z'之间的字符
    helper::print_ascii_by_for('a','Z');
    //  一层module，打印'A'到'z'之间的字符
    helper::print_ascii_by_for('A','z');

    // 二层module，打印'a'到'Z'之间的字符
    helper::print_ascii::print_ascii_by_while('a','Z');
    // 二层module，打印'A'到'z'之间的字符
    helper::print_ascii::print_ascii_by_while('A','z');
}
