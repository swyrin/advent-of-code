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

## The manual to use Swyrin-branded macros

> [!WARNING]
> The API isn't stable yet and will change often to fit my taste.

> [!NOTE]
> If reading is not something you can do, there exists `examples/template.rs`, or the "Minimum working code" at the
> bottom of this page

### Input definition: `AocInput` derive macro

Inspired by [`serde`](https://lib.rs/crates/serde) syntax & powered by [`aoc_parse`](https://lib.rs/crates/aoc-parse)
crate.

> [!WARNING]
> The implementation of this one is VERY ATROCIOUS since AoC input varies a lot, so everything in this macro is just
> whack-a-mole game.
>
> If it doesn't work, perform manual [`impl std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).

```rust
#[derive(AocInput)]
struct Input {
    #[parse(line(u32+))]
    pub(crate) numbers: Vec<u32>
}
```

### Program definition: `aoc!` & `part` proc macros

You may want to use the `aoc!` macro to not having to write `fn main()`
with `input.txt` reading and processing every time. It will collect the `#[part]` functions thanks to the existence of [
`inventory`](https://docs.rs/crate/inventory/latest)

Each `#[part]` function must:

- Not:
    - `extern "C"` because why would you do that?
    - `unsafe`
    - `async`
    - `const`, I am not stuttering: https://doc.rust-lang.org/reference/const_eval.html#const-functions
    - Having generic, like `part_one<T>(input: &T)`, had enough w/ lifetimes.
    - Having variadic, like `part_one(input: &Input)`
- Have ONE parameter, that is borrowed input type.
- The input type must [`impl std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
    - `#[derive(AocInput)]` will do that one for you.
- The return value of that function must [
  `impl std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)

Each `#[part]` will:

- Generate a `#[cfg(test)] mod tests`
- Collect its sample tests from `#[sample(...)]`s placed
  underneath & write directly to that module.

### Minimum working code

```rust
use macros::{AocInput, aoc, part, sample};

#[derive(AocInput)]
struct Input {
    #[parse(line(u32+))]
    pub(crate) numbers: Vec<u32>
}

aoc!();

#[part]
#[sample(input = "1", expected = "1")]
#[sample(input = "2", expected = "2")]
fn part_one(input: &Input) -> impl std::fmt::Display {
    input.numbers[0]
}

#[part]
fn part_two(input: &Input) -> impl std::fmt::Display {
    input.numbers.len()
}
```
