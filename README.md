# upfront

upfront is a command-line tool that updates a specified field in the YAML frontmatter of a note file.

## Usage

```
upfront <file> <field> <new_value>
```

- `<file>`: The path to the note file.
- `<field>`: The field in the frontmatter to update.
- `<new_value>`: The new value for the specified field.

## Example

Assuming you have a note file `note.md` with the following content:

```markdown
---
title: My Note
tags:
  - note
  - example
---

# My Note

This is an example note.
```

To update the `title` field in the frontmatter, run:

```
upfront note.md title "Updated Note Title"
```

The tool will display the old and new content, and prompt you to confirm the changes before saving the file.

## Installation

1. Clone the repository:

```
git clone https://github.com/your-username/upfront.git
```

2. Build the project:

```
cd upfront
cargo build --release
```

3. Move the binary to a directory in your `PATH`:

```
mv target/release/upfront /usr/local/bin
```

Now you can run `upfront` from anywhere in your terminal.

## Dependencies

- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [difference](https://crates.io/crates/difference) - Displaying differences between strings
- [serde_yaml](https://crates.io/crates/serde_yaml) - YAML serialization and deserialization

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
