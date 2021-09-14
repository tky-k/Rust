fn main() {
    let mut x = 6;
    println!("x is {}", x);
    // if x is not mut then compile error, because x is immutable variable. but set value twice.
    x = 5;
    println!("x is {}", x);

    let y = 5;
    println!("y is {}", y);

    let y = y + 1;
    println!("y is {}", y);

    let y = y * 2;
    println!("y is {}", y);

    let str = "hoge";
    println!("char is {}", str);
    let str = str.len();
    println!("char is {}", str);

    // i8 can be set between -128 and 127
    let i: i8 = 127;
    // u8 can be set between 0 and 255
    let u: u8 = 255;

    let i:i32 = 1000;
    println!("i is {}", i);

    let c:char = '🐑';
    println!("c is emoji {}", c);

    let tup: (i32, f32, i8) = (500, 2.5, 127);

    let fai = tup.1;
    println!("fai is {}", fai);

    let array = [1,2,3,4,5];
    println!("array is {:?}", array);
    let array = array[0];
    println!("array is {}", array);

    another_function(255, 256);

    println!("{}", ret_five());

    let array = [1,2,3,4,5];
    for a in array.iter() {
        println!("array value is {}", a);
    }

    for a in (1..4).rev() {
        println!("reverse array value is {}", a);
    }

}

fn another_function(x: i32, y: i32) {
    println!("another_function! x value is {}", x);
    println!("another_function! y value is {}", y);

}

fn ret_five() -> i32 {
    5
}