fn main() {
    let a1:String = String::from("hello"); 
let len= calculate_length(&a1); //Borrow operation
println!("the length of {} is {}",a1,len);
//------------------------------------------

    let s1:String = String::from("hey swappy");
    let r1 = &s1;
    let r2 = &s1;

    println!("r1= {} , r2= {}",r1,r2);
[]
//-----------------------------------------
let mut b1:String = String::from("hey ");
let t1 = &mut b1;
println!("t1= {}",t1);

let t2 = &mut b1;
t2.push_str("code");
println!("t2= {}",t2);
// println!("t1= {} , t2= {}",t1,t2);
}


fn calculate_length(s2:&String)->usize{
    return s2.len();
}
 
//2.20.00