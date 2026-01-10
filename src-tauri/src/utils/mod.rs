use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use crate::models::clipboard_item::ClipboardKind;

pub fn hash_content(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}


pub fn classify_clipboard(content: &str) -> ClipboardKind {
    let trimmed = content.trim();

    if is_location(trimmed) {
        return ClipboardKind::location;
    }

    if is_url(trimmed) {
        return ClipboardKind::link;
    }

    if is_code(trimmed) {
        return ClipboardKind::code;
    }

    ClipboardKind::text
}

fn is_location(text: &str) -> bool {
    let text = text.trim();

    if text.is_empty() {
        return false;
    }


    let map_url_patterns = [
        r"https?://(www\.)?google\.[^/]+/maps",
        r"https?://maps\.google\.[^/]+",
        r"https?://(www\.)?openstreetmap\.org",
        r"https?://maps\.apple\.com",
        r"https?://(www\.)?waze\.com",
    ];

    let map_url_re = Regex::new(&map_url_patterns.join("|")).unwrap();
    if map_url_re.is_match(text) {
        return true;
    }

    if text.starts_with("geo:") || text.starts_with("maps://") {
        return true;
    }


    let lat_lng_re = Regex::new(
        r"(-?\d{1,3}\.\d+)\s*,\s*(-?\d{1,3}\.\d+)"
    ).unwrap();

    if lat_lng_re.is_match(text) {
        return true;
    }

  
    let named_coords_re = Regex::new(
        r"(lat|latitude)\s*[:=]\s*-?\d+(\.\d+)?\s*(,|\s)\s*(lon|lng|longitude)\s*[:=]\s*-?\d+(\.\d+)?"
    ).unwrap();

    if named_coords_re.is_match(text) {
        return true;
    }


    let plus_code_re = Regex::new(
        r"\b[A-Z0-9]{4,}\+[A-Z0-9]{2,}\b"
    ).unwrap();

    if plus_code_re.is_match(text) {
        return true;
    }

    false
}


fn is_url(text: &str) -> bool {
    let url_re = Regex::new(
        r"^(https?|ftp)://[^\s/$.?#].[^\s]*$"
    ).unwrap();

    url_re.is_match(text)
}

pub fn is_code(text: &str) -> bool {
    let text = text.trim();
    if text.is_empty() {
        return false;
    }

    let mut score = 0;

    let strong_patterns = [
        // C / C++
        r"#include\s*<[^>]+>",
        r"\b(int|char|float|double|void)\s+\w+\s*\(",

        // Go
        r"\bpackage\s+\w+",
        r"\bfunc\s+\w+\s*\(",

        // JS / TS
        r"\b(const|let|var)\s+\w+",
        r"\bexport\s+(default\s+)?(class|function|const)",
        r"\binterface\s+\w+",
        r"\btype\s+\w+\s*=",
        r"=>",

        // Rust
        r"\bfn\s+\w+\s*\(",
        r"\bimpl\s+\w+",
        r"\buse\s+[\w:]+",
        r"\bpub\s+(fn|struct|enum)",

        // Python
        r"\bdef\s+\w+\s*\(",
        r"\bclass\s+\w+:",
        r"\bimport\s+\w+",
        r"\bfrom\s+\w+\s+import\s+",

        // Java / Kotlin
        r"\bpublic\s+(class|static|void)",
        r"\bSystem\.out\.println",
        r"\bfun\s+\w+\(",

        // C#
        r"\busing\s+System",
        r"\bnamespace\s+\w+",

        // Swift
        r"\bimport\s+Foundation",
        r"\bfunc\s+\w+\(",

        // SQL
        r"\bSELECT\s+.+\s+FROM\s+",
        r"\bINSERT\s+INTO\s+",
        r"\bCREATE\s+TABLE\s+",

        // Shell
        r"^#!/bin/(ba)?sh",
        r"\b(cd|ls|grep|awk|sed)\b",

        // Config / data
        r#""\w+"\s*:"#,
        r"\w+\s*=\s*.+",
        r"\[\[?\w+\]\]?",

        // HTML / XML
        r"<\/?\w+[^>]*>",

        // CSS
        r"\.[\w-]+\s*\{",
        r"\w+\s*:\s*[^;]+;",
    ];

    let strong_re = Regex::new(&strong_patterns.join("|")).unwrap();
    if strong_re.is_match(text) {
        score += 3;
    }

    // ----------------------------
    // 2. Estrutura de código
    // ----------------------------
    let lines: Vec<&str> = text.lines().collect();
    let line_count = lines.len();

    if line_count >= 3 {
        score += 1;
    }

    // Indentação consistente
    let indented_lines = lines
        .iter()
        .filter(|l| l.starts_with("    ") || l.starts_with('\t'))
        .count();

    if indented_lines >= 2 {
        score += 2;
    }

    // ----------------------------
    // 3. Símbolos característicos
    // ----------------------------
    let symbols = ["{", "}", ";", "(", ")", "::", "=>", "<", ">", "[", "]"];

    let symbol_hits = symbols
        .iter()
        .filter(|s| text.contains(*s))
        .count();

    if symbol_hits >= 3 {
        score += 2;
    }

    // ----------------------------
    // 4. Proporção linhas "codadas"
    // ----------------------------
    let code_like_lines = lines.iter().filter(|line| {
        strong_re.is_match(line)
            || line.contains(';')
            || line.contains('{')
            || line.contains("=>")
            || line.contains("::")
    }).count();

    if line_count > 0 {
        let ratio = code_like_lines as f32 / line_count as f32;
        if ratio > 0.4 {
            score += 2;
        }
    }

    // ----------------------------
    // 5. Penalizações (texto natural)
    // ----------------------------
    let natural_language_patterns = [
        r"\b(the|and|with|that|this|porque|então|quando|para)\b",
    ];

    let nl_re = Regex::new(&natural_language_patterns.join("|")).unwrap();
    if nl_re.is_match(text) && line_count < 3 {
        score -= 2;
    }

    score >= 4
}
