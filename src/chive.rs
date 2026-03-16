use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub const CHIVEST_FILE: &str = ".chivest";

#[derive(Debug, Clone)]
pub struct ChiveRecord {
    pub id: usize,
    pub filename: String,
    pub description: String,
    pub notes: String,
    pub last_edited: String,
}

#[derive(Debug)]
pub struct ChiveFile {
    pub created: String,
    pub records: Vec<ChiveRecord>,
}

impl ChiveFile {
    pub fn read() -> Result<Self, String> {
        if !Path::new(CHIVEST_FILE).exists() {
            return Err(
                "No .chivest file found in this directory. Run 'chivest new' to create one."
                    .to_string(),
            );
        }
        let file = fs::File::open(CHIVEST_FILE)
            .map_err(|e| format!("Failed to open .chivest: {e}"))?;
        let reader = io::BufReader::new(file);
        let mut created = String::new();
        let mut records = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if trimmed.starts_with("# created:") {
                created = trimmed["# created:".len()..].trim().to_string();
            } else if trimmed.starts_with('#')
                || trimmed.is_empty()
                || trimmed == "id|filename|description|notes|last_edited"
            {
                continue;
            } else {
                let parts: Vec<&str> = trimmed.splitn(5, '|').collect();
                if parts.len() >= 2 {
                    records.push(ChiveRecord {
                        id: parts[0].trim().parse().unwrap_or(0),
                        filename: parts.get(1).unwrap_or(&"").trim().to_string(),
                        description: parts.get(2).unwrap_or(&"").trim().to_string(),
                        notes: parts.get(3).unwrap_or(&"").trim().to_string(),
                        last_edited: parts.get(4).unwrap_or(&"").trim().to_string(),
                    });
                }
            }
        }

        Ok(ChiveFile { created, records })
    }

    pub fn write(&self) -> Result<(), String> {
        let mut file = fs::File::create(CHIVEST_FILE)
            .map_err(|e| format!("Failed to write .chivest: {e}"))?;
        writeln!(file, "# chivest").map_err(|e| e.to_string())?;
        writeln!(file, "# created: {}", self.created).map_err(|e| e.to_string())?;
        writeln!(file, "id|filename|description|notes|last_edited").map_err(|e| e.to_string())?;
        for r in &self.records {
            writeln!(
                file,
                "{}|{}|{}|{}|{}",
                r.id, r.filename, r.description, r.notes, r.last_edited
            )
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

pub fn current_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn scan_directory(include_hidden: bool) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let entries =
        fs::read_dir(".").map_err(|e| format!("Failed to read directory: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !include_hidden && name.starts_with('.') {
            continue;
        }
        if name == CHIVEST_FILE {
            continue;
        }
        files.push(name);
    }

    files.sort();
    Ok(files)
}
