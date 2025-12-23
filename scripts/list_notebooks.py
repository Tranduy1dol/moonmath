#!/usr/bin/env python3
"""List all notebooks with summary."""
import json
from pathlib import Path


def main():
    notebooks = sorted(Path("notebooks").glob("*.ipynb"))
    print(f"📚 Found {len(notebooks)} notebooks:\n")

    total_cells = 0
    for nb_path in notebooks:
        with open(nb_path) as f:
            data = json.load(f)

        cells = len(data.get("cells", []))
        code_cells = sum(
            1 for c in data.get("cells", []) if c.get("cell_type") == "code"
        )
        markdown_cells = cells - code_cells
        kernel = data.get("metadata", {}).get("kernelspec", {}).get("name", "unknown")

        total_cells += cells
        print(f"  📓 {nb_path.name}")
        print(f"     Cells: {cells} ({code_cells} code, {markdown_cells} markdown)")
        print(f"     Kernel: {kernel}")
        print()

    print(f"📊 Total: {len(notebooks)} notebooks, {total_cells} cells")


if __name__ == "__main__":
    main()
