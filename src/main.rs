enum MultiType{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element of v is {third}");

    let third: Option<&i32> = v.get(2); // If .get is outside vector size, it will return None
    match third {
        Some(third) => println!("The third element of v is {third}"),
        None => println!("There is no third element."),
    }

    println!("The second element of v2 is {}", &v2[1]);

    for i in &v {
        println!("Vetor v: {i}");
    }

    for i in &mut v {
        *i += 50;
        println!("Vector v + 50: {i}");
    }

    let row = vec![
        MultiType::Int(3),
        MultiType::Text(String::from("Hello")),
        MultiType::Float(10.12),
    ];

    let mut s = String::from("Intial contents");
    s.push_str(" ADDED CONTENTS");
    println!("{s}");
    s.push('u');
    println!("{s}");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    let s4 = String::from("Test");
    for i in s4.chars() {
        println!("{i}");
    }
    
}
