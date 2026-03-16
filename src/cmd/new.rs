use crate::chive::{current_date, scan_directory, ChiveFile, ChiveRecord, CHIVEST_FILE};
use std::io::{self, Write};
use std::path::Path;

pub fn run(include_hidden: bool) -> Result<(), String> {
    if Path::new(CHIVEST_FILE).exists() {
        return Err(
            "A .chivest file already exists here. Use 'chivest update' to sync.".to_string(),
        );
    }

    let files = scan_directory(include_hidden)?;
    let count = files.len();

    if count == 0 {
        println!("No files found in directory (nothing to chivest).");
        return Ok(());
    }

    if count > 50 {
        print!("{count} files found. Continue? [y/N] ");
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    let date = current_date();
    let records: Vec<ChiveRecord> = files
        .into_iter()
        .enumerate()
        .map(|(i, filename)| ChiveRecord {
            id: i + 1,
            filename,
            description: String::new(),
            notes: String::new(),
            last_edited: date.clone(),
        })
        .collect();

    let chive = ChiveFile {
        created: date,
        records,
    };
    chive.write()?;
    println!("Created .chivest with {count} entries.");
    Ok(())
}
