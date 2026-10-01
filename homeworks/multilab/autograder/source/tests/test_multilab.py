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


class MultiLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here, for some reason it initializes before the `os.chdir` in
        # `run_tests.py`
        os.chdir("/autograder/source/multilab")

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

    @cargo_test(1.0, 7)
    def test_multiset_docs(self):
        """Testing multiset doc tests"""
        return "cargo test multiset --doc"

    @cargo_test(1.1, 8)
    def test_multiset_basic(self):
        """Testing multiset basic"""
        return "cargo test multiset_tests::test_insert_remove_basic"

    @cargo_test(1.2, 10)
    def test_multiset_more(self):
        """Testing multiset more"""
        return "cargo test multiset_tests::test_insert_remove_more"

    @cargo_test(1.3, 10)
    def test_multiset_multiple(self):
        """Testing multiset multiple"""
        return "cargo test multiset_tests::test_multiple_insertions_and_removals"

    @cargo_test(1.4, 15)
    def test_multiset_interleaved(self):
        """Testing multiset interleaved"""
        return "cargo test multiset_tests::test_interleaved_insert_remove"

    @cargo_test(2.0, 6)
    def test_multimap_docs(self):
        """Testing multimap doc tests"""
        return "cargo test multimap --doc"

    @cargo_test(2.1, 6)
    def test_multimap_duplicate(self):
        """Testing multimap duplicates"""
        return "cargo test multimap_tests::test_insert_duplicate_values"

    @cargo_test(2.2, 6)
    def test_multimap_many(self):
        """Testing multimap many"""
        return "cargo test multimap_tests::test_insert_many_values"

    @cargo_test(2.3, 6)
    def test_multimap_not_exist(self):
        """Testing multimap not exist"""
        return "cargo test multimap_tests::test_remove_nonexistent_key"

    @cargo_test(2.4, 6)
    def test_multimap_remove(self):
        """Testing multimap remove"""
        return "cargo test multimap_tests::test_remove_value_from_multiple"

    @cargo_test(2.5, 20)
    def test_multimap_integrated(self):
        """Testing multimap integrated"""
        return "cargo test multimap_tests::test_large_insert_remove"
