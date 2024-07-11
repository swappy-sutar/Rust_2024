fn main() {
    // u = Unsigned.
    let num:u8 = 5;
    println!("this is the number: {}",num);

    // i = Signed.
    let mut num1:i8 = 5;
    num1 = 10;
    println!("this is the number: {}",num1);

    // &str   = Fixed length string. -special memory
    let string_literal:&str = "hey this is swapnil,";
    println!("this is the string: {}",string_literal);

    // String = Dynamic length string. - heap memory
    let my_string:String = String::from("Nice to meet you..!!");
    println!("this is the string: {}",my_string);

    //Tupple-----------------------------------------------
    // 52.00

    let emp_info:(&str,u8) = ("swapnil",28);
    let name = emp_info.0;
    let age = emp_info.1;
    println!("name = {} age = {}",name,age);

    //DE-structuring.
    let (name_is,age_is) = emp_info;
    println!("name = {} age = {}",name_is,age_is);

 //---------------------------------------------------------
}


