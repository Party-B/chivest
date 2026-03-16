use crate::chive::{ChiveFile, ChiveRecord};
use crate::color;

pub fn run(long: bool) -> Result<(), String> {
    let chive = ChiveFile::read()?;

    if chive.records.is_empty() {
        println!("No entries in .chivest.");
        return Ok(());
    }

    let (id_w, file_w, desc_w, notes_w) = col_widths(&chive.records);

    if long {
        print_table_long(&chive.records, id_w, file_w, desc_w, notes_w);
        println!("\n  Created: {}", chive.created);
    } else {
        print_table(&chive.records, id_w, file_w, desc_w, notes_w);
    }

    Ok(())
}

pub fn col_widths(records: &[ChiveRecord]) -> (usize, usize, usize, usize) {
    let id_w = records.iter().map(|r| digit_count(r.id)).max().unwrap_or(1).max(1);
    let file_w = records.iter().map(|r| r.filename.len()).max().unwrap_or(0).max(8);
    let desc_w = records.iter().map(|r| r.description.len()).max().unwrap_or(0).max(11);
    let notes_w = records.iter().map(|r| r.notes.len()).max().unwrap_or(0).max(5);
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
        // Pad first, then apply color so borders stay aligned
        let id_cell   = format!("{:>id_w$}", r.id);
        let file_cell = format!("{:<file_w$}", r.filename);
        let desc_cell = {
            let s = format!("{:<desc_w$}", r.description);
            if r.description.is_empty() { color::dim(&s) } else { s }
        };
        let notes_cell = {
            let s = format!("{:<notes_w$}", r.notes);
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
        let file_cell = format!("{:<file_w$}", r.filename);
        let desc_cell = {
            let s = format!("{:<desc_w$}", r.description);
            if r.description.is_empty() { color::dim(&s) } else { s }
        };
        let notes_cell = {
            let s = format!("{:<notes_w$}", r.notes);
            if r.notes.is_empty() { color::dim(&s) } else { s }
        };
        let date_cell = format!("{:<date_w$}", r.last_edited);
        println!("│ {id_cell} │ {file_cell} │ {desc_cell} │ {notes_cell} │ {date_cell} │");
    }

    println!("{}", hline(cols, "└", "┴", "┘"));
}
