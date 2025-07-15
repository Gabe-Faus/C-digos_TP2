use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INDENT: &str = "    "; // 4 espaços

fn count_leading_indent(line: &str) -> usize {
    let mut count = 0;
    for ch in line.chars() {
        match ch {
            ' ' => count += 1,
            '\t' => count += 4,
            _ => break,
        }
    }
    count
}

fn format_code_preserving_indent(file_path: &str, reserved_words: &[&str]) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut formatted_lines = Vec::new();

    // Compila regex dinâmicos para as palavras reservadas
    let re_patterns: Vec<(String, Regex)> = reserved_words.iter().map(|&kw| {
        let pattern = format!(r"^{}\s+(.*):$", regex::escape(kw));
        (kw.to_string(), Regex::new(&pattern).unwrap())
    }).collect();

    for line_res in reader.lines() {
        let mut line = line_res?;
        line = line.replace("\t", INDENT);
        let original_indent = count_leading_indent(&line);
        let stripped = line.trim();

        if stripped.is_empty() {
            formatted_lines.push(String::new());
            continue;
        }

        let mut matched_structure = false;
        for (kw, re) in &re_patterns {
            if let Some(caps) = re.captures(stripped) {
                if let Some(cond) = caps.get(1) {
                    // Remove espaços extras dentro da condição
                    let cond_str = cond.as_str().split_whitespace().collect::<Vec<_>>().join(" ");
                    let indent_level = original_indent / 4;
                    let formatted_line = format!("{}{} {}:", INDENT.repeat(indent_level), kw, cond_str);
                    formatted_lines.push(formatted_line);
                    matched_structure = true;
                    break;
                }
            }
        }

        if !matched_structure {
            let indent_level = original_indent / 4;
            let mut processed_line = stripped.to_string();
            if stripped.starts_with("print") {
                // Remove espaços entre print e '('
                let print_re = Regex::new(r"print\s*\(").unwrap();
                processed_line = print_re.replace(&processed_line, "print(").to_string();
            }
            formatted_lines.push(format!("{}{}", INDENT.repeat(indent_level), processed_line));
        }
    }

    Ok(formatted_lines)
}

fn main() -> io::Result<()> {
    let reserved_words = [
        "if", "in", "else", "def", "while", "for", "val", "var",
        "return", "Ok", "Err", "Just", "Nothing", "unwrap", "tryUnwrap",
        "isNothing", "isError", "and", "or", "not", "True", "False"
    ];

    let result = format_code_preserving_indent(r"C:\Users\faust\Desktop\teste.txt", &reserved_words)?;

    for line in result {
        println!("{}", line);
    }

    Ok(())
}
