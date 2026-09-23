# My Advent of Code archives

This repository is my Advent of Code solutions, hopefully in Rust.

## What's inside?

- `src/bin/year_XXXX_day_YY.rs` - Submissions.
- `examples/template.rs` - Scaffold for new submissions.

## Which years are included?

- [ ] 2021 (in-progress)
- [x] 2024 (quited after D14)
- [x] 2025

Below are boring things, you have been warned.

-----------------------

# The manual to use Swyrin-branded macros

## Input definition: `AocInput` derive macro

First of all: use `#[derive(AocInput)]` to provide your own input definition (s):

```rust
#[derive(AocInput)]
struct Input {
    #[parse(line(u32+))]
    pub(crate) numbers: Vec<u32>
}
```

(powered by [`aoc_parse`](https://lib.rs/crates/aoc-parse), so give them a praise)

(yes, the APIs are inspired by the glorious [`serde`](https://lib.rs/crates/serde))

### Constraints

- Struct fields are parsed like the order shown in struct.
- Struct with multiple fields must have their fields be either `#[parse(section(...))]` or `#[parse(sections(...))]`

> [!WARNING]
> The implementation of this one is VERY ATROCIOUS since AoC input varies a lot, so everything in this macro is just
> whack-a-mole game.
>
> If the thing doesn't work, perform [`impl std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html),
> slap a `#[derive(Debug, Clone)]` since that is what `#[derive(AocInput)]` derive do for you - plus the inlined
> parsers.

### Program definition: `aoc!` proc macro

You may want to use `aoc!` macro to reduce the boilerplate of having to define `fn main()`
with input reading and writing personalized results every time.

This one accepts two functions only with the name of `part_one` and `part_two`.

To provide a sample input, simply put `#[sample(input, expected)]` on top of the part function.

> [!TIP]
> The functions `part_{one,two}` must:
>  - Accept a struct with [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) as supertrait.
>    - Same for `input` in `#[sample]`
>
>  - Return data with [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) as supertrait.
>    - Same for `expected` in `#[sample]`
>
> ```rust
> aoc! {
>    #[sample(input = "a", expected = "b")]
>    fn part_one(Input { .. }: &Input) -> impl std::fmt::Display {}    
> }
> ``` 
