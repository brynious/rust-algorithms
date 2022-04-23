// fn euclidean_alg(a: , b) {

// }

#[allow(unused_variables)]
pub fn gcd(input: String) -> i64 {
    let mut int_vec: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap().abs())
        .collect();
    int_vec.retain(|x| *x != 0);
    int_vec.sort();

    if int_vec.len() == 0 {
        return 0;
    }

    let current_gcd: i64 = int_vec[0];

    

    int_vec[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(gcd("-2 4   8".to_string()), 2);
        // assert_eq!(gcd("15 0 -5".to_string()), 5);
        // assert_eq!(gcd("6 20 25 5 30".to_string()), 1);
        // assert_eq!(gcd("     28 21952 49 294 3822".to_string()), 7);
        assert_eq!(102 % 10, 2);

    }
}
