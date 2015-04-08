

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


fn main() {
    match std::env::args().nth(1) {
        None => {
            panic!("Missing number to factorise");
        },
        Some(n_str) => {
            match n_str.parse() {
                Err(_) => {
                    panic!("Invalid number: {}", n_str);
                },
                Ok(n) => {
                    for factor in factors_of(n) {
                        println!("{}", factor);
                    }
                },
            }
        },
    }
}
