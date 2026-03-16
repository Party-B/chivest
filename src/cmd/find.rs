use crate::chive::{ChiveFile, ChiveRecord};
use crate::color;

pub fn run(query: &str) -> Result<(), String> {
    let chive = ChiveFile::read()?;
    let q = query.to_lowercase();

    let matches: Vec<ChiveRecord> = chive
        .records
        .into_iter()
        .filter(|r| {
            r.filename.to_lowercase().contains(&q)
                || r.description.to_lowercase().contains(&q)
                || r.notes.to_lowercase().contains(&q)
        })
        .collect();

    if matches.is_empty() {
        println!("No matches for '{query}'.");
        return Ok(());
    }

    println!("Found {} match(es) for '{}':", matches.len(), color::yellow(query));
    print_found_table(&matches, query);
    Ok(())
}

/// Highlight occurrences of `query` inside `s`, returning a string padded to `width`.
/// Padding is appended *outside* any ANSI codes so column alignment is preserved.
fn highlight_cell(s: &str, width: usize, query: &str) -> String {
    let lower_s = s.to_lowercase();
    let lower_q = query.to_lowercase();
    let padding = " ".repeat(width.saturating_sub(s.len()));

    if let Some(pos) = lower_s.find(&lower_q) {
        let end = pos + query.len();
        let highlighted = format!(
            "{}{}{}",
            &s[..pos],
            color::yellow(&s[pos..end]),
            &s[end..]
        );
        format!("{highlighted}{padding}")
    } else {
        format!("{s}{padding}")
    }
}

fn print_found_table(records: &[ChiveRecord], query: &str) {
    // Column widths from this result set
    let id_w = records.iter().map(|r| r.id.to_string().len()).max().unwrap_or(1).max(1);
    let file_w = records.iter().map(|r| r.filename.len()).max().unwrap_or(0).max(8);
    let desc_w = records.iter().map(|r| r.description.len()).max().unwrap_or(0).max(11);
    let notes_w = records.iter().map(|r| r.notes.len()).max().unwrap_or(0).max(5);

    let hline = |l: &str, mid: &str, r: &str| {
        let segs: Vec<String> = [id_w, file_w, desc_w, notes_w]
            .iter()
            .map(|&w| "─".repeat(w + 2))
            .collect();
        format!("{}{}{}", l, segs.join(mid), r)
    };

    println!("{}", hline("┌", "┬", "┐"));
    let h = |s: &str, w: usize| color::bold(&format!("{s:<w$}"));
    println!(
        "│ {} │ {} │ {} │ {} │",
        h("#", id_w), h("Filename", file_w), h("Description", desc_w), h("Notes", notes_w)
    );
    println!("{}", hline("├", "┼", "┤"));

    for r in records {
        let id_cell    = format!("{:>id_w$}", r.id);
        let file_cell  = highlight_cell(&r.filename,    file_w,  query);
        let desc_cell  = if r.description.is_empty() {
            color::dim(&format!("{:<desc_w$}", ""))
        } else {
            highlight_cell(&r.description, desc_w, query)
        };
        let notes_cell = if r.notes.is_empty() {
            color::dim(&format!("{:<notes_w$}", ""))
        } else {
            highlight_cell(&r.notes, notes_w, query)
        };
        println!("│ {id_cell} │ {file_cell} │ {desc_cell} │ {notes_cell} │");
    }

    println!("{}", hline("└", "┴", "┘"));
}
