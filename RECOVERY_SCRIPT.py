#!/usr/bin/env python3
"""
Recovery script to fix 86 corrupted skill files.
Strategy: Restore from git, then apply targeted fixes.
"""

import subprocess
import sys
from pathlib import Path

SKILLS_DIR = Path(r"c:\dsaengine\src\skills")

corrupted_files = [
    "activity_selection.rs", "add_two_numbers.rs", "asteroid_collision.rs",
    "avl_tree.rs", "b_tree_index.rs", "balance_checker.rs", "bellman_ford.rs",
    "bfs_generator.rs", "bit_manipulation.rs", "bst_validator.rs", "clone_graph.rs",
    "coin_change.rs", "consistent_hashing.rs", "construct_from_traversal.rs",
    "course_schedule.rs", "cycle_detection.rs", "daily_temperatures.rs",
    "dfs_generator.rs", "difference_array.rs", "dp_on_trees.rs",
    # ... (66 more)
]

def restore_from_git(file_path):
    """Restore file from git history"""
    try:
        result = subprocess.run(
            ["git", "checkout", "HEAD", str(file_path)],
            cwd=str(SKILLS_DIR.parent.parent),
            capture_output=True,
            timeout=10
        )
        return result.returncode == 0
    except Exception as e:
        print(f"  Git restore failed: {e}")
        return False

def main():
    print("="*70)
    print("RECOVERY SCRIPT: Restore 86 Corrupted Skill Files")
    print("="*70)
    print()
    
    print("Strategy:")
    print("  1. Restore each corrupted file from git")
    print("  2. Verify restoration successful")
    print("  3. Mark as 'ready for proper refactoring'")
    print()
    
    restored = 0
    failed = 0
    
    for filename in corrupted_files[:5]:  # Test with first few
        file_path = SKILLS_DIR / filename.split("/")[0] / filename.split("/")[-1]
        
        if not file_path.exists():
            print(f"❌ {filename}: File not found at {file_path}")
            failed += 1
            continue
        
        print(f"Restoring {filename}...", end=" ")
        if restore_from_git(file_path):
            print("✓")
            restored += 1
        else:
            print("✗")
            failed += 1
    
    print()
    print(f"Results: {restored} restored, {failed} failed")
    print()
    print("NEXT STEPS:")
    print("  1. Run: git status (verify all files restored)")
    print("  2. Then: Apply Phase 3 refactoring with PROPER templates")
    print("  3. Each skill must be individually analyzed for its signature")

if __name__ == "__main__":
    main()
