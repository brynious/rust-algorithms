fn calculate_triangle_number(index: u128) -> u128 {
    index * (index + 1) / 2
}

// returns number of digits in numbers range i.e. 3 = 112123 >> 6 digits
fn calculate_total_digits(limit: u128) -> u128 {
    let mut total_digits = calculate_triangle_number(limit);

    for digit_count in 1..(limit.to_string().len()) {
        // create integer of length digit_count of only digit 9
        let nines_vector: Vec<char> = vec!['9'; digit_count];
        let nines_as_int: u128 = nines_vector.iter().collect::<String>().parse().unwrap();

        total_digits += calculate_triangle_number(limit - nines_as_int)
    }
    total_digits
}

fn find_digit_range(start: u128, target: u128, end: u128) -> (u128, u128, u128) {
    let range_midpoint = (end - start) / 2;
    let middle_bound = calculate_total_digits(start + range_midpoint);

    while end - start > 1 {
        if target <= middle_bound {
            return find_digit_range(start, target, start + range_midpoint);
        } else {
            return find_digit_range(start + range_midpoint, target, end);
        }
    }

    (start, target, end)
}

fn get_nth_digit_in_series(n: usize) -> Option<u32> {
    let mut concatenated_numbers: String = String::from("");
    for i in 1..=n {
        concatenated_numbers.push_str(&(i.to_string()));
        if concatenated_numbers.len() >= n {
            let digit_vector: Vec<char> = concatenated_numbers.to_string().chars().collect();
            let nth_digit = digit_vector[n - 1].to_digit(10);
            return nth_digit;
        }
    }

    return None;
}

#[allow(dead_code)]
fn find_digit_at_position(position: u128) -> Option<u32> {
    let range = find_digit_range(0, position, position);
    let digit_position: u128 = position - calculate_total_digits(range.0);

    if digit_position.to_string().len() > 1 {
        return get_nth_digit_in_series(digit_position as usize);
    } else {
        return Some(digit_position as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_triangle_number() {
        assert_eq!(calculate_triangle_number(0), 0);
        assert_eq!(calculate_triangle_number(1), 1);
        assert_eq!(calculate_triangle_number(5), 15);
        assert_eq!(calculate_triangle_number(6), 21);
        assert_eq!(calculate_triangle_number(7), 28);
        assert_eq!(calculate_triangle_number(55346), 1531617531);
        assert_eq!(calculate_triangle_number(64346), 2070236031);
    }

    #[test]
    fn test_calculate_total_digits() {
        assert_eq!(calculate_total_digits(0), 0);
        assert_eq!(calculate_total_digits(1), 1);
        assert_eq!(calculate_total_digits(2), 3);
        assert_eq!(calculate_total_digits(3), 6);
        assert_eq!(calculate_total_digits(4), 10);
        assert_eq!(calculate_total_digits(10), 56);
    }

    #[test]
    fn test_find_digit_range() {
        assert_eq!(find_digit_range(0, 100, 100), (12, 100, 13));
        assert_eq!(find_digit_range(0, 10000000, 10000000), (2472, 10000000, 2473));
    }

    #[test]
    fn test_get_nth_digit_in_series() {
        assert_eq!(get_nth_digit_in_series(16), Some(1));
        assert_eq!(get_nth_digit_in_series(19), Some(4));
        assert_eq!(get_nth_digit_in_series(169), Some(9));
        assert_eq!(get_nth_digit_in_series(610), Some(2));
        assert_eq!(get_nth_digit_in_series(1888), Some(6));
        assert_eq!(get_nth_digit_in_series(6604), Some(2));
        assert_eq!(get_nth_digit_in_series(25570), Some(6));
        assert_eq!(get_nth_digit_in_series(38471), Some(8));
        assert_eq!(get_nth_digit_in_series(147256), Some(1));
    }

    #[test]
    fn test_find_digit_at_position() {
        let mut start;

        assert_eq!(find_digit_at_position(0), Some(0));
        assert_eq!(find_digit_at_position(1), Some(1));
        assert_eq!(find_digit_at_position(2), Some(1));
        assert_eq!(find_digit_at_position(3), Some(2));
        assert_eq!(find_digit_at_position(4), Some(1));
        assert_eq!(find_digit_at_position(10), Some(4));
        assert_eq!(find_digit_at_position(10u128.pow(2)), Some(1));
        assert_eq!(find_digit_at_position(10u128.pow(3)), Some(4));
        assert_eq!(find_digit_at_position(10u128.pow(4)), Some(9));
        assert_eq!(find_digit_at_position(10u128.pow(5)), Some(2));
        assert_eq!(find_digit_at_position(10u128.pow(6)), Some(6));
        assert_eq!(find_digit_at_position(10u128.pow(7)), Some(2));
        assert_eq!(find_digit_at_position(10u128.pow(8)), Some(6));
        assert_eq!(find_digit_at_position(10u128.pow(9)), Some(8));
        assert_eq!(find_digit_at_position(10u128.pow(10)), Some(1));
        assert_eq!(find_digit_at_position(10u128.pow(11)), Some(1));
        assert_eq!(find_digit_at_position(10u128.pow(12)), Some(9));
        assert_eq!(find_digit_at_position(10u128.pow(13)), Some(8));

        start = std::time::Instant::now();
        assert_eq!(find_digit_at_position(10u128.pow(14)), Some(3));
        println!("Time taken for position 10u128.pow(14): {:?}", start.elapsed());

        start = std::time::Instant::now();
        assert_eq!(find_digit_at_position(10u128.pow(15)), Some(7));
        println!("Time taken for position 10u128.pow(15): {:?}", start.elapsed());

        start = std::time::Instant::now();
        assert_eq!(find_digit_at_position(10u128.pow(16)), Some(6));
        println!("Time taken for position 10u128.pow(16): {:?}", start.elapsed());

        start = std::time::Instant::now();
        assert_eq!(find_digit_at_position(10u128.pow(17)), Some(1));
        println!("Time taken for position 10u128.pow(17): {:?}", start.elapsed());

        start = std::time::Instant::now();
        assert_eq!(find_digit_at_position(10u128.pow(18)), Some(1));
        println!("Time taken for position 10u128.pow(18): {:?}", start.elapsed());
    }
}
