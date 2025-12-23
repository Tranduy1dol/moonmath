#!/usr/bin/env python3
"""Validate Rust syntax in notebook code cells."""
import json
import subprocess
import sys
import tempfile
from pathlib import Path


def main():
    print("🦀 Validating Rust syntax in notebook code cells...")
    notebooks = list(Path("notebooks").glob("*.ipynb"))
    errors = []
    total_cells = 0
    valid_cells = 0

    for nb_path in notebooks:
        with open(nb_path, "r", encoding="utf-8") as f:
            nb = json.load(f)

        kernel = nb.get("metadata", {}).get("kernelspec", {}).get("language", "")
        if kernel.lower() != "rust":
            print(f"  ⏭️  Skipping {nb_path} (not a Rust notebook)")
            continue

        print(f"  📓 Checking {nb_path}...")

        for i, cell in enumerate(nb.get("cells", [])):
            if cell.get("cell_type") != "code":
                continue

            source = cell.get("source", [])
            if isinstance(source, list):
                code = "".join(source)
            else:
                code = source

            if not code.strip():
                continue

            if code.strip().startswith(":"):
                continue

            total_cells += 1

            wrapped_code = f"fn __check__() {{\n{code}\n}}"

            with tempfile.NamedTemporaryFile(
                mode="w", suffix=".rs", delete=False
            ) as f:
                f.write(wrapped_code)
                temp_path = f.name

            try:
                result = subprocess.run(
                    ["rustfmt", "--check", temp_path],
                    capture_output=True,
                    text=True,
                    timeout=10,
                )

                if result.returncode > 1 or "error: " in result.stderr:
                    with open(temp_path, "w") as f:
                        f.write(code)

                    result2 = subprocess.run(
                        ["rustfmt", "--check", temp_path],
                        capture_output=True,
                        text=True,
                        timeout=10,
                    )

                    if result2.returncode > 1 or "error: " in result2.stderr:
                        errors.append(
                            {
                                "notebook": str(nb_path),
                                "cell": i + 1,
                                "error": result2.stderr or result.stderr,
                                "code_preview": code[:100] + "..."
                                if len(code) > 100
                                else code,
                            }
                        )
                    else:
                        valid_cells += 1
                else:
                    valid_cells += 1

            except subprocess.TimeoutExpired:
                errors.append(
                    {
                        "notebook": str(nb_path),
                        "cell": i + 1,
                        "error": "Timeout during syntax check",
                        "code_preview": code[:100] + "..."
                        if len(code) > 100
                        else code,
                    }
                )
            except Exception as e:
                errors.append(
                    {
                        "notebook": str(nb_path),
                        "cell": i + 1,
                        "error": str(e),
                        "code_preview": code[:100] + "..."
                        if len(code) > 100
                        else code,
                    }
                )
            finally:
                Path(temp_path).unlink(missing_ok=True)

    print(f"\n📊 Summary: {valid_cells}/{total_cells} code cells validated")

    if errors:
        print(f"\n❌ Found {len(errors)} syntax error(s):")
        for err in errors:
            print(f"\n  📓 {err['notebook']} - Cell {err['cell']}")
            print(f"  📝 Code: {err['code_preview']}")
            print(f"  ⚠️  Error: {err['error'][:200]}")
        sys.exit(1)
    else:
        print("\n✅ All Rust code cells have valid syntax!")


if __name__ == "__main__":
    main()
