use crate::chive::{current_date, ChiveFile, CHIVEST_FILE};
use crate::cmd::ls::{col_widths, print_table};
use std::fs;
use std::io::{self, Write};

pub fn run(remove_chive: bool, item: Option<usize>) -> Result<(), String> {
    if remove_chive {
        print!("Remove .chivest file? This cannot be undone. [y/N] ");
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        if input.trim().eq_ignore_ascii_case("y") {
            fs::remove_file(CHIVEST_FILE)
                .map_err(|e| format!("Failed to remove .chivest: {e}"))?;
            println!("Removed .chivest.");
        } else {
            println!("Aborted.");
        }
        return Ok(());
    }

    let mut chive = ChiveFile::read()?;

    let target_id = match item {
        Some(id) => id,
        None => {
            let (id_w, file_w, desc_w, notes_w) = col_widths(&chive.records);
            print_table(&chive.records, id_w, file_w, desc_w, notes_w);
            print!("\nEnter item number to clear description and notes: ");
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            input
                .trim()
                .parse::<usize>()
                .map_err(|_| "Invalid item number.".to_string())?
        }
    };

    let record = chive
        .records
        .iter_mut()
        .find(|r| r.id == target_id)
        .ok_or_else(|| format!("No item with id {target_id}."))?;

    let fname = record.filename.clone();
    record.description.clear();
    record.notes.clear();
    record.last_edited = current_date();

    chive.write()?;
    println!("Cleared description and notes for '{fname}'.");
    Ok(())
}
