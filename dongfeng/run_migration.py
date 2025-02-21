#! /usr/bin/python

import os
import subprocess
import uuid
from typing import Any


import tomllib

#
# TODO: create detail description about this script
#

# Path to your diesel.toml file
toml_file_path = "diesel.toml"


def get_schema_keys(toml_file: str) -> list[str] | None:
    """Parse the toml file and extract all schema keys."""
    try:
        excluded_key = ["file", "custom_type_derives"]
        with open(toml_file, "rb") as f:  # 'rb' mode is required for tomllib
            config = tomllib.load(f)

        # Extracting schema keys under the 'print_schema' section
        # Get print schema section

        print_schema: dict[str, Any] = config["print_schema"]
        schema_keys = [key for key in print_schema.keys() if key not in excluded_key]

        return schema_keys

    except Exception as e:
        print(f"Error reading {toml_file}: {e}")
        return None


def run_diesel_print_schema(schema_key: str):
    """Run the Diesel CLI command for a specific schema key."""
    try:
        print(f"\nRunning diesel print-schema for schema: {schema_key}")

        subprocess.run(
            [
                "diesel",
                "migration",
                "generate",
                "--diff-schema",
                schema_key,
                "--schema-key",
                schema_key,
            ],
        )

    except subprocess.CalledProcessError as e:
        print(f"Error executing diesel command for {schema_key}: {e}")


def main():
    full_path = os.path.abspath(toml_file_path)
    print(f"Generate Schema for Diesel Toml: {full_path}")
    schema_keys = get_schema_keys(full_path)

    if schema_keys:
        # process will doing
        # - delete migrations folder
        # - run diesel setup, to re-creating needed compmonent (like db and migrations folder)
        # - its will generate migration script based on diff schema
        # - then reset the database

        # subprocess.run(["diesel", "database", "drop", "--locked-schema"])
        subprocess.run(["rm", "-rf", "migrations"])
        subprocess.run(["diesel", "setup"])

        for schema_key in schema_keys:
            run_diesel_print_schema(schema_key)

        # subprocess.run(["diesel", "database", "setup"])
        # subprocess.run(["diesel", "migration", "run"])
    else:
        print("No schema keys found in the diesel.toml file.")


if __name__ == "__main__":
    main()
