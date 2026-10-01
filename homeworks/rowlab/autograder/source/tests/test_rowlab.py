import os
import re
import subprocess
import unittest
from functools import wraps

from gradescope_utils.autograder_utils.decorators import number, partial_credit, weight, leaderboard


# Main decorator for Gradescope tests.
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

            # Run the command and show student the output.
            cmd = func(self)
            print(f"Running `{cmd}`...\n")
            output = run_cmd(cmd)
            print(output)

            # Check for any errors in output.
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


# Runs given shell command in a subprocess.
def run_cmd(cmd):
    test = subprocess.Popen(
        cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    # Capture the output of the subprocess command.
    output = test.stdout.read().strip().lower().decode()
    return output


class RowLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here, for some reason it initializes before the `os.chdir` in
        # `run_tests.py`
        os.chdir("/autograder/source/rowlab")

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

    @cargo_test(0.0, 0)
    def test_build(self):
        """Build everything up front"""
        return "cargo test --no-run"

    @cargo_test(1.0, 25)
    def test_mean(self):
        """Testing mean"""
        return "cargo test -- --exact test_mean"

    @cargo_test(2.0, 25)
    def test_minmax(self):
        """Testing min and max"""
        return "cargo test -- --exact test_minmax"

    @cargo_test(3.0, 50)
    def test_all(self):
        """Testing everything"""
        return "cargo test -- --exact test_all"

    @leaderboard("runtime", sort_order="asc")
    def test_bench_rowlab(self, set_leaderboard_value=None):
        """Benchmarking with Criterion"""

        # Even though this is not a test, we still have to prefix it with `test_` for the Gradsecope
        # test suite to work properly.

        if not self.passed_clippy:
            self.fail(
                "Detected warnings and/or errors in `cargo clippy` and `cargo fmt` output!\n"
                "Please fix the lints above to receive credit for this assignment\n"
                "Hint: run `cargo fmt` if you see a 'diff' warning, and `cargo clippy` otherwise!\n"
            )

        # Example Criterion Output:
        """
        Benchmarking brc: Warming up for 3.0000 s
        Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 207.6s.
        brc                     time:   [12.694 s 12.765 s 12.857 s]
                                change: [-2.7843% -1.0671% +0.7337%] (p = 0.28 > 0.05)
                                No change in performance detected.
        """

        criterion_output = run_cmd("cargo bench")

        # Find the `time` line with the 3 measurements in brackets.
        # This pattern looks for "time: [number s number s number s]" with flexible whitespace.
        # (Generated with the help of LLMs)
        time_pattern = (
            r"time:\s*\[\s*([\d\.]+)\s+s\s+([\d\.]+)\s+s\s+([\d\.]+)\s+s\s*\]"
        )

        re_match = re.search(time_pattern, criterion_output)
        if not re_match:
            self.fail("Could not find `time` data in the expected Criterion format")

        # We should have exactly 3 numbers in the `time` line output.
        num_groups = len(re_match.groups())
        if num_groups != 3:
            self.fail(f"Expected 3 time measurements, found {num_groups}")

        # Extract the middle number as the best estimate of execution time.
        try:
            average_time = float(re_match.group(2))
            set_leaderboard_value(average_time)
        except ValueError:
            self.fail("Failed to convert '{}' to a float".format(re_match.group(2)))
