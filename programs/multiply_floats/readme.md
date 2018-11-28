# Add Two Integers

## Description

A program that adds two integers provided by the user and prints the result.

## Output

```
Enter an integer:
40
Enter an integer:
99
The sum of 40 and 99 is 139
```

## Lesson

The program was relatively simple and thus I tried to use it to begin learning code modularization in Rust as recommended in the docs.
Lessons:

-   Error propagation
-   Separation of concerns
-   Error handling at a single location
-   existence of `std::process`

Truth be told: The code in this program does not differ much from that of the _add_two_integers_ program. This is basically a copy-pasted version of that program. The only alterations made were:

-   Using `f32` instead of `i32`
-   Changing the mathematical operation from `+` to `-` and
-   Renaming function names
