use clap::{Arg, Command};
use difference::{Changeset, Difference};
use serde_yaml::{from_str, to_string, Value};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    process,
};

fn main() {
    let matches = app().get_matches();
    let note_path = matches.get_one::<String>("file").unwrap();
    let field = matches.get_one::<String>("field").unwrap();
    let new_value = matches.get_one::<String>("new_value").unwrap();

    if !Path::new(note_path).exists() {
        eprintln!("File not found: {}", note_path);
        process::exit(1);
    }

    let content = fs::read_to_string(note_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", note_path);
        process::exit(1);
    });

    if content.is_empty() {
        eprintln!("File is empty: {}", note_path);
        process::exit(1);
    }

    let (frontmatter, rest) = extract_frontmatter(&content);
    if frontmatter.is_empty() {
        eprintln!("No frontmatter found in {}", note_path);
        println!("{}", content);
        process::exit(1);
    }

    let mut yaml: Value = from_str(&frontmatter).unwrap_or_else(|_| {
        eprintln!("Failed to parse YAML in {}", note_path);
        process::exit(1);
    });

    if yaml.get(field).is_some() {
        yaml[field] = Value::String(new_value.to_string());
        let new_frontmatter = to_string(&yaml).unwrap_or_else(|_| {
            eprintln!("Failed to serialize YAML in {}", note_path);
            process::exit(1);
        });

        let new_content = format!("---\n{}---\n{}", new_frontmatter, rest);

        println!("Old content:");
        println!(
            "{}\n--------------------------------------------------\n",
            content
        );
        println!("New content:");
        display_differences(&content, &new_content);

        if confirm_save() {
            fs::write(note_path, new_content).unwrap_or_else(|err| {
                eprintln!("Failed to write file: {}", err);
                process::exit(1);
            });
            println!("Field \"{}\" updated in {}", field, note_path);
        } else {
            println!("Changes discarded");
        }
    } else {
        eprintln!("Field \"{}\" not found in {}", field, note_path);
    }
}

fn app() -> Command {
    Command::new("upfront")
        .version("1.0")
        .author("Oleksii luchnikov <oleksiiluchnikov@gmail.com>")
        .about("Updates a specified field in the YAML frontmatter of a note")
        .arg(Arg::new("file").required(true).index(1))
        .arg(Arg::new("field").required(true).index(2))
        .arg(Arg::new("new_value").required(true).index(3))
}

fn extract_frontmatter(content: &str) -> (String, String) {
    let mut parts = content.split("---\n").skip(1).fuse();
    let frontmatter = parts.next().unwrap_or_default().to_owned();
    let rest = parts.next().map(String::from).unwrap_or_default();
    (frontmatter, rest)
}

fn display_differences(old: &str, new: &str) {
    let Changeset { diffs, .. } = Changeset::new(old, new, "\n");

    for diff in diffs {
        match diff {
            Difference::Same(ref x) => println!("{}", x),
            Difference::Add(ref x) => println!("\x1b[32m{}\x1b[0m", x),
            Difference::Rem(ref x) => println!("\x1b[31m{}\x1b[0m", x),
        }
    }
}

fn confirm_save() -> bool {
    let mut input = String::new();
    print!("Save changes? [y/N] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}


