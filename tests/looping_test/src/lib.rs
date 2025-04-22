#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::{Command, Stdio};

    const MANIFEST_PATH: &str = "../../solutions/looping/Cargo.toml";
    const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n";
    const ANSWER: &str = "The letter e\n";

    // These tests will hang infinitely if the correct answer is never given (because of std::process::Child::wait_with_output)
    // I assume the tester will eventually time out and reject the solution in such a case
    // If not, such a test case must be manually implemented here

    #[test]
    fn test_correct_answer_on_first_try() {
        let mut looping = Command::new("cargo")
            .args(&["run", "--manifest-path", MANIFEST_PATH, "-q"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let mut stdin = looping.stdin.take().expect("Failed to open stdin");
        stdin
            .write_all(ANSWER.as_bytes())
            .expect("Failed to write to stdin");

        let output = looping.wait_with_output().expect("Failed to read stdout");
        assert_eq!(
            String::from_utf8(output.stdout).expect("Invalid output"),
            format!("{}Number of trials: 1\n", RIDDLE)
        );
    }

    #[test]
    fn test_more_than_one_trial_to_get_the_right_answer() {
        let mut looping = Command::new("cargo")
            .args(&["run", "--manifest-path", MANIFEST_PATH, "-q"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let n_incorrect_tries = 4;
        let expected_output = RIDDLE.repeat(n_incorrect_tries + 1);

        let mut stdin = looping.stdin.take().expect("Failed to open stdin");
        for _ in 0..n_incorrect_tries {
            stdin
                .write_all("incorrect answer\n".as_bytes())
                .expect("Failed to write to stdin");
        }
        stdin
            .write_all(ANSWER.as_bytes())
            .expect("Failed to write to stdin");

        let output = looping.wait_with_output().expect("Failed to read stdout");

        assert_eq!(
            String::from_utf8(output.stdout).expect("Invalid output"),
            format!(
                "{}Number of trials: {}\n",
                expected_output,
                n_incorrect_tries + 1
            )
        );
    }
}
