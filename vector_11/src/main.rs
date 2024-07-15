fn main() {

    // let mut v:Vec<i32> = Vec::new(); //type 1 
    let mut v = Vec::<i32>::new(); //type 2

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    println!("Vector v = {:?}",v);
//--------------------------------------------
    let mut v = vec![1,2,3,4,5,6];
    v.push(10);
    println!("Vector v = {:?}",v);
//--------------------------------------------
//  vector with functions

let vec_fun: Vec<&str> = Vec!["swap","sam","Raj"];
     vec_with_fun(&vec_fun);
    //  println!("Vector with main functions = {:?}",vec_fun);
}


fn vec_with_fun(vec2: &Vec<&str>){
    println!("Vector with vec_with_fun() functions = {:?}",vec2);
      
}