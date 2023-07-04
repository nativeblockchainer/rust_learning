pub mod print_ascii;

/**
 * 用for循环实现
 */
pub fn print_ascii_by_for(start: char,end: char){
    let start_dec: u8 = start as u8;
    let end_dec: u8 = end as u8;
    println!("打印{}({})到{}({})之间的字符！",start,start_dec,end,end_dec);
    
    if start_dec >= end_dec {
        for c in (end..=start).rev() {
            println!("{}",c);
        }
    } else {
        for c in start..=end {
            println!("{}",c);
        }
    }
}