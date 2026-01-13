//error handling
// using PANIC, the program will start unwindling the program from the start

use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let greting_file_result = File::open("hello.txt");

    let greeting_file = match greting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating th file: {e:?}"),
            },
            _ => {
               panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

// using generics
fn largest<T>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item>largest{
            largest = item;
        }
    }
    largest
}


fn main_v1(){
    let greting_file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating the file: {error:?}");
            })
        } else{
            panic!("Problem opening the file: {error:?}");
        }
    });

    // generic types, traits
    let number_list = vec![34,50,25,100,65];
    let mut largest = &number_list[0];
    for number in &number_list{
        if number > largest{
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    
}

// fn main(){
//     panic!("crash and burn")
// }