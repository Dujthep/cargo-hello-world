fn main() {
    let a:f64 = 11_000.55_032;
    let b:u64 = 2110_23;
    let mut c:u64 = 1_0;
    println!("a value is {}", a);
    println!("b value is {}", b);
    println!("c value is {}", c.to_string());

    c = 11;
    println!("c change value to {}", c.to_string());
}
