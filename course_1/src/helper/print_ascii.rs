/**
 * 用while实现
 * 指定开始和结束字符，打印之间的ascii表的字符
 */
pub fn print_ascii_by_while(start: char,end: char){
    let start_dec: u8 = start as u8;
    let end_dec: u8 = end as u8;
    println!("打印{}({})到{}({})之间的字符！",start,start_dec,end,end_dec);
    
    let mut a = start_dec;
    if start_dec >= end_dec {
        while a >= end_dec  {
            println!("{}",a as char);
            a = a - 1;
        }
    } else {
        while a <= end_dec  {
            println!("{}",a as char);
            a = a + 1;
        }
    }
}
    
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
