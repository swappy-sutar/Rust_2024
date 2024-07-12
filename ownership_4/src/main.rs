fn main() {
    let x:String = String::from("swapnil");
    process_string(x);
    // println!("the value of x in main is =  {}",x);
 
//---------------------------------------------
    let s1:String = get_string(); //S1, when is the owner of hello
    println!("this is s1 =  {}",s1);

    let s2:String = String::from("World");//s2, two is the owner of world
    let s3:String =send_get_string(s2);

    println!("this is s3 =  {}",s3);
//---------------------------------------------

let a1:String = String::from("hello"); 
let len= calculate_length(a1.clone()); //Transferring the clone of a1;
println!("the length of {} is {}",a1,len);

}

fn process_string(item:String){
    println!("the value of item in process_string() is =  {}",item);
}
//---------------------------------------------

fn get_string()->String{
    let new_string:String = String::from("Hello");
    return new_string;
} 

fn send_get_string(received_string:String)->String{
    return received_string;
}
//---------------------------------------------
//  1.58.00

fn calculate_length(s:String)->usize{
    return s.len();
}