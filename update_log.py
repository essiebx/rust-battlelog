#!/usr/bin/env python3
"""
Interactive Rust learning log appender
Prompts for a custom entry and appends it to README.md
"""

from datetime import date

def append_custom_log():
    today = date.today().strftime("%B %d, %Y")
    print(f"📝 Adding log for {today}")
    print("Type your entry (use \\n for new lines), then press Enter:\n")

    entry = input("➤ Entry: ").replace("\\n", "\n- ")
    formatted_entry = f"\n### 🧠 {today}\n- {entry}\n"

    try:
        with open("README.md", "a", encoding="utf-8") as f:
            f.write(formatted_entry)
        print("✅ README.md updated successfully!")
    except FileNotFoundError:
        print("❌ README.md not found. Make sure you're in the project root.")

if __name__ == "__main__":
    append_custom_log()
