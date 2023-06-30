/**
 * 制定开始和结束字符，打印asci表的字符
 */
pub fn print_asci_by(start: char,end: char){
    let start_dec: u8 = start as u8;
    let end_dec: u8 = end as u8;
    println!("打印{}({})到{}({})之间的字符！",start,start_dec,end,end_dec);
    
    if start_dec >= end_dec {
        let mut a = start_dec;
        while a >= end_dec  {
            println!("{}",a as char);
            a = a - 1;
        }
    } else {
        let mut a = start_dec;
        while a <= end_dec  {
            println!("{}",a as char);
            a = a + 1;
        }
    }
    
}