#!/usr/bin/env python3
"""Verify notebook metadata consistency."""
import json
from pathlib import Path


def main():
    print("🔍 Checking metadata consistency...")
    notebooks = list(Path("notebooks").glob("*.ipynb"))
    kernel_info = {}

    for nb_path in notebooks:
        with open(nb_path, "r", encoding="utf-8") as f:
            nb = json.load(f)

        kernel = nb.get("metadata", {}).get("kernelspec", {}).get("name", "unknown")
        language = nb.get("metadata", {}).get("kernelspec", {}).get("language", "unknown")

        key = f"{kernel} ({language})"
        if key not in kernel_info:
            kernel_info[key] = []
        kernel_info[key].append(str(nb_path))

    print("📊 Kernel usage summary:")
    for kernel, files in kernel_info.items():
        print(f"  {kernel}: {len(files)} notebook(s)")

    if len(kernel_info) > 1:
        print("\n⚠️  Multiple kernels detected - ensure this is intentional")

    print("\n✅ Metadata check completed!")


if __name__ == "__main__":
    main()
