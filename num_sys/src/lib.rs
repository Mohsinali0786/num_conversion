use std::io;
pub fn bin_num ()
{       
    println!("Please input number :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u32 = input.trim().parse().unwrap();
    println!("Decimal number {}",num);
    println!("binary of {} is {:b}",num,num);
    println!("octal of {} is {:o}",num,num);
    println!("hexa decimal of {} is {:x}",num,num);
    
}
