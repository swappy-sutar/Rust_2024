fn main() {
    changeindex();
    fix_sized();

//--------------------------------------------------
    let arr: [&str; 3] = ["swapnil", "sam", "swappy"];
    string_type(arr);
    println!("fn main array = {:?}", arr);

    let mut arr1: [&str; 3] = ["swapnil", "sam", "swappy"];
    string_type1(&mut arr1);
    println!("fn main array2 = {:?}", arr1);
//--------------------------------------------------

}

fn changeindex() {
    let mut arr2: [u8; 3] = [1, 2, 3];
    arr2[1] = 10;
    println!("arr2 = {:?}", arr2);
    println!("length of array: {}", arr2.len());
}

fn fix_sized() {
    let arr1: [u8; 5] = [1, 2, 3, 4, 5];
    println!("arr1 = {}", arr1[0]);
    println!("arr1 = {:?}", arr1);
    println!("length of array: {}", arr1.len());
}

//--------------------------------------------------

fn string_type(arr: [&str; 3]) {
    println!("fn string_type array = {:?}", arr);
}

fn string_type1(arr4: &mut [&str; 3]) {
    arr4[0] = "Raj";
    println!("fn string_type1 array = {:?}", arr4);
}
//--------------------------------------------------


