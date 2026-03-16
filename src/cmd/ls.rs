use crate::chive::{ChiveFile, ChiveRecord};
use crate::color;

const MAX_FILE_W: usize  = 40;
const MAX_DESC_W: usize  = 40;
const MAX_NOTES_W: usize = 30;

/// Truncate a string to `max` chars, appending "..." if cut.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max.saturating_sub(3)).collect::<String>() + "..."
    }
}

pub fn run(long: bool, file: Option<String>) -> Result<(), String> {
    let chive = ChiveFile::read()?;

    if chive.records.is_empty() {
        println!("No entries in .chivest.");
        return Ok(());
    }

    let records: Vec<_> = match &file {
        Some(name) => {
            // Strip any leading path components so both `file.txt` and `./file.txt` work
            let name = std::path::Path::new(name)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(name.as_str());
            let found: Vec<_> = chive.records.iter()
                .filter(|r| r.filename == name)
                .cloned()
                .collect();
            if found.is_empty() {
                println!("No chivest entry for '{name}'.");
                return Ok(());
            }
            found
        }
        None => chive.records.clone(),
    };

    let (id_w, file_w, desc_w, notes_w) = col_widths(&records);

    if long {
        print_table_long(&records, id_w, file_w, desc_w, notes_w);
        if file.is_none() {
            println!("\n  Created: {}", chive.created);
        }
    } else {
        print_table(&records, id_w, file_w, desc_w, notes_w);
    }

    Ok(())
}

pub fn col_widths(records: &[ChiveRecord]) -> (usize, usize, usize, usize) {
    let id_w    = records.iter().map(|r| digit_count(r.id)).max().unwrap_or(1).max(1);
    let file_w  = records.iter().map(|r| r.filename.chars().count()).max().unwrap_or(0).max(8).min(MAX_FILE_W);
    let desc_w  = records.iter().map(|r| r.description.chars().count()).max().unwrap_or(0).max(11).min(MAX_DESC_W);
    let notes_w = records.iter().map(|r| r.notes.chars().count()).max().unwrap_or(0).max(5).min(MAX_NOTES_W);
    (id_w, file_w, desc_w, notes_w)
}

fn digit_count(n: usize) -> usize {
    if n == 0 { 1 } else { n.ilog10() as usize + 1 }
}

fn hline(parts: &[usize], l: &str, mid: &str, r: &str) -> String {
    let segments: Vec<String> = parts.iter().map(|&w| "─".repeat(w + 2)).collect();
    format!("{}{}{}", l, segments.join(mid), r)
}

pub fn print_table(records: &[ChiveRecord], id_w: usize, file_w: usize, desc_w: usize, notes_w: usize) {
    let cols = &[id_w, file_w, desc_w, notes_w];
    println!("{}", hline(cols, "┌", "┬", "┐"));

    // Bold header
    let h = |s: &str, w: usize| color::bold(&format!("{s:<w$}"));
    println!(
        "│ {} │ {} │ {} │ {} │",
        h("#", id_w), h("Filename", file_w), h("Description", desc_w), h("Notes", notes_w)
    );
    println!("{}", hline(cols, "├", "┼", "┤"));

    for r in records {
        // Truncate then pad so borders stay aligned regardless of content length
        let id_cell   = format!("{:>id_w$}", r.id);
        let file_cell = format!("{:<file_w$}", truncate(&r.filename, file_w));
        let desc_cell = {
            let s = format!("{:<desc_w$}", truncate(&r.description, desc_w));
            if r.description.is_empty() { color::dim(&s) } else { s }
        };
        let notes_cell = {
            let s = format!("{:<notes_w$}", truncate(&r.notes, notes_w));
            if r.notes.is_empty() { color::dim(&s) } else { s }
        };
        println!("│ {id_cell} │ {file_cell} │ {desc_cell} │ {notes_cell} │");
    }

    println!("{}", hline(cols, "└", "┴", "┘"));
}

fn print_table_long(records: &[ChiveRecord], id_w: usize, file_w: usize, desc_w: usize, notes_w: usize) {
    let date_w = 11; // "Last Edited" header
    let cols = &[id_w, file_w, desc_w, notes_w, date_w];
    println!("{}", hline(cols, "┌", "┬", "┐"));

    let h = |s: &str, w: usize| color::bold(&format!("{s:<w$}"));
    println!(
        "│ {} │ {} │ {} │ {} │ {} │",
        h("#", id_w), h("Filename", file_w), h("Description", desc_w),
        h("Notes", notes_w), h("Last Edited", date_w)
    );
    println!("{}", hline(cols, "├", "┼", "┤"));

    for r in records {
        let id_cell   = format!("{:>id_w$}", r.id);
        let file_cell = format!("{:<file_w$}", truncate(&r.filename, file_w));
        let desc_cell = {
            let s = format!("{:<desc_w$}", truncate(&r.description, desc_w));
            if r.description.is_empty() { color::dim(&s) } else { s }
        };
        let notes_cell = {
            let s = format!("{:<notes_w$}", truncate(&r.notes, notes_w));
            if r.notes.is_empty() { color::dim(&s) } else { s }
        };
        let date_cell = format!("{:<date_w$}", r.last_edited);
        println!("│ {id_cell} │ {file_cell} │ {desc_cell} │ {notes_cell} │ {date_cell} │");
    }

    println!("{}", hline(cols, "└", "┴", "┘"));
}
