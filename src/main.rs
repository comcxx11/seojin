use rand::Rng;

fn main() {
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