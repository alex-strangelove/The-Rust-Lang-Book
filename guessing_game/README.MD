# Guessing game

#### Imports

```rust
use std::io // Standard I/O library (user input)
use colored::Colorize; // Colorize text (println!("{}", "hello".green()))
use rand::Rng; // Library for random number generation
use std::cmp::Ordering; // Enum to match .cmp() expressions
```

#### Use `println!` macro to prompt the user:

```rust
println!("Guess the number!");
println!("Please input your guess.");
```

#### Create mutable variable to store user input:

```rust
let mut guess = String::new();
```

#### Read standard input:

```rust
 io::stdin()
    .read_line(&mut guess) // Return io::Result<usize>
    .expect("Failed to read line"); // So, use .expect() or unwrap() to handle Result type
```

#### Parse user input string into unsigned 32bit integer value:
```rust
let guess = match guess.trim().parse::<u32>() {
    Ok(num) => num,
    Err(_) => continue,
};
```

#### Using `Ordering` enum types, match the expression:
```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("{}", "To less".red()),
    Ordering::Equal => {
        println!("{}", "Success!".green());
        break;
}
    Ordering::Greater => println!("{}", "Too big".red()),
}
```