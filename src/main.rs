use rand::Rng;
use std::io;

fn main() {
    show_bite();
}

fn guess() {
    println!("Guess the number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=101);

    println!("The secret number is : {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn u_int() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let n3: u32 = rng.gen();
    let n4: u64 = rng.gen();
    let n5: u128 = rng.gen();
    println!("Random u8: {} {}", n1, u8::MAX);
    println!("Random u16: {} {}", n2, u16::MAX);
    println!("Random u32: {} {}", n3, u32::MAX);
    println!("Random u64: {} {}", n4, u64::MAX);
    println!("Random u128: {} {}", n5, u128::MAX);

    println!("{}", u8::BITS);
    println!("{}", u16::BITS);
    println!("{}", u32::BITS);
    println!("{}", u64::BITS);
    println!("{}", u128::BITS);

    assert_eq!(u8::MAX, 255);
}
#[warn(dead_code)]
fn print_vec32() {
    let array: Vec<u32> = (1..6).collect();
    println!("range {:?}", array);
}

fn show_bite() {
    let string = "PseudoSecret-Pseudosecret-IMPORTANT-Please-Use-Ur-Own-Key-IMPORTANT";

    for c in string.chars() {
        let encoded_char = c.to_string().into_bytes();
        let byte_length = encoded_char.len();
        println!("Character: {}, Byte Length: {}", c, byte_length);
    }
}