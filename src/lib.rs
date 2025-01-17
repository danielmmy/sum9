use std::collections::HashMap;

pub fn sum9_bruteforce(numbers: &Vec<i32>) -> Option<(usize, usize)> {
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 9 {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn sum9_map(numbers: &Vec<i32>) -> Option<(usize, usize)> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(numbers.len());
    for (index, i) in numbers.iter().enumerate() {
        map.insert(*i, index);
    }

    for (k, v) in &map {
        let missing = 9 - k;
        if let Some(v2) = map.get(&missing) {
            if *v2 > *v {
                return Some((*v, *v2));
            }
            return Some((*v2, *v));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum9_bruteforce_tests() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_bruteforce(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }

    #[test]
    fn sum9_map_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }
}
