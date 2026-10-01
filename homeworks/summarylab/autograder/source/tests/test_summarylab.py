import os, subprocess
import unittest
from functools import wraps
from gradescope_utils.autograder_utils.decorators import weight, number, partial_credit


# Main decorator for gradescope tests
def cargo_test(test_num, weight):
    def cargo_test_wrapper(func):
        @number(test_num)
        @partial_credit(weight)
        @wraps(func)
        def test(self, set_score=None):
            if not self.passed_clippy:
                set_score(0)
                print(
                    "Detected warnings and/or errors in cargo clippy output! "
                    "Setting score to 0 and moving on to tests:\n"
                )
            else:
                print("cargo clippy succeeded, moving on to tests:\n")

            # Run the command and show student the output
            cmd = func(self)
            print(f"Running `{cmd}`...\n")
            output = run_cmd(cmd)
            print(output)

            # Check for any errors in output
            if not verify_output_errors(output):
                self.fail(
                    "Error detected! Please review the above to see what went wrong."
                )

        return test

    return cargo_test_wrapper


def verify_output_errors(output):
    return "error" not in output and "FAILED" not in output


def verify_output_warnings(output):
    return (
        "diff" not in output
        and "warning" not in output
        and verify_output_errors(output)
    )


# Runs given shell command in a subprocess
def run_cmd(cmd):
    test = subprocess.Popen(
        cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    # Capture the output of the subprocess command
    output = test.stdout.read().strip().lower().decode()
    return output


class SummaryLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here,
        # for some reason it initializes before the os.chdir in run_tests.py
        os.chdir("/autograder/source/summarylab")

        self.clippy_output = run_cmd("cargo clippy && cargo fmt --all -- --check")
        self.passed_clippy = verify_output_warnings(self.clippy_output)

    @number(0.0)
    @weight(0)
    def test_clippy_check(self):
        """Testing cargo clippy"""
        print(self.clippy_output)
        if not self.passed_clippy:
            self.fail(
                "Detected warnings and/or errors in `cargo clippy` and `cargo fmt` output!\n"
                "Please fix the lints above to receive credit for this assignment\n"
                "Hint: run `cargo fmt` if you see a 'diff' warning, and `cargo clippy` otherwise!\n"
            )

    @cargo_test(1.0, 25)
    def test_email_basic(self):
        """Testing Emails Basic"""
        return "cargo test test_basic_and_empty_emails"

    @cargo_test(1.1, 25)
    def test_email_malformed(self):
        """Testing Emails Malformed"""
        return "cargo test test_malformed_emails"

    @cargo_test(1.2, 25)
    def test_email_summary(self):
        """Testing Emails Summary"""
        return "cargo test test_summaries"

    @cargo_test(1.3, 25)
    def test_email_long(self):
        """Testing Emails Long"""
        return "cargo test test_summarize_extremely_long_message"

    @cargo_test(2.0, 10)
    def test_tweet_basic(self):
        """Testing Tweets Basic"""
        return "cargo test test_basic_and_empty_tweets"

    @cargo_test(2.1, 10)
    def test_tweet_malformed(self):
        """Testing Tweets Malformed"""
        return "cargo test test_malformed_tweets"

    @cargo_test(2.2, 10)
    def test_tweet_re(self):
        """Testing Retweets and Replies"""
        return "cargo test test_retweets_and_replies"

    @cargo_test(2.3, 20)
    def test_tweet_long(self):
        """Testing Tweets Long"""
        return "cargo test test_edge_cases_and_long_content"
