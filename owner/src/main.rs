fn main() {
    let mut x = 5;
    let y = x;
    x = 3;
    println!("x value is {}", x);
    println!("y value is {}", y);
    // let s1 = String::from("hello");
    // let s2 = s1;
    // compile error because s1 value brrowed to s2
    // println!("s1 value is {}", s1);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 value is {}, s2 value is {}", s1, s2);

    let s1 = String::from("takes_ownership");
    takes_ownership(s1);
    // compile error value borroed here after move
    // println!("s1 value is {}", s1);

    let i = 5;
    makes_copy(i);
    println!("i value is {}", i);

    let s1 = gives_ownership();
    println!("gives_ownership return value is {}", s1);

    let s2 = String::from("hogehoge");
    let s3 = gives_and_takeback_ownership(s2);
    
    // compile error value borroed here after move
    // println!("s2 value is {}", s2);
    println!("s3 value is {}", s3);

    let (x, s) = calculate_length(String::from("hoge"));
    println!("x value is {}, s value is {}", x, s);
    
    let s1 = String::from("hogehoge");
    let len = calculate_length2(&s1);
    println!("s1 value is {}, len value is {}", s1, len);

    let s = String::from("hogehoge");
    change(&s);

    let mut s = String::from("fuga");
    let s1 = &mut s;
    // compile error because borrowed twice
    // let s2 = &mut s;
    // println!("{}, {}", s1, s2);
    let mut s = String::from("bar");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("r2 value is {}", r2);

    let del = delegate();

    let s = String::from ("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let word = find_words(&s);
    println!("{}", word);
    // compile error mutable borrow occures here
    // s.clear();
    println!("{}", word);

    let s = String::from("hello");
    param_string(&s);
    param_slice(&s);
    
}

fn takes_ownership(s: String) {
    println!("string value is {}", s);
}

fn makes_copy(i: i32) {
    println!("i value is {}", i);
}

fn gives_ownership() -> String {
    String::from("gives_ownership")
}


fn gives_and_takeback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(s: &String) {
    // compile error because reference cannot be changed
    // s.push_str("string");
}

fn delegate() -> String {
    let s = String::from("delegate");

    s

}

fn find_words(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn param_string(s: &String) {

}

fn param_slice(s: &str) {

}