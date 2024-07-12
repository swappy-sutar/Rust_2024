fn main() {
    let a1:String = String::from("hello"); 
let len= calculate_length(a1.clone()); //Transferring the clone of a1;
println!("the length of {} is {}",a1,len);

}


fn calculate_length(s:String)->usize{
    return s.len();
}