import os, subprocess
import unittest
from functools import wraps
from gradescope_utils.autograder_utils.decorators import weight, number, partial_credit


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
    return "error" not in output and "test result: failed." not in output


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


class IterLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here, for some reason it initializes before the `os.chdir` in
        # `run_tests.py`
        os.chdir("/autograder/source/iterlab")

        self.clippy_output = run_cmd("cargo clippy && cargo fmt --all -- --check")
        self.passed_clippy = verify_output_warnings(self.clippy_output)

    @number(0.0)
    @weight(0)
    def test_clippy_check(self):
        """Testing cargo clippy"""
        print(self.clippy_output)
        if not self.passed_clippy:
            self.fail(
                "Detected warnings and/or errors in cargo clippy and cargo fmt output!\n"
                "Please fix the lints above to receive credit for this assignment\n"
                "Hint: run cargo fmt if you see a 'diff' warning!\n"
            )

    @cargo_test(1.0, 10)
    def test_fibonacci(self):
        """Testing Fibonacci Iterator"""
        return "cargo test fibonacci_tests"

    @cargo_test(2.0, 35)
    def test_cycle(self):
        """Testing Cycle Iterator"""
        return "cargo test cycle_tests"

    @cargo_test(3.0, 40)
    def test_interleave(self):
        """Testing Interleave Iterator"""
        return "cargo test interleave_tests"

    @cargo_test(4.0, 15)
    def test_double(self):
        """Testing Double Iterator"""
        return "cargo test double_tests"

    @cargo_test(5.0, 15)
    def test_sum_squares(self):
        """Testing sum_squares"""
        return "cargo test sum_squares_tests"

    @cargo_test(6.0, 35)
    def test_fib_fun(self):
        """Testing fib_fun"""
        return "cargo test fib_fun_tests"
