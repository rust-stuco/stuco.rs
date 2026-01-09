import argparse
import multiprocessing
import os
import signal

# The configuration file.
MARP_CONFIG = "marp_config.json"

# The topics for each week with their directory names.
# Format: week_number -> (directory_name, topic_name)
TOPICS = {
    1: ("01_introduction", "introduction"),
    2: ("02_ownership_p1", "ownership_p1"),
    3: ("03_structs_enums", "structs_enums"),
    4: ("04_collections_generics", "collections_generics"),
    5: ("05_errors_traits", "errors_traits"),
    6: ("06_modules_testing", "modules_testing"),
    7: ("07_ecosystem", "ecosystem"),
    8: ("08_closures_iterators", "closures_iterators"),
    9: ("09_lifetimes", "lifetimes"),
    10: ("10_smart_pointers", "smart_pointers"),
    11: ("11_unsafe", "unsafe"),
    12: ("12_parallelism", "parallelism"),
    13: ("13_concurrency", "concurrency"),
    # ??: ("temp_ownership_p2", "ownership_p2"),
}

# Global event to signal workers to stop.
stop_event = multiprocessing.Event()


# Handle SIGINT (Ctrl+C) by setting stop event and terminating the pool.
def signal_handler(sig, frame, pool):
    print("Received Ctrl+C, stopping all rendering...")
    stop_event.set()
    pool.close()
    pool.join()
    print("All workers stopped.")
    exit(1)


# Check if the markdown file has been modified since the outputs were generated.
def needs_rendering(markdown_file, outputs, dir_name):
    if not all(os.path.exists(os.path.join(dir_name, out)) for out in outputs):
        return True

    md_mtime = os.path.getmtime(os.path.join(dir_name, markdown_file))

    return any(
        os.path.getmtime(os.path.join(dir_name, out)) < md_mtime for out in outputs
    )


# Helper function to render the output file while handling interrupts.
def render_output(md_file, out):
    if stop_event.is_set():
        print(f"Interrupted during {out}")
        return False
    if os.system(f"marp {md_file} -c ../{MARP_CONFIG} -o {out}") != 0:
        raise Exception(f"Error rendering {out}")
    return True


# Given a week number, directory name, and topic name, renders the 4 slide decks.
def render(week, dir_name, topic, config, dry_run=False):
    md_file = f"{topic}.md"

    if stop_event.is_set():
        print(f"Stopping render for {topic} due to interrupt")
        return

    if not os.path.isdir(dir_name):
        print(f"Error: Directory {dir_name} does not exist")
        return

    if not os.path.isfile(os.path.join(dir_name, md_file)):
        print(f"Error: File {dir_name}/{md_file} does not exist")
        return

    if dry_run:
        print(f"[DRY RUN] Would render {dir_name}/{md_file}")
        return

    try:
        # We have to change into the correct directory for the `marp` command to work properly with
        # the images inside the slides.
        os.chdir(dir_name)

        # The default theme in the repository is dark.
        if not render_output(md_file, f"{topic}-dark.html"):
            return
        if not render_output(md_file, f"{topic}-dark.pdf"):
            return

        # Create a temporary file for the light theme.
        temp_light = f"{topic}-light-temp.md"
        with open(md_file, "r") as src_file:
            content = src_file.read()

            # Replace dark theme with light theme.
            content = content.replace("class: invert", "# class: invert")

            # Write it to the temporary file.
            with open(temp_light, "w") as dest_file:
                dest_file.write(content)

        # Render the light theme slides.
        if not render_output(temp_light, f"{topic}-light.html"):
            return
        if not render_output(temp_light, f"{topic}-light.pdf"):
            return

        # Delete the temporary file after rendering.
        os.remove(temp_light)

        print(f"✓ Successfully rendered {topic}")

    except Exception as e:
        print(f"✗ Error rendering {topic}: {e}")
    finally:
        # Always change back to the root directory.
        os.chdir("..")


def get_render_args(args, parser: argparse.ArgumentParser):
    if args.topics:
        # Extract valid topic names from TOPICS dict
        valid_topics = {topic for dir_name, topic in TOPICS.values()}
        selected_topics = set(args.topics)
        invalid_topics = selected_topics - valid_topics

        if invalid_topics:
            parser.error(f"Invalid topics specified: {invalid_topics}")

        print(f"Rendering slides {selected_topics}")
        topics = {
            week: (dir_name, topic)
            for week, (dir_name, topic) in TOPICS.items()
            if topic in selected_topics
        }
    elif args.all or args.force:
        print("Rendering all slides...")
        topics = TOPICS
    else:
        # Filter topics based on modification status.
        topics = dict()
        for week, (dir_name, topic) in TOPICS.items():
            md_file = f"{topic}.md"
            outputs = [
                f"{topic}-dark.html",
                f"{topic}-dark.pdf",
                f"{topic}-light.html",
                f"{topic}-light.pdf",
            ]
            if needs_rendering(md_file, outputs, dir_name):
                topics[week] = (dir_name, topic)
            else:
                print(f"Skipping {topic} in {dir_name}: outputs are up-to-date")

    return [
        (week, dir_name, topic, args.config, args.dry_run)
        for week, (dir_name, topic) in topics.items()
    ]


# Validate that required tools and files are available.
def validate_environment(config_file):
    # Check if marp command exists
    if os.system("which marp > /dev/null 2>&1") != 0:
        print("Error: 'marp' command not found. Please install Marp CLI:")
        print("  npm install -g @marp-team/marp-cli")
        return False

    # Check if config file exists
    if not os.path.exists(config_file):
        print(f"Error: Config file '{config_file}' not found")
        return False

    return True


# Parse the command-line arguments.
def parse_args(parser: argparse.ArgumentParser):
    parser.add_argument("--config", default=MARP_CONFIG, help="Marp config file")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview what would be rendered without actually rendering",
    )
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--force", action="store_true", help="Force render all slides")
    group.add_argument("--all", action="store_true", help="Render all slides")
    group.add_argument(
        "--topics",
        nargs="+",
        help="Render specific topics by name (e.g., introduction ownership_p1)",
    )

    return parser.parse_args()


def main():
    parser = argparse.ArgumentParser(description="Render Marp slides for a Rust course")
    args = parse_args(parser)

    # Validate environment before starting
    if not validate_environment(args.config):
        exit(1)

    render_args = get_render_args(args, parser)

    if not render_args:
        print("No slides to render.")
        return

    total = len(render_args)
    print(f"\nRendering {total} topic{'s' if total > 1 else ''}...\n")

    with multiprocessing.Pool(min(len(TOPICS), multiprocessing.cpu_count())) as pool:
        results = pool.starmap_async(render, render_args)

        # Signal handler for Ctrl+C.
        signal.signal(
            signal.SIGINT, lambda sig, frame: signal_handler(sig, frame, pool)
        )

        try:
            results.wait()
            if not results.successful():
                print("\n✗ Some rendering tasks failed")
            else:
                print(f"\n✓ All {total} topic{'s' if total > 1 else ''} rendered successfully")
        except KeyboardInterrupt:
            print("\nKeyboardInterrupt")
        finally:
            pool.close()
            pool.join()


if __name__ == "__main__":
    main()
