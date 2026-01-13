enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin)-> u8 {
    match coin{
        Coin::Penny =>{
            println!("Lucky penny!");
            1
        }
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else{
        return None;
    };
    if state.existed_in(1900){
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is reltively new."))
    }
}