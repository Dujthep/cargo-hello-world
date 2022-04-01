// #[derive(Debug)]
struct Employee {
    name: String,
    age: u32,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {}, value b: {})", self.name, self.age)
    }
}

fn main() {
    let a: f64 = 11_000.55_032;
    let b: u64 = 2110_23;
    let mut c: u64 = 1_0;
    println!("a value is {}", a);
    println!("b value is {}", b);
    println!("c value is {}", c.to_string());

    c = 11;
    println!("c change value to {}", c.to_string());

    let mut z = String::new();
    z.push_str("hello");
    println!("z value is {}", z);
    let x = "hello world";
    println!("x value is {}", x.replace("hello", "hi"));
    let y = String::from("world");
    println!("z plus y value is {}", z + &y);

    let state_code = "AA";
    let state = match state_code {
        "MH" => {
            println!("Found match for MH");
            "Maharashtra"
        }
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown",
    };
    println!("State name is {}", state);

    let employee_a = Employee { name: String::from("A"), age: 25 };
    println!("employee a profile is {}", employee_a);

    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    
    let mut v = vec![1,2,3];
    let v2 = v;
    v = vec![];
    println!("{:?}",v);
}
