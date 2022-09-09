fn triangle_num(i: u128) -> u128 {
    i * (i + 1) / 2
}

// returns number of digits in numbers range i.e. 3 = 112123 >> 6 digits
fn calc_digits_to_tip(num: u128) -> u128 {
    let mut answer = triangle_num(num);

    for n in 1..(num.to_string().len()) {
        // create integer of length n of only digit 9
        let nines_vector: Vec<char> = vec!['9'; n];
        let nines_int: u128 = nines_vector.iter().collect::<String>().parse().unwrap();

        answer += triangle_num(num - nines_int)
    }
    answer
}

fn find_range(start: u128, target: u128, stop: u128) -> (u128, u128, u128) {
    let new_range = (stop - start) / 2;
    let middle_bound = calc_digits_to_tip(start + new_range);

    while stop - start > 1 {
        if target <= middle_bound {
            return find_range(start, target, start + new_range);
        } else {
            return find_range(start + new_range, target, stop);
        }
    }

    (start, target, stop)
}

fn brute_get_series_nth_digit(num: usize) -> Option<u32> {
    let mut num_string: String = String::from("");
    for i in 1..=num {
        num_string.push_str(&(i.to_string()));
        if num_string.len() >= num {
            let digit_vec: Vec<char> = num_string.to_string().chars().collect();
            let answer = digit_vec[num - 1].to_digit(10);
            return answer;
        }
    }

    return None;
}

#[allow(dead_code)]
fn locate_digit(integer: u128) -> Option<u32> {
    let variance = find_range(0, integer, integer);
    let digit: u128 = integer - calc_digits_to_tip(variance.0);

    if digit.to_string().len() > 1 {
        return brute_get_series_nth_digit(digit as usize);
    } else {
        return Some(digit as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_num() {
        assert_eq!(triangle_num(0), 0);
        assert_eq!(triangle_num(1), 1);
        assert_eq!(triangle_num(5), 15);
        assert_eq!(triangle_num(6), 21);
        assert_eq!(triangle_num(7), 28);
        assert_eq!(triangle_num(55346), 1531617531);
        assert_eq!(triangle_num(64346), 2070236031);
    }

    #[test]
    fn test_calc_digits_to_tip() {
        assert_eq!(calc_digits_to_tip(0), 0);
        assert_eq!(calc_digits_to_tip(1), 1);
        assert_eq!(calc_digits_to_tip(2), 3);
        assert_eq!(calc_digits_to_tip(3), 6);
        assert_eq!(calc_digits_to_tip(4), 10);
        assert_eq!(calc_digits_to_tip(10), 56);
    }

    #[test]
    fn test_find_range() {
        assert_eq!(find_range(0, 100, 100), (12, 100, 13));
        assert_eq!(find_range(0, 10000000, 10000000), (2472, 10000000, 2473));
    }

    #[test]
    fn test_brute_get_series_nth_digit() {
        assert_eq!(brute_get_series_nth_digit(16), Some(1));
        assert_eq!(brute_get_series_nth_digit(19), Some(4));
        assert_eq!(brute_get_series_nth_digit(169), Some(9));
        assert_eq!(brute_get_series_nth_digit(610), Some(2));
        assert_eq!(brute_get_series_nth_digit(1888), Some(6));
        assert_eq!(brute_get_series_nth_digit(6604), Some(2));
        assert_eq!(brute_get_series_nth_digit(25570), Some(6));
        assert_eq!(brute_get_series_nth_digit(38471), Some(8));
        assert_eq!(brute_get_series_nth_digit(147256), Some(1));
    }

    #[test]
    fn test_locate_digit() {
        assert_eq!(locate_digit(0), Some(0));
        assert_eq!(locate_digit(1), Some(1));
        assert_eq!(locate_digit(2), Some(1));
        assert_eq!(locate_digit(3), Some(2));
        assert_eq!(locate_digit(4), Some(1));
        assert_eq!(locate_digit(10), Some(4));
        assert_eq!(locate_digit(10u128.pow(2)), Some(1));
        assert_eq!(locate_digit(10u128.pow(3)), Some(4));
        assert_eq!(locate_digit(10u128.pow(4)), Some(9));
        assert_eq!(locate_digit(10u128.pow(5)), Some(2));
        assert_eq!(locate_digit(10u128.pow(6)), Some(6));
        assert_eq!(locate_digit(10u128.pow(7)), Some(2));
        assert_eq!(locate_digit(10u128.pow(8)), Some(6));
        assert_eq!(locate_digit(10u128.pow(9)), Some(8));
        assert_eq!(locate_digit(10u128.pow(10)), Some(1));
        // assert_eq!(locate_digit(10u128.pow(11)), Some(1));
        // assert_eq!(locate_digit(10u128.pow(12)), Some(9));
        // assert_eq!(locate_digit(10u128.pow(13)), Some(8));
        // assert_eq!(locate_digit(10u128.pow(14)), Some(3));
        // assert_eq!(locate_digit(10u128.pow(15)), Some(7));
        // assert_eq!(locate_digit(10u128.pow(16)), Some(6));
        // assert_eq!(locate_digit(10u128.pow(17)), Some(1));
    }
}
