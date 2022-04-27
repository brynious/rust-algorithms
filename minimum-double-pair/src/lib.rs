fn parse_line(line: &str) -> Vec<usize> {
    let mut int_vec: Vec<usize> = line
        .to_string()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap().abs() as usize)
        .collect();
    int_vec.retain(|x| *x != 0);

    int_vec
}

#[allow(unused_variables)]
pub fn min_double_pair(input: &str) -> Option<usize> {
    let mut sum_vec: Vec<usize> = vec![];
    let mut dup_vec: Vec<usize> = vec![];

    let mut int_vec: Vec<usize> = parse_line(input);

    int_vec.sort();

    for i in 0..int_vec.len() {
        if dup_vec.len() != 0 {
            if int_vec[i] + int_vec[i + 1] > *dup_vec.iter().min().unwrap() {
                break;
            }
        }
        for j in i + 1..int_vec.len() {
            let sum1 = int_vec[i] + int_vec[j];
            if dup_vec.len() != 0 {
                if sum1 > *dup_vec.iter().min().unwrap() {
                    break;
                }
            } else if !sum_vec.contains(&sum1) {
                sum_vec.push(sum1);
            } else {
                dup_vec.push(sum1);
            }
        }
    }

    if dup_vec.len() == 0 {
        return None;
    } else {
        return Some(*dup_vec.iter().max().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_double_pair_works() {
        assert_eq!(min_double_pair("2 1 3 4"), Some(5));
        assert_eq!(min_double_pair("10 20 40 45 5 15 25"), Some(25));
        assert_eq!(min_double_pair("24 23 8 29 31 5"), None);
        assert_eq!(min_double_pair("10 20 50 51 52 53 54 12 15 16"), Some(62));
    }
}
