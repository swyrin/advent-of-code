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

Constraints:

- Struct fields are parsed like the order shown in struct.
- Struct with multiple fields must have their fields be either `#[parse(section(...))]` or `#[parse(sections(...))]`
    - There exists `#[trust_me]` as the "I know what I am doing."

(powered by [`aoc_parse`](https://lib.rs/crates/aoc-parse), so give them a praise)

(yes, the APIs are inspired by the glorious [`serde`](https://lib.rs/crates/serde))

> The implementation of this one is VERY ATROCIOUS since AoC input varies a lot, so everything in this macro is just
> whack-a-mole game.
>
> If the thing doesn't work, perform [`impl std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html),
> slap a `#[derive(Debug, Clone)]` since that is what `#[derive(AocInput)]` derive do for you - plus the inlined
> parsers.

### Program definition: `aoc!` proc macro

You may want to use `aoc!` macro to reduce the boilerplate of having to define `fn main()`
with input reading and writing personalized results every time.

This one accepts two functions only with the name of `part_one` and `part_two`, and will
throw if there is none. And I enforce the unpacking of struct field (s). I love Rust.

(Shit design, I know)

> Due to my laziness to install [`num_traits`](https://lib.rs/crates/num-traits) crate, `part_{one,two}` must
> return `impl std::fmt::Display`

```rust
aoc! {
    fn part_one(Input { .. }: &Input) -> impl std::fmt::Display {}    
}
```

### Test definition: `#[sample]` attribute macro

- Yes, I don't want to use `#[parametrize]` just for ONE test.
- Contrary of other atrocious candidates, this one is... very simple.

```rust
#[sample(input = "a", expected = "b")]
fn part_one() -> impl std::fmt::Display {}
```

- To the least of surprises:
    - `input` is `&str`, as `AocInput` employs [`impl FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
      for this.
    - `expected` is `impl std::fmt::Display` due to the return of `part` functions.