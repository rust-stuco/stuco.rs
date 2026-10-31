# Based on Gradescope's base image.
FROM gradescope/autograder-base

# Add our source files to our image.
ADD source /autograder/source

# Place run_autograder in parent directory so Gradescope can execute it.
RUN cp /autograder/source/run_autograder /autograder/run_autograder

# Ensure that scripts are executable.
RUN chmod +x /autograder/run_autograder

# Install Gradescope utilities for Python.
RUN pip3 install gradescope-utils

# Set up Rust.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to `PATH`.
ENV PATH="/root/.cargo/bin:${PATH}"
