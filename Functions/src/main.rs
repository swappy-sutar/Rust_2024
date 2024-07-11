fn main() {
    print();
    print_value(5);

    let num1:u8 = 10;
    let num2:u8 = 20;
    let sum = add(num1, num2);
    println!("sum of num1 & num2 is {}", sum);
}

fn print(){
    println!("hey this is print funtion");
}  

fn print_value(item:u8){
    println!("{}",item);
}

fn add(item1:u8,item2:u8)->u8{
    return item1+item2;
}