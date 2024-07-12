fn main() {
    println!("loops");
    first();
    second();
    third();
}

// in rust there are 3 types of loops 
// 1. loop
// 2. for loop
// 3. while loop

    fn first(){
        // loop
        // loop is a infinite loop

        let mut num = 0;
        loop{
            num += 1;
            println!("loop num: {}", num);
            if num == 10{
                println!("we did it");
                break;
            }
        }
    }

    fn second(){
       //while loop
        println!("while loop");

        let mut num = 0;
        while  num < 10{
            num += 1;
            println!("while number: {}", num);
        }
    }

    fn third(){
        //for loop
        println!("for loop");
        for i in 0..10{
            println!("for loop number: {}", i);
        }
            

    }