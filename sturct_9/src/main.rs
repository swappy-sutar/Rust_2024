
struct User{
    name:String,
    age:u32,
    current_emp:bool
}

fn main() {
    let mut data = User{
        name:String::from("Rahul"),
        age:20,
        current_emp:true
    };
    data.age =22;
    println!("Name:{}",data.name);
    println!("Age:{}",data.age);
    println!("Current Employee:{}",data.current_emp);
}

 