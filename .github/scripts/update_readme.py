#!/usr/bin/env python3

import re
import os
from datetime import datetime, timezone
from zoneinfo import ZoneInfo

def parse_test_output(filename):
    """Parse cargo test output to determine which days are completed."""
    completed_days = set()
    completed_parts = {}  # Track which parts are completed for each day
    
    with open(filename, 'r') as f:
        content = f.read()
    
    # Parse regular and gold part tests
    day_patterns = [
        (r'test day(\d+)::tests::test_day\d+(?!_gold).*ok', 'part1'),  # Regular part
        (r'test day(\d+)_gold::tests::test_day\d+_gold.*ok', 'part2')  # Gold part
    ]
    
    for pattern, part in day_patterns:
        matches = re.finditer(pattern, content, re.MULTILINE)
        for match in matches:
            day_num = int(match.group(1))
            if day_num not in completed_parts:
                completed_parts[day_num] = set()
            completed_parts[day_num].add(part)
    
    # A day is fully completed only if both parts are done
    for day, parts in completed_parts.items():
        if len(parts) == 2:  # Both part1 and part2 are completed
            completed_days.add(day)
    
    return completed_days, completed_parts

def get_day_status(day_num, completed_days, completed_parts):
    """Determine the status for a given day."""
    today = datetime.now(ZoneInfo("America/New_York"))
    current_year = today.year
    challenge_start = datetime(current_year, 12, 1, tzinfo=ZoneInfo("America/New_York"))
    challenge_day = datetime(current_year, 12, day_num, tzinfo=ZoneInfo("America/New_York"))
    
    if challenge_day > today:
        return "⏳", "⏳", "Upcoming"
    
    part1 = "⭐️" if day_num in completed_parts and 'part1' in completed_parts[day_num] else "❌"
    part2 = "⭐️" if day_num in completed_parts and 'part2' in completed_parts[day_num] else "❌"
    
    if day_num in completed_days:
        status = "Complete"
    elif challenge_day <= today:
        # Check if tests exist for this day
        if os.path.exists(f"src/day{day_num}.rs") or os.path.exists(f"src/day{day_num}_gold.rs"):
            if part1 == "⭐️" and part2 == "❌":
                status = "Part 1 only"
            else:
                status = "Not completed"
        else:
            part1 = "🚫"
            part2 = "🚫"
            status = "Skipped"
    
    return part1, part2, status

def update_readme():
    """Update the README.md with current progress."""
    completed_days, completed_parts = parse_test_output(os.environ['TEST_OUTPUT'])
    
    with open('README.md', 'r') as f:
        lines = f.readlines()
    
    # Find the table in the README
    table_start = -1
    table_end = -1
    for i, line in enumerate(lines):
        if '## Progress' in line:
            table_start = i
        elif table_start != -1 and line.strip() and not line.startswith('|') and not line.startswith('#'):
            table_end = i - 1
            break
    
    if table_start == -1:
        raise Exception("Could not find progress table in README")
    
    # Generate new table rows
    new_rows = []
    new_rows.append('| Day   | Part 1 | Part 2 |\n')
    new_rows.append('|-------|--------|--------|\n')
    
    for day in range(1, 26):
        part1, part2, status = get_day_status(day, completed_days, completed_parts)
        day_str = f"{day:>2}"
        # Remove any extra spaces around emojis and center them
        part1 = part1.strip()
        part2 = part2.strip()
        new_rows.append(f'| {day_str:<5} |   {part1}   |   {part2}   |\n')
    
    new_rows.append('\n')
    
    # Replace the old table with the new one
    lines[table_start+1:table_end+1] = new_rows
    
    # Write the updated content back to README
    with open('README.md', 'w') as f:
        f.writelines(lines)

if __name__ == '__main__':
    update_readme()
