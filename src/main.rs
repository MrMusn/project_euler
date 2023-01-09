fn main() {
    println!(
        "Exercise #1: Sum of multiples of 3 and 5 below 1000 = {}",
        project_euler::euler_utils::sum_vec_i32(
            Vec::from_iter(
                project_euler::find_multiples_below_target(vec![3,5], 1000)
            )
        )
    );
    println!(
        "Exercise #2: Sum of all even Fibonacci numbers below 4 million = {}", 
        project_euler::euler_utils::sum_vec_i32(
            Vec::from_iter(
                project_euler::find_even_fibonacci_numbers_below_target(4000000)
            )
        )
    );
    println!(
        "Exercise #3: Largest prime factor of the number 600851475143 = {}", 
        project_euler::find_largest_prime_factor_of_n(600851475143)
    );
}