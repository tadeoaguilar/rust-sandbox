




fn main() {
    let a = divide(45,9);
    let b = divide(10,0);
    let c = divide(145,9);
    if let Ok(v) = a {
        println!("val = {}",v)
    }

    
    
}

fn divide(a: i32, b: i32) -> Result<i32,String> {
    if b == 0{
        return Err("Cannot divide to zero".to_string());
    }
   Ok(a/b)
}