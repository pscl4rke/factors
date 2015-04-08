

fn factors_of(n: usize) -> Vec<usize> {
    vec![1, n]
}


#[test]
fn test_factoring_of_prime_numbers() {
    assert_eq!(factors_of(19), vec![1,19]);
}


fn main() {
    panic!("The CLI has not been implemented yet");
}
