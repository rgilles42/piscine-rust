fn main() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::{Command, Stdio};
    const MANIFEST_PATH: &str = "../../solutions/looping/Cargo.toml";
    const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n";
    const ANSWER: &str = "The letter e\n";

    #[test]
    fn test_correct_answer_on_first_try() {
        let mut looping = Command::new("cargo")
            .args(&["run", "--manifest-path", MANIFEST_PATH, "-q"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        {
            let stdin = looping.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(ANSWER.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = looping.wait_with_output().expect("Failed to read stdout");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            RIDDLE.to_string() + "Number of trials: 1\n"
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

        // the expected output will be collected by line and only checked
        // at the end
        // So every response of the program is saved in a variable and
        // then compared with all the lines produced in order
        let mut expected_output = RIDDLE.to_string();
        let n_fails = 4;

        // Send the wrong answer to the program
        for _ in 0..n_fails {
            let stdin = looping.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all("no\n".as_bytes())
                .expect("Failed to write to stdin");
            // Add a new line of the output of each line
            expected_output.push_str(RIDDLE);
        }

        // Send the correct answer
        {
            let stdin = looping.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(ANSWER.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = looping.wait_with_output().expect("Failed to read stdout");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected_output + &format!("Number of trials: {}\n", n_fails + 1)
        );
    }
}
