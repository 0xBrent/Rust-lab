// If i is divisible by 3, return the String "Fizz"
// If i is divisible by 5, return the String "Buzz"
// If i is divisible by both 3 and 5, return the String "FizzBuzz"
// If neither of them is true return the number as a String

use std::ops::RangeInclusive;

fn main() {
    let range = 1..=100;
    println!("{}", fizzbuzz(range))
}

fn fizzbuzz(i: RangeInclusive<i32>) -> String {
    i.into_iter()
        .map(|i| match i {
            i if i % 3 == 0 && i % 5 == 0 => "\nfizzbuzz".to_string(),
            i if i % 3 == 0 => "\nfizz".to_string(),
            i if i % 5 == 0 => "\nbuzz".to_string(),
            _ => format!("\n{}", i),
        })
        .collect::<String>()
}

fn fizzbuzz_tuple(i: RangeInclusive<i32>) -> String {
    i.into_iter()
        .map(|i| {
            let remainders = (i % 3, i % 5);

            match remainders {
                (0, 0) => "\nFizzBuzz".to_string(),
                (0, _) => "\nFizz".to_string(),
                (_, 0) => "\nBuzz".to_string(),
                (_, _) => format!("\n{}", i),
            }
        })
        .collect()
}
