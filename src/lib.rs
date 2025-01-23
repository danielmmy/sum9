mod good_multiplier_hash;
mod good_multiplier_hash_i32;
mod no_hash;

use std::{collections::HashMap, hash::BuildHasherDefault};

use good_multiplier_hash::GoodMultiplierHash;
use good_multiplier_hash_i32::GoodMultiplierHashI32;
use no_hash::NoHash;
use rustc_hash::FxBuildHasher;

pub fn sum9_bruteforce(numbers: &[i32]) -> Option<(usize, usize)> {
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 9 {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn sum9_prealloc_vec(numbers: &[i32]) -> Option<(usize, usize)> {
    let max = *numbers.iter().max()?;
    let min = *numbers.iter().min()?;
    let total = if min < 0 { max - min } else { max + min };

    let mut indexes: Vec<i32> = vec![-1; (total + 1) as usize];

    for (index, i) in numbers.iter().enumerate() {
        let mut idx = *i;
        if idx < 0 {
            idx = max - idx;
        }
        indexes[idx as usize] = index as i32;
    }

    for (index, i) in numbers.iter().enumerate() {
        let mut pos = 9 - i;

        if pos < 0 {
            pos = max - pos;
        }

        if pos >= total {
            continue;
        }

        let index2 = indexes[pos as usize];
        if index2 >= 0 {
            let index2 = index2 as usize;
            if index2 > index {
                return Some((index, index2));
            }
            return Some((index2, index));
        }
    }

    None
}

pub fn sum9_map(numbers: &[i32]) -> Option<(usize, usize)> {
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

pub fn sum9_faster_map(numbers: &[i32]) -> Option<(usize, usize)> {
    let mut map = HashMap::with_capacity_and_hasher(numbers.len(), FxBuildHasher);
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

pub fn sum9_good_multiplier_hash_map(numbers: &[i32]) -> Option<(usize, usize)> {
    let mut map = HashMap::with_capacity_and_hasher(
        numbers.len(),
        BuildHasherDefault::<GoodMultiplierHash>::default(),
    );
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

pub fn sum9_good_multiplier_hash_i32_map(numbers: &[i32]) -> Option<(usize, usize)> {
    let mut map = HashMap::with_capacity_and_hasher(
        numbers.len(),
        BuildHasherDefault::<GoodMultiplierHashI32>::default(),
    );
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

pub fn sum9_no_hash_map(numbers: &[i32]) -> Option<(usize, usize)> {
    let mut map =
        HashMap::with_capacity_and_hasher(numbers.len(), BuildHasherDefault::<NoHash>::default());
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
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
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
    fn sum9_prealloc_vec_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_prealloc_vec(&numbers);

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
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }

    #[test]
    fn sum9_faster_map_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_faster_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }

    #[test]
    fn sum9_good_multiplier_hash_map_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_good_multiplier_hash_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }

    #[test]
    fn sum9_good_multiplier_hash_i32_map_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_good_multiplier_hash_i32_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }

    #[test]
    fn sum9_no_hash_map_test() {
        // arrange
        let test_data = vec![
            (vec![1, 2, 3, 4], None),
            (vec![2, 7, 11, 15], Some((0, 1))),
            (vec![-2, 2, 12, 11, 15], Some((0, 3))),
            (vec![2, 12, 11, 15, -2], Some((2, 4))),
            (vec![11, 15, 2, 7], Some((2, 3))),
        ];

        for (numbers, expected) in test_data {
            // act
            let res = sum9_no_hash_map(&numbers);

            // assert
            assert_eq!(res, expected, "Test {:?}", numbers);
        }
    }
}
