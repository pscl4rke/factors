

fn factors_of(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    for i in 1..(n+1) {
        if (n % i) == 0 {
            factors.push(i);
        }
    }
    factors
}


#[test]
fn test_factoring_of_prime_numbers() {
    assert_eq!(factors_of(19), vec![1,19]);
}


#[test]
fn test_factoring_of_even_numbers() {
    assert_eq!(factors_of(12), vec![1,2,3,4,6,12]);
}


#[test]
fn test_factoring_of_odd_numbers() {
    assert_eq!(factors_of(15), vec![1,3,5,15]);
}


#[test]
fn test_factoring_of_square_numbers() {
    assert_eq!(factors_of(25), vec![1,5,25]);
}


macro_rules! userfail {
    ($msg:expr) => {{
        use std::io::Write; // for the .write() method
        let prog = std::env::args().nth(0).unwrap_or("ERROR".to_string());
        let line = format!("{}: {}\n", prog, $msg);
        std::io::stderr().write(line.as_bytes()).ok();
        std::process::exit(1);
    }};
}


fn main() {
    let n_str = match std::env::args().nth(1) {
        None => userfail!("Missing number to factorise"),
        Some(n_str) => n_str,
    };
    let n = match n_str.parse() {
        Err(_) => userfail!(format!("Invalid number: {}", n_str)),
        Ok(n) => n,
    };
    for factor in factors_of(n) {
        println!("{}", factor);
    }
}
