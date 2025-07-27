# Recursive Grep

A recursive command-line utility written in Rust that searches for a given substring or pattern across all files in a directory tree. Inspired by the UNIX `grep` tool, this implementation supports case-insensitive matching, regex queries, line limiting, and match counting.

## 🔧 Features

- Recursively searches files in a directory and its subdirectories
- Prints matching lines with line numbers and file names
- Supports optional:
  - Case-insensitive search
  - Regex-based patterns
  - Count-only mode (suppress line output)
  - Line limit (max number of matches before exit)

## 🧪 Usage

```bash
recursive_grep [OPTIONS] <PATTERN> <DIRECTORY>
````

### Required Arguments:

* `<PATTERN>` — The string or regex pattern to search for
* `<DIRECTORY>` — The root directory to start the search from

### Options:

| Flag                  | Description                                 | Default    |
| --------------------- | ------------------------------------------- | ---------- |
| `--ignore-case`, `-i` | Case-insensitive matching                   | `false`    |
| `--count`, `-c`       | Only print the number of matches per file   | `false`    |
| `--regex`, `-r`       | Enable regular expression pattern matching  | `false`    |
| `--max-lines <N>`     | Stop after N total matches across all files | `infinite` |

## 📦 Build & Run

Requires Rust toolchain (`cargo`).

```bash
# Build
cargo build --release

# Example usage
./target/release/recursive_grep -i --max-lines 10 "search_term" ./src
```

## 📁 Example Output

```text
src/lib.rs:42: fn search_util(pattern: &str) {
src/main.rs:13: println!("Search term found!");
```

With `--count`:

```text
src/lib.rs: 3 matches
src/main.rs: 1 match
```

## 🛠️ Implementation Notes

* Traverses directories using Rust's `walkdir` crate
* Pattern matching with `regex` crate if `--regex` is enabled
* File reading uses buffered IO for efficiency
* Ignores binary files and inaccessible paths gracefully

## 🧑‍🎓 License & Attribution

This project was developed for educational purposes as part of a university assignment. It is not intended for production use or redistribution.

```
