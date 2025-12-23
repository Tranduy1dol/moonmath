#!/usr/bin/env python3
"""Validate notebook JSON structure."""
import sys
import json
import nbformat
from pathlib import Path


def main():
    print("🔍 Validating notebook JSON structure...")
    notebooks = list(Path("notebooks").glob("*.ipynb"))
    errors = []

    for nb_path in notebooks:
        print(f"  Checking {nb_path}...")
        try:
            with open(nb_path, "r", encoding="utf-8") as f:
                content = json.load(f)

            nb = nbformat.read(nb_path, as_version=4)

            if "cells" not in content:
                errors.append(f"{nb_path}: Missing 'cells' field")
            if "metadata" not in content:
                errors.append(f"{nb_path}: Missing 'metadata' field")

            print(f"    ✅ {len(nb.cells)} cells found")

        except json.JSONDecodeError as e:
            errors.append(f"{nb_path}: Invalid JSON - {e}")
        except Exception as e:
            errors.append(f"{nb_path}: {e}")

    if errors:
        print("\n❌ Validation failed:")
        for error in errors:
            print(f"  - {error}")
        sys.exit(1)
    else:
        print(f"\n✅ All {len(notebooks)} notebooks have valid JSON structure!")


if __name__ == "__main__":
    main()
