// VARIABLE ASSIGMENTS

fn fibonacci(num_one: i32, num_two, i32) -> i32 {
    
}

fn print_fib(numbers: Vec<i32>) {
    for number in &numbers {
        println!("{}", number);
    }
}

fn main() {
    let a_random_number = 10;
    // easy. Looking at the rust docs, I see `mut` being set before the variable name. What is `mut`?
    // let's try it out.
    let mut mut_is_short_for_mutable = 10;
    // ok. Still compiles fine.
    // Let's see what differences I can discern from these two declarations.

    println!("{}", a_random_number + 1);
    println!("{}", mut_is_short_for_mutable + 1);
    // prints fine. Still don't understand what the deal is. I'll try incrementing.
    println!("{}", a_random_number += 1);
    println!("{}", mut_is_short_for_mutable += 1);

    // whoa, slammed with a ton of errors. Maybe it doesn't like me doing that incrementation within the print function?

    a_random_number += 1;
    mut_is_short_for_mutable += 1;

    println!("{}", a_random_number);
    println!("{}", mut_is_short_for_mutable);
    // ok, only one error this time. What's the error say this time...
    //
    // A-HA! error: re-assignment of immutable variable `a_random_number`
    // a_random_number can't be changed!
}
