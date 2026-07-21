/// Inspect the env var `PRIVATE` to determine if we should run against personalized input.
pub fn should_run_with_personalised_input() -> bool {
    let should_run_personal = std::env::var("PRIVATE").unwrap_or(String::from("0"));
    let should_run_personal = should_run_personal
        .parse::<u8>()
        .expect("PRIVATE is not a number.");

    bool::try_from(should_run_personal).expect("PRIVATE conversion is cooked.")
}
