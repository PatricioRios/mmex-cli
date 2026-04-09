use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, ContentArrangement, Table};
use serde::Serialize;

pub enum OutputFormat {
    Table,
    Json,
}

impl OutputFormat {
    pub fn from_flag(json: bool) -> Self {
        if json {
            OutputFormat::Json
        } else {
            OutputFormat::Table
        }
    }
}

pub fn print_json<T: Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let header_cells: Vec<Cell> = headers.iter().map(|h| Cell::new(h)).collect();
    table.set_header(header_cells);

    for row in rows {
        table.add_row(row.iter().map(|cell| Cell::new(cell)));
    }

    println!("{}", table);
}

pub fn format_option<T: std::fmt::Display>(opt: Option<T>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_default()
}

pub fn format_bool(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
