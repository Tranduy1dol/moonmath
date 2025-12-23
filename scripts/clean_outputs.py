#!/usr/bin/env python3
"""Clean notebook outputs and execution counts."""
import json
from pathlib import Path


def main():
    notebooks = list(Path("notebooks").glob("*.ipynb"))

    for nb_path in notebooks:
        with open(nb_path, "r", encoding="utf-8") as f:
            nb = json.load(f)

        modified = False
        for cell in nb.get("cells", []):
            if cell.get("cell_type") == "code":
                if cell.get("outputs"):
                    cell["outputs"] = []
                    modified = True
                if cell.get("execution_count") is not None:
                    cell["execution_count"] = None
                    modified = True

        if modified:
            with open(nb_path, "w", encoding="utf-8") as f:
                json.dump(nb, f, indent=1, ensure_ascii=False)
            print(f"  🧹 Cleaned {nb_path}")
        else:
            print(f"  ✅ {nb_path} already clean")

    print("\n✅ All notebooks cleaned!")


if __name__ == "__main__":
    main()
