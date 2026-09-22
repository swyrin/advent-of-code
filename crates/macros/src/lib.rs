pub use aoc_parse;
pub use macros_impl::{AocInput, aoc};

pub fn split_sections(s: &str) -> Vec<&str> {
    s.split("\n\n").collect()
}

pub fn parse_sections<P: aoc_parse::Parser>(
    parser: &P,
    sections: &[&str],
) -> Result<(usize, P::Output), aoc_parse::ParseError> {
    let mut take = sections.len();

    loop {
        if take == 0 {
            let first = sections.first().copied().unwrap_or("");
            return parser.parse(first).map(|value| (1, value));
        }

        match parser.parse(&sections[..take].join("\n\n")) {
            Ok(value) => return Ok((take, value)),
            Err(_) => take -= 1,
        }
    }
}
