/// Get advent of code session token.
#[cfg(not(test))]
pub fn get_session_token() -> String {
    std::env::var("AOC_SESSION_TOKEN").expect("Please set AOC_SESSION_TOKEN")
}
