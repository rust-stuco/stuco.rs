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

            if self.passed_clippy:
                RowLabTest.passed_tests.add(func.__name__)

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
    # The graded tests that must pass for the benchmark to earn points and reach the leaderboard.
    # `unittest` runs tests in alphabetical order, so the test names are numbered to make these run
    # before the benchmark.
    REQUIRED_TESTS = {"test_1_mean", "test_2_minmax", "test_3_all"}

    passed_tests = set()
    bench_time = None

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
    def test_0_build(self):
        """Build everything up front"""
        return "cargo test --no-run && cargo bench --no-run"

    @cargo_test(1.0, 25)
    def test_1_mean(self):
        """Testing mean"""
        return "cargo test -- --exact test_mean"

    @cargo_test(1.1, 25)
    def test_2_minmax(self):
        """Testing min and max"""
        return "cargo test -- --exact test_minmax"

    @cargo_test(1.2, 25)
    def test_3_all(self):
        """Testing everything"""
        return "cargo test -- --exact test_all"

    @number(2.0)
    @weight(25)
    def test_4_bench_basic(self):
        """Benchmarking (basic)"""

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

        print("Running `cargo bench`...\n")
        criterion_output = run_cmd('cargo bench; echo "exit code: $?"')
        print(criterion_output[-5000:])

        if "signal: 9" in criterion_output:
            self.fail(
                "Out of memory!\nThe benchmark used more than the "
                "6.0 GB of memory available to the autograder.\n"
                "Make sure you do not hold more data than you need at once."
            )

        exit_code = re.search(r"exit code: (\d+)\s*$", criterion_output)
        if exit_code and exit_code.group(1) != "0":
            self.fail(
                f"`cargo bench` failed with exit code {exit_code.group(1)}, see the output above"
            )

        # Find the `time` line with the 3 measurements in brackets.
        # This pattern looks for "time: [number s number s number s]" with flexible whitespace.
        # (Generated with the help of LLMs)
        time_pattern = (
            r"time:\s*\[\s*([\d\.]+)\s+s\s+([\d\.]+)\s+s\s+([\d\.]+)\s+s\s*\]"
        )

        re_match = re.search(time_pattern, criterion_output)
        if not re_match:
            self.fail("Could not find `time` data in the expected Criterion format")

        try:
            bench_time = float(re_match.group(2))
        except ValueError:
            self.fail("Failed to convert '{}' to a float".format(re_match.group(2)))

        # The benchmark still runs so students can see their time, but it only counts when the
        # solution is correct.
        failed_tests = sorted(RowLabTest.REQUIRED_TESTS - RowLabTest.passed_tests)
        if failed_tests:
            self.fail(
                f"These tests did not pass: {', '.join(failed_tests)}\n"
                "The benchmark gets 0 points and your runtime is not added to the leaderboard "
                "until every test passes."
            )

        RowLabTest.bench_time = bench_time

    @number(2.1)
    @partial_credit(0)
    def test_5_bench_bonus(self, set_score=None):
        """Benchmarking (bonus)"""

        if RowLabTest.bench_time is None:
            set_score(0)
            self.fail("No extra credit points awarded")

        bonus = max(round(-7 * RowLabTest.bench_time + 210), 0)
        set_score(bonus)
        if bonus > 0:
            print(f"Awarded {bonus} extra credit points for performance. Well done!")
        else:
            self.fail("No extra credit points awarded")

    @leaderboard("runtime", sort_order="asc")
    def test_6_bench_rowlab(self, set_leaderboard_value=None):
        """Leaderboard runtime"""

        # Even though this is not a test, we still have to prefix it with `test_` for the Gradsecope
        # test suite to work properly.

        if RowLabTest.bench_time is None:
            self.fail("Benchmark failed, see 'Benchmarking (basic)' above")

        set_leaderboard_value(RowLabTest.bench_time)
