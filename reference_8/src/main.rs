// fn main() {
//     let mut number:String = String::from("hello");
//     let num1 = &mut number;
//     num1.push_str(" sutar");
//     println!("num1 = {}",num1);

//     let num2 = &num1;
//     println!("num2 = {} ",num2);
//     // println!("num1 = {} num2 = {}",num1 ,num2);

// }


fn main(){
    let mut x= 5;
    x=x+1;
    let y  = &mut x;
    *y = *y + 1;
    println!("x = {}",x);

    let refrence_to_string=create_string_ref();
}

fn create_string_ref()->&String{
    let s:String = String::from("hello");
    return &s;
} 