# Find the ASCII Value of a Character

## Description

This program accepts a character from the user and prints its ASCII value

## Output

```

```

## Lessons

-   Rust protects you from using uninitialized variables.

Tried to do:

```rust
let mut word = "";
loop {
    ...
    word = input.trim();
    ...
}
let character = word.chars().next().unwrap();
```

and got:

```
error[E0381]: use of possibly uninitialized variable: `*word`
  --> src/main.rs:21:21
   |
21 |     let character = word.chars().next().unwrap();
   |                     ^^^^ use of possibly uninitialized `*word`
```

I could not understand why I was getting the error at first but after thinking about it I did. It might be possible to exit the loop without initializing word and thus it makes sense to require that word be initialized. My program, in this instance, was correct and word was getting initialized. Had that not been the case, without Rust's protection, I might have been effd. Thanks, Rust.

-   The borrow checker is harder to figure out than I first thought.

I now feel that all the complains with regard to the borrow checker are justified. Tried to do:

```rust
let mut input = String::new();
let mut word = "";
loop {
    ...
    stdin().read_line(&mut input)?;
    word = input.trim();
    ...
}
```

and got:

```
error[E0502]: cannot borrow `input` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:29
   |
18 |      stdin().read_line(&mut input)?;
   |                             ^^^^^ mutable borrow occurs here
19 |      word = input.trim();
   |             ----- immutable borrow occurs here
...
24 | }
   | - immutable borrow ends here
```

smh!

Revisiting the docs (The Rules of References):

-   At any given time, you can have either one mutable reference or any number of immutable references.
-   References must always be valid.

Fixed it with:

```rust
let mut input = String::new();
let mut word;
loop {
    ...
    stdin().read_line(&mut input)?;
    word = input.trim().to_string();
    ...
}
```

Some part of me knows why this would fix it. I seem to be out of touch with that part as I have no explanation. :-[

-   Looping and updating variables

I cannot decipher what is wrong with this:

```rust
fn get_character() -> Result<char, Box<Error>> {
    let mut input = String::new();
    let mut character_count;
    let mut word;
    loop {
        println!("Enter a character: ");
        stdin().read_line(&mut input)?;
        word = input.trim().to_string();
        character_count = word.len();
        if character_count == 0 {
            println!("You did not enter a character!");
            word.clear();
        } else if character_count > 1 {
            println!("You entered multiple characters!");
            word.clear();
        } else {
            break;
        }
    }
    let character = word.chars().next().unwrap();
    Ok(character)
}

fn character_to_bytes(character: char) -> Vec<u8> {
    ...
}

fn run() -> Result<(), Box<Error>> {
    ...
}

fn main() {
    ...
}
```

Running with; empty character > single character

```
Enter a character:

You did not enter a character!
Enter a character:
a
ASCII value of a = [97]
```

Running with; single character

```
Enter a character:
g
ASCII value of g = [103]
```

Running with; multiple characters > empty character > single character

```
Enter a character:
sa
You entered multiple characters!
Enter a character:

You entered multiple characters!
Enter a character:
d
You entered multiple characters!
Enter a character:
```

:-O
