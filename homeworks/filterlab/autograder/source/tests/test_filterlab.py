import os
import subprocess
import unittest
from functools import wraps

from gradescope_utils.autograder_utils.decorators import number, partial_credit, weight


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


class FilterLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here, for some reason it initializes before the `os.chdir` in
        # `run_tests.py`
        os.chdir("/autograder/source/filterlab")

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

    @cargo_test(1.0, 20)
    def test_simple(self):
        """Testing simple test"""
        return "cargo test -- --exact simple_test"

    @cargo_test(2.0, 20)
    def test_medium(self):
        """Testing medium test"""
        return "cargo test -- --exact medium_test"

    @cargo_test(3.0, 20)
    def test_random_medium(self):
        """Testing random medium test"""
        return "cargo test -- --exact random_medium_test"

    @cargo_test(4.0, 20)
    def test_random_large(self):
        """Testing random large test"""
        return "cargo test --release -- --exact random_large_test"

    @cargo_test(5.0, 0)
    def test_bench_read(self):
        """Benchmarking writes (manually graded)"""
        return "cargo bench --bench write --quiet"

    @cargo_test(6.0, 0)
    def test_bench_write(self):
        """Benchmarking reads (manually graded)"""
        return "cargo bench --bench read --quiet"
