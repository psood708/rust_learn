fn main() {
    let s1 = String::from("cudbamgru");
    let len = calc_lng_v2(&s1);
    println!("The length of '{s1}' is {len}.")
}

// fn another_func(x:i32){
//     println!("Value of x is : {x}");
// }

// fn plus_one(x:i32) -> i32{
//     x+1
// }
fn luping(){
    let number = 3;
    if number < 5 {
        println!("Condition is true");
    }else{
        println!("Condition is false");
    }
}
fn prt2(){
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is : {number}")
}
fn fine_loop(){
    let mut count = 0;
    'counting_up:loop{
        println!("Count: {count}");
        let mut remaining = 10;

        loop{
            println!("Remaining : {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
    println!("End count = {count}");

}
fn looping_idx(){
    // let a =[10,20,30,40,50,60];
    // let mut index = 0;
    // while index < 5 {
    //     println!("Value is : {} ",a[index]);
    //     index+=1;
    // }

    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!");
}
fn fibonnaci(x:u32)-> u32{
    if x==0{
        return 0;
    }else if x==1{
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=x{
        result=a+b;
        a = b;
        b = result;
    }
    result

}
fn calc_lng(s:String) -> (String,usize){
    let length= s.len();
    (s,length)
}
// References and Borrowing
// calc_leng_v2 : the following function would talk reference instead of taking ownership.
fn calc_lng_v2(s:&String) -> usize{
    s.len()
}
// mutble references allow us to change the borrowed data
// we can use scopes to use mutable and immutable references
// we can only have one mutable reference to a variable

// summer of bitcoinn
