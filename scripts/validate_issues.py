#!/usr/bin/env python3
"""Check for common notebook issues."""
import json
from pathlib import Path


def main():
    print("🔍 Checking for common notebook issues...")
    notebooks = list(Path("notebooks").glob("*.ipynb"))
    warnings = []

    for nb_path in notebooks:
        with open(nb_path, "r", encoding="utf-8") as f:
            nb = json.load(f)

        cells_with_output = 0
        empty_cells = 0

        for cell in nb.get("cells", []):
            if cell.get("cell_type") == "code":
                if cell.get("execution_count") is not None:
                    cells_with_output += 1
                source = "".join(cell.get("source", []))
                if not source.strip():
                    empty_cells += 1

        kernel = nb.get("metadata", {}).get("kernelspec", {})
        if not kernel:
            warnings.append(f"{nb_path}: No kernel specification found")

        if empty_cells > 0:
            warnings.append(f"{nb_path}: {empty_cells} empty code cell(s)")

        print(
            f"  {nb_path}: {cells_with_output} executed cells, kernel: {kernel.get('name', 'unknown')}"
        )

    if warnings:
        print("\n⚠️  Warnings:")
        for warning in warnings:
            print(f"  - {warning}")

    print("\n✅ Issues check completed!")


if __name__ == "__main__":
    main()
