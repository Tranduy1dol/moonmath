#!/usr/bin/env python3
"""
Script to replace GitHub-style alerts with emoji-based notes in Jupyter notebooks.
GitHub's `> [!NOTE]` syntax doesn't render in notebook markdown cells.
"""

import json
import glob
import re

def fix_notes_in_notebook(filepath: str) -> int:
    """
    Replace `> [!NOTE]\n> TODO: ...` with `**📝 Note:** TODO: ...`
    Returns the number of replacements made.
    """
    with open(filepath, 'r', encoding='utf-8') as f:
        notebook = json.load(f)
    
    replacements = 0
    
    for cell in notebook.get('cells', []):
        if cell.get('cell_type') != 'markdown':
            continue
        
        source = cell.get('source', [])
        if not source:
            continue
        
        new_source = []
        i = 0
        while i < len(source):
            line = source[i]
            
            # Check for `> [!NOTE]` pattern (handles `> [!NOTE]\n` or `> [!NOTE]`)
            if re.match(r'^>\s*\[!NOTE\]', line, re.IGNORECASE):
                # Look for the next line with `> TODO: ...` or similar content
                if i + 1 < len(source):
                    next_line = source[i + 1]
                    # Extract content after `> `
                    match = re.match(r'^>\s*(.+)', next_line)
                    if match:
                        content = match.group(1)
                        # Preserve trailing newline if present
                        if content.endswith('\\n'):
                            content = content[:-2]
                        if content.endswith('\n'):
                            new_source.append(f"**📝 Note:** {content}\n")
                        else:
                            new_source.append(f"**📝 Note:** {content}")
                        i += 2  # Skip both lines
                        replacements += 1
                        continue
                
                # If no next line or doesn't match, just convert the NOTE line
                if line.endswith('\n'):
                    new_source.append("**📝 Note:**\n")
                else:
                    new_source.append("**📝 Note:**")
                replacements += 1
                i += 1
                continue
            
            new_source.append(line)
            i += 1
        
        cell['source'] = new_source
    
    with open(filepath, 'w', encoding='utf-8') as f:
        json.dump(notebook, f, indent=1, ensure_ascii=False)
        f.write('\n')
    
    return replacements


def main():
    notebooks = glob.glob('notebooks/*.ipynb')
    total_replacements = 0
    
    for nb_path in sorted(notebooks):
        count = fix_notes_in_notebook(nb_path)
        if count > 0:
            print(f"  ✅ {nb_path}: {count} replacement(s)")
            total_replacements += count
        else:
            print(f"  ⏭️  {nb_path}: no changes")
    
    print(f"\n📊 Total: {total_replacements} replacement(s) across {len(notebooks)} notebooks")


if __name__ == '__main__':
    main()
