# Justfile for moonmath notebook validation
# Run `just --list` to see all available commands

# Default recipe: run all validations
default: validate

# Run all notebook validations
validate: validate-json validate-issues validate-metadata validate-rust
    @echo "✅ All validations completed!"

# Validate notebook JSON structure
validate-json:
    @python3 scripts/validate_json.py

# Check for common notebook issues
validate-issues:
    @python3 scripts/validate_issues.py

# Verify notebook metadata consistency
validate-metadata:
    @python3 scripts/validate_metadata.py

# Validate Rust syntax in code cells
validate-rust:
    @python3 scripts/validate_rust.py

# List all notebooks with summary
list-notebooks:
    @python3 scripts/list_notebooks.py

# Install required Python dependencies for validation
install-deps:
    pip install -r requirements.txt

# Clean notebook outputs and execution counts
clean-outputs:
    @python3 scripts/clean_outputs.py
