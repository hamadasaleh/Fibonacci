use std::io;
use thousands::Separable;
fn main() {
    // This is a rust macro (!), which behaves slightly differently from a normal function
    println!("Compute nth element of Fibonacci sequence!");

    println!("Please input an integer value?");

    // We create a variable n to store the user input
    // We make the variable mutable, that is its value can change after it has been defined
    // which strays from default behaviour in Rust
    // We bind n to a new instance of the integer 32 type provided by the standard library
    // the ::new is an associated function of the String type (a 'method' in Python)
    let mut n = String::new();

    // '&' indicates that the argument n is a reference
    // What is the purpose of references?
    // They allow multiple parts of your code to access
    // the same variable without having to duplicate it in memory
    io::stdin().read_line(&mut n).expect("Failed to read line");

    // We shadow the previous value of n with a new one
    let n: u64 = n.trim().parse().expect("Please type a number!");

    let u = smart_fibonacci(n);
    // Separate thousands with ','
    println!("{}", u.separate_with_commas());
}

/// This algorithm scales linearly with n
/// The limit for representing u64 is reached for n=94
fn smart_fibonacci(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        // Initialise array for storing fibonacci cumsum
        let n_size: usize = n as usize;
        let mut u = vec![0; n_size + 1];
        u[1] = 1;
        // Compute the sum of the ith and (i+1)th terms
        // And store it into the (i+2)th slot in vector
        for i in 2..=n {
            // = -> inclusive range
            let i_size = i as usize;
            u[i_size] = u[i_size - 1] + u[i_size - 2];
        }
        {
            u[n_size]
        }
    }
}

/// This algorithm scales exponentially with n
#[allow(dead_code)] // Removes unused function warning. Use sparingly, as is good practice to remove genuinely unnecesary functions to keep clean codebase.
fn naive_fibonacci(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        naive_fibonacci(n - 1) + naive_fibonacci(n - 2)
    }
}
