use crate::chive::{current_date, scan_directory, ChiveFile, ChiveRecord};
use crate::color;
use std::collections::HashSet;
use std::io::{self, Write};

pub fn run(include_hidden: bool, yes: bool) -> Result<(), String> {
    let mut chive = ChiveFile::read()?;
    let files = scan_directory(include_hidden)?;

    let existing: HashSet<String> = chive.records.iter().map(|r| r.filename.clone()).collect();
    let current: HashSet<String> = files.iter().cloned().collect();

    let mut new_files: Vec<String> = files
        .into_iter()
        .filter(|f| !existing.contains(f))
        .collect();
    new_files.sort();

    let removed_ids: Vec<usize> = chive
        .records
        .iter()
        .filter(|r| !current.contains(&r.filename))
        .map(|r| r.id)
        .collect();

    if new_files.is_empty() && removed_ids.is_empty() {
        println!(".chivest is already up to date.");
        return Ok(());
    }

    // Add new files
    for filename in &new_files {
        let next_id = chive.records.iter().map(|r| r.id).max().unwrap_or(0) + 1;
        println!("  {} {filename}", color::green("+"));
        chive.records.push(ChiveRecord {
            id: next_id,
            filename: filename.clone(),
            description: String::new(),
            notes: String::new(),
            last_edited: current_date(),
        });
    }

    // Prompt for removed files
    let mut ids_to_remove = Vec::new();
    for id in &removed_ids {
        let filename = chive
            .records
            .iter()
            .find(|r| r.id == *id)
            .map(|r| r.filename.as_str())
            .unwrap_or("?");

        let remove = if yes {
            println!("  {} {filename}", color::red("-"));
            true
        } else {
            print!(
                "  {} '{}' no longer exists. Remove from chivest? [y/N] ",
                color::red("-"),
                filename
            );
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            input.trim().eq_ignore_ascii_case("y")
        };

        if remove {
            ids_to_remove.push(*id);
        }
    }

    chive.records.retain(|r| !ids_to_remove.contains(&r.id));
    chive.write()?;

    println!(
        "\nDone. {} added, {} removed.",
        new_files.len(),
        ids_to_remove.len()
    );
    Ok(())
}
