fn euclidean_alg(a: usize, b: usize) -> usize {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn parse_line(line: String) -> Vec<usize> {
    let mut int_vec: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap().abs() as usize)
        .collect();
    int_vec.retain(|x| *x != 0);

    int_vec
}

#[allow(unused_variables)]
pub fn gcd(input: &str) -> usize {
    let int_vec: Vec<usize> = parse_line(input.to_string());

    let mut start: usize = int_vec[0] as usize;
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
        assert_eq!(gcd("-2 4   8"), 2);
        assert_eq!(gcd("15 0 -5"), 5);
        assert_eq!(gcd("6 20 25 5 30"), 1);
        assert_eq!(gcd("        5 20"), 5);
        assert_eq!(gcd("     28 21952      49 294 3822"), 7);
        assert_eq!(
            gcd("10_000_000_000_000_000 100_000_000_000_000_000"),
            10000000000000000
        );
    }
}
