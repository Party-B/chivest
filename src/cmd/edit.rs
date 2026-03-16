use crate::chive::{current_date, ChiveFile};
use crate::cmd::ls::{col_widths, print_table};
use std::io::{self, Write};

pub fn run(item: Option<usize>) -> Result<(), String> {
    let mut chive = ChiveFile::read()?;

    let target_id = match item {
        Some(id) => id,
        None => {
            let (id_w, file_w, desc_w, notes_w) = col_widths(&chive.records);
            print_table(&chive.records, id_w, file_w, desc_w, notes_w);
            print!("\nEnter item number to edit: ");
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

    println!("Editing '{}' (Enter = keep current, '-' = clear)", record.filename);

    // --- Description ---
    if record.description.is_empty() {
        print!("  Description: ");
    } else {
        print!("  Description [{}]: ", record.description);
    }
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    match input.trim() {
        "" => {}
        "-" => record.description.clear(),
        s  => record.description = s.to_string(),
    }

    // --- Notes ---
    if record.notes.is_empty() {
        print!("  Notes: ");
    } else {
        print!("  Notes [{}]: ", record.notes);
    }
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    match input.trim() {
        "" => {}
        "-" => record.notes.clear(),
        s  => record.notes = s.to_string(),
    }

    record.last_edited = current_date();
    chive.write()?;
    println!("Saved.");
    Ok(())
}
