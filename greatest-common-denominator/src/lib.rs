fn euclidean_alg(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn parse_line(line: String) -> Vec<u64> {
    let mut int_vec: Vec<u64> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap().abs() as u64)
        .collect();
    int_vec.retain(|x| *x != 0);

    int_vec
}

#[allow(unused_variables)]
pub fn gcd(input: String) -> u64 {
    let int_vec: Vec<u64> = parse_line(input);

    let mut start = int_vec[0];
    let mut second = 1;
    let stop = int_vec.len();

    while second != stop && start != 1 {
        start = euclidean_alg(start, int_vec[second]);
        second += 1;
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd("-2 4   8".to_string()), 2);
        assert_eq!(gcd("15 0 -5".to_string()), 5);
        assert_eq!(gcd("6 20 25 5 30".to_string()), 1);
        assert_eq!(gcd("        5 20".to_string()), 5);
        assert_eq!(gcd("     28 21952 49 294 3822".to_string()), 7);
    }
}
