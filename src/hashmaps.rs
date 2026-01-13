
fn main() {
    
    // enum SpreadsheetCell{
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // let row = vec1[
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    let v = vec![1, 2, 3];

    let third = &v[2]; 
    // println!("The third element is {third}");

    let third_opt = v.get(2);
    match third_opt {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element."),
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    // println!("String is : {} ",s);

    for c in "3a".chars(){
        // println!("{c}");
    }

    // hashmaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),40);

    let team_nm = String::from("Blue");
    let score = scores.get(&team_nm).copied().unwrap_or(0);

    for (key,value) in &scores{
        println!("{key}:{value}");
    }

    // hasmaps v1
    // use std::collections::HashMap;
    // by defult hashmaps is something called SipHash thaat resists DoS attacks
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let cnt = map.entry(word).or_insert(0);
        *cnt +=1;
    }
    println!("{map:?}");

}
    