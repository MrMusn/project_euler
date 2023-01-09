#![allow(unused)]

pub mod euler_utils;

use std::{collections::HashSet, vec::{self, IntoIter}};

pub fn find_largest_prime_factor_of_n(mut n: i64) -> i64 {
    let (mut max_prime, mut count): (i64, i64) = (-1, 5);
    factorise_n_by_dividing_divisor_with_n(&mut n, &mut max_prime, 2);
    factorise_n_by_dividing_divisor_with_n(&mut n, &mut max_prime, 3);
    while count as f64 <= (n as f64).sqrt() {
        factorise_n_by_dividing_divisor_with_n(&mut n, &mut max_prime, count);
        factorise_n_by_dividing_divisor_with_n(&mut n, &mut max_prime, (count + 2));
        count += 6;
    }
    if n > 4 { max_prime = n; } max_prime
}

fn factorise_n_by_dividing_divisor_with_n(n: &mut i64, factor: &mut i64, divisor: i64) {
    while *n % divisor == 0 {
        *factor = divisor;
        *n = *n / divisor;
    }
}

pub fn find_even_fibonacci_numbers_below_target(target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![2];
    let (mut n_minus_1, mut n_minus_2, mut n): (i32, i32, i32) = (1, 2, 3);
    while n < target {
        if n % 2 == 0 { result.push(n); }
        (n_minus_1 = n_minus_2, n_minus_2 = n, n = n_minus_1 + n_minus_2);
    }
    result
}

pub fn find_multiples_below_target(multiples: Vec<i32>, target: i32) -> HashSet<i32> {
    let mut result: HashSet<i32> = HashSet::new();
    for n in 1..target {
        multiples.iter().for_each(|i| if n % i == 0 { result.insert(n); });
    }
    result
}