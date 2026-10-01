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


class PokerLabTest(unittest.TestCase):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # Need to cd into crate root here, for some reason it initializes before the `os.chdir` in
        # `run_tests.py`
        os.chdir("/autograder/source/pokerlab")

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

    # The following test setup code was generated with AI (Claude 3.5 Sonnet in Zed).

    @cargo_test(1.0, 5)
    def test_high_card_vs_high_card(self):
        """Testing high card vs high card comparison"""
        return "cargo test test_high_card_vs_high_card -- --exact"

    @cargo_test(2.0, 5)  
    def test_high_card_tie(self):
        """Testing high card tie comparison"""
        return "cargo test test_high_card_tie -- --exact"

    @cargo_test(3.0, 5)
    def test_one_pair_vs_high_card(self):
        """Testing one pair vs high card comparison"""
        return "cargo test test_one_pair_vs_high_card -- --exact"

    @cargo_test(4.0, 5)
    def test_one_pair_vs_one_pair(self):
        """Testing one pair vs one pair comparison"""
        return "cargo test test_one_pair_vs_one_pair -- --exact" 

    @cargo_test(5.0, 5)
    def test_two_pair_vs_one_pair(self):
        """Testing two pair vs one pair comparison"""
        return "cargo test test_two_pair_vs_one_pair -- --exact"

    @cargo_test(6.0, 5)
    def test_two_pair_vs_two_pair(self):
        """Testing two pair vs two pair comparison"""
        return "cargo test test_two_pair_vs_two_pair -- --exact"

    @cargo_test(7.0, 5)
    def test_two_pair_tie(self):
        """Testing two pair tie comparison"""
        return "cargo test test_two_pair_tie -- --exact"

    @cargo_test(8.0, 5)
    def test_three_of_a_kind_vs_three_of_a_kind(self):
        """Testing three of a kind vs three of a kind comparison"""
        return "cargo test test_three_of_a_kind_vs_three_of_a_kind -- --exact"

    @cargo_test(9.0, 5)
    def test_straight_vs_two_pair(self):
        """Testing straight vs two pair comparison"""
        return "cargo test test_straight_vs_two_pair -- --exact"

    @cargo_test(10.0, 5)
    def test_wheel_straight_vs_three_of_a_kind(self):
        """Testing wheel straight vs three of a kind comparison"""
        return "cargo test test_wheel_straight_vs_three_of_a_kind -- --exact"

    @cargo_test(11.0, 5)
    def test_straight_vs_straight(self):
        """Testing straight vs straight comparison"""
        return "cargo test test_straight_vs_straight -- --exact"

    @cargo_test(12.0, 5)
    def test_straight_vs_straight_tie(self):
        """Testing straight vs straight tie comparison"""
        return "cargo test test_straight_vs_straight_tie -- --exact"

    @cargo_test(13.0, 5)
    def test_flush_vs_one_pair(self):
        """Testing flush vs one pair comparison"""
        return "cargo test test_flush_vs_one_pair -- --exact"

    @cargo_test(14.0, 5)
    def test_flush_vs_two_pair(self):
        """Testing flush vs two pair comparison"""
        return "cargo test test_flush_vs_two_pair -- --exact"

    @cargo_test(15.0, 5)
    def test_flush_vs_three_of_a_kind(self):
        """Testing flush vs three of a kind comparison"""
        return "cargo test test_flush_vs_three_of_a_kind -- --exact"

    @cargo_test(16.0, 5)
    def test_flush_vs_straight(self):
        """Testing flush vs straight comparison"""
        return "cargo test test_flush_vs_straight -- --exact"

    @cargo_test(17.0, 5)
    def test_flush_vs_flush_different_ranks(self):
        """Testing flush vs flush different ranks comparison"""
        return "cargo test test_flush_vs_flush_different_ranks -- --exact"

    @cargo_test(18.0, 5)
    def test_flush_vs_flush_tie(self):
        """Testing flush vs flush tie comparison"""
        return "cargo test test_flush_vs_flush_tie -- --exact"

    @cargo_test(19.0, 5)
    def test_full_house_vs_three_of_a_kind(self):
        """Testing full house vs three of a kind comparison"""
        return "cargo test test_full_house_vs_three_of_a_kind -- --exact"

    @cargo_test(20.0, 5)
    def test_full_house_vs_flush(self):
        """Testing full house vs flush comparison"""
        return "cargo test test_full_house_vs_flush -- --exact"

    @cargo_test(21.0, 5)
    def test_full_house_vs_full_house(self):
        """Testing full house vs full house comparison"""
        return "cargo test test_full_house_vs_full_house -- --exact"

    @cargo_test(22.0, 5)
    def test_four_of_a_kind_vs_straight(self):
        """Testing four of a kind vs straight comparison"""
        return "cargo test test_four_of_a_kind_vs_straight -- --exact"

    @cargo_test(23.0, 5)
    def test_four_of_a_kind_vs_full_house(self):
        """Testing four of a kind vs full house comparison"""
        return "cargo test test_four_of_a_kind_vs_full_house -- --exact"

    @cargo_test(24.0, 5)
    def test_four_of_a_kind_vs_flush(self):
        """Testing four of a kind vs flush comparison"""
        return "cargo test test_four_of_a_kind_vs_flush -- --exact"

    @cargo_test(25.0, 5)
    def test_straight_flush_vs_high_card(self):
        """Testing straight flush vs high card comparison"""
        return "cargo test test_straight_flush_vs_high_card -- --exact"

    @cargo_test(26.0, 5)
    def test_straight_flush_vs_straight(self):
        """Testing straight flush vs straight comparison"""
        return "cargo test test_straight_flush_vs_straight -- --exact"

    @cargo_test(27.0, 5)
    def test_straight_flush_vs_flush(self):
        """Testing straight flush vs flush comparison"""
        return "cargo test test_straight_flush_vs_flush -- --exact"

    @cargo_test(28.0, 5)
    def test_straight_flush_vs_full_house(self):
        """Testing straight flush vs full house comparison"""
        return "cargo test test_straight_flush_vs_full_house -- --exact"

    @cargo_test(29.0, 5)
    def test_straight_flush_vs_four_of_a_kind(self):
        """Testing straight flush vs four of a kind comparison"""
        return "cargo test test_straight_flush_vs_four_of_a_kind -- --exact"

    @cargo_test(30.0, 5)
    def test_straight_flush_vs_straight_flush(self):
        """Testing straight flush vs straight flush comparison"""
        return "cargo test test_straight_flush_vs_straight_flush -- --exact"
