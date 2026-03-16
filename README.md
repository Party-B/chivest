# chivest

A terminal utility for annotating and tracking the files in a directory. Create a `.chivest` index, add descriptions and notes to each entry, search and filter, and keep it in sync as files change.

## Install

```bash
git clone <repo>
cd chivest
cargo install --path .
```

Requires Rust 1.85+ (edition 2024). The binary is placed in `~/.cargo/bin/chivest`.

## Usage

### `chivest new [-a]`
Create a `.chivest` file in the current directory, listing all files.

```bash
chivest new        # exclude hidden files
chivest new -a     # include hidden files (-la also works)
```

Prompts for confirmation if more than 50 files are found. Errors if a `.chivest` already exists — use `update` to sync instead.

---

### `chivest ls [-l]`
Display the chivest table.

```bash
chivest ls         # item number, filename, description, notes
chivest ls -l      # adds a "Last Edited" column and creation date (-la also works)
```

Empty description and notes cells are dimmed. Headers are bold. Colors are suppressed automatically when output is piped.

---

### `chivest edit [N]`
Set or update the description and notes for an entry.

```bash
chivest edit       # shows the table, then prompts for item number
chivest edit 3     # edit item 3 directly
```

At each prompt: press **Enter** to keep the current value, type **`-`** to clear it, or type anything else to replace it.

---

### `chivest find <query>`
Search entries by filename, description, or notes (case-insensitive). Matching text is highlighted in the results table.

```bash
chivest find main
chivest find "entry point"
```

---

### `chivest rm [N] [--chive]`
Clear the description and notes for an entry, or remove the `.chivest` file entirely.

```bash
chivest rm         # shows table, prompts for item number to clear
chivest rm 2       # clear item 2 directly
chivest rm --chive # remove the entire .chivest file (prompts for confirmation)
```

---

### `chivest update [-a] [-y]`
Sync `.chivest` with the current state of the directory. New files are added automatically; removed files prompt for confirmation before being dropped.

```bash
chivest update        # prompt for each removed file
chivest update -y     # auto-confirm all removals
chivest update -a     # include hidden files
chivest -S            # pacman-style alias for update
chivest -Sy           # pacman-style alias for update -y
```

---

## .chivest file format

The `.chivest` file is plain pipe-delimited text, safe to open and edit by hand:

```
# chivest
# created: 2026-03-17
id|filename|description|notes|last_edited
1|README.md|Project readme||2026-03-17
2|Cargo.toml|Rust manifest||2026-03-17
3|src/main.rs|Entry point||2026-03-17
```

Note: the `|` character should not be used inside description or notes fields.

---

## Ideas for further development

- **`chivest export`** — dump the table to stdout as Markdown, CSV, or JSON for use in other tools or documentation
- **Shell completions** — generate tab completions for bash/zsh/fish via `clap_complete`, making item-number arguments autocomplete from the live `.chivest` file
- **Recursive mode** — a `--recursive` flag for `new` and `update` to track files in subdirectories, with filenames stored as relative paths
- **Tags** — add a tags field to each record with `chivest tag <N> <tag>`, and filter with `chivest ls --tag=<tag>`
- **`chivest show <N>`** — full-detail view of a single entry, useful when descriptions grow long enough to be truncated in the table
