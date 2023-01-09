#[test]
fn find_largest_prime_factor_of_number() {
    assert_eq!(project_euler::find_largest_prime_factor_of_n(13195), 29);
}

#[test]
fn is_primes() {
    assert_eq!(project_euler::euler_utils::is_prime(2), true);
    assert_eq!(project_euler::euler_utils::is_prime(3), true);
    assert_eq!(project_euler::euler_utils::is_prime(5), true);
    assert_eq!(project_euler::euler_utils::is_prime(7), true);
    assert_eq!(project_euler::euler_utils::is_prime(11), true);
    assert_eq!(project_euler::euler_utils::is_prime(13), true);
    assert_eq!(project_euler::euler_utils::is_prime(17), true);
    assert_eq!(project_euler::euler_utils::is_prime(19), true);

    assert_eq!(project_euler::euler_utils::is_prime(4), false);
    assert_eq!(project_euler::euler_utils::is_prime(6), false);
    assert_eq!(project_euler::euler_utils::is_prime(8), false);
    assert_eq!(project_euler::euler_utils::is_prime(9), false);
    assert_eq!(project_euler::euler_utils::is_prime(12), false);
}

#[test]
fn find_even_fibonacci_numbers_below_target() {
    assert_eq!(project_euler::find_even_fibonacci_numbers_below_target(4000000).iter().sum::<i32>(), 4613732);
}

#[test]
fn find_multiples_below_target() {
    assert_eq!(project_euler::find_multiples_below_target(vec![3,5], 1000).iter().sum::<i32>(), 233168);
}