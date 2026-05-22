use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::str;

use figlet_rs::FIGlet;
use serde_json::{json, Value};

struct DocumentStore {
    documents: HashMap<String, String>,
}

impl DocumentStore {
    fn new() -> Self {
        Self { documents: HashMap::new() }
    }

    fn open(&mut self, uri: String, text: String) {
        self.documents.insert(uri, text);
    }

    fn change(&mut self, uri: &str, text: String) {
        self.documents.insert(uri.to_string(), text);
    }

    fn close(&mut self, uri: &str) {
        self.documents.remove(uri);
    }

    fn get(&self, uri: &str) -> Option<&str> {
        self.documents.get(uri).map(|s| s.as_str())
    }
}

struct LspServer {
    docs: DocumentStore,
}

impl LspServer {
    fn new() -> Self {
        Self { docs: DocumentStore::new() }
    }

    fn handle_message(&mut self, msg: &Value) -> Option<Value> {
        let id = msg.get("id");
        let method = msg.get("method")?.as_str()?;
        let params = msg.get("params");

        match method {
            "initialize" => Some(self.initialize(id.unwrap(), params)),
            "initialized" => None,
            "shutdown" => {
                Some(json!({"jsonrpc": "2.0", "id": id, "result": null}))
            }
            "exit" => std::process::exit(0),
            "textDocument/didOpen" => {
                if let Some(p) = params { self.did_open(p); }
                None
            }
            "textDocument/didChange" => {
                if let Some(p) = params { self.did_change(p); }
                None
            }
            "textDocument/didClose" => {
                if let Some(p) = params { self.did_close(p); }
                None
            }
            "textDocument/codeAction" => {
                Some(self.code_action(id.unwrap(), params.unwrap()))
            }
            _ => {
                eprintln!("[ascii-banner-lsp] Unknown method: {method}");
                None
            }
        }
    }

    fn initialize(&self, id: &Value, _params: Option<&Value>) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "capabilities": {
                    "textDocumentSync": 1,
                    "codeActionProvider": {
                        "codeActionKinds": ["refactor.rewrite"]
                    }
                },
                "serverInfo": {
                    "name": "ascii-banner-lsp",
                    "version": "0.1.0"
                }
            }
        })
    }

    fn did_open(&mut self, params: &Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap().to_string();
        let text = params["textDocument"]["text"].as_str().unwrap().to_string();
        eprintln!("[ascii-banner-lsp] opened: {uri}");
        self.docs.open(uri, text);
    }

    fn did_change(&mut self, params: &Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap();
        if let Some(changes) = params["contentChanges"].as_array() {
            if let Some(change) = changes.last() {
                if let Some(text) = change["text"].as_str() {
                    self.docs.change(uri, text.to_string());
                    return;
                }
            }
        }
    }

    fn did_close(&mut self, params: &Value) {
        let uri = params["textDocument"]["uri"].as_str().unwrap();
        eprintln!("[ascii-banner-lsp] closed: {uri}");
        self.docs.close(uri);
    }

    fn code_action(&self, id: &Value, params: &Value) -> Value {
        let uri = match params["textDocument"]["uri"].as_str() {
            Some(u) => u.to_string(),
            None => {
                return json!({"jsonrpc": "2.0", "id": id, "result": null});
            }
        };

        let range = &params["range"];
        let start_line = range["start"]["line"].as_u64().unwrap_or(0) as usize;
        let start_char = range["start"]["character"].as_u64().unwrap_or(0) as usize;
        let end_line = range["end"]["line"].as_u64().unwrap_or(0) as usize;
        let end_char = range["end"]["character"].as_u64().unwrap_or(0) as usize;

        if start_line == end_line && start_char == end_char {
            return json!({"jsonrpc": "2.0", "id": id, "result": null});
        }

        let doc = match self.docs.get(&uri) {
            Some(d) => d,
            None => {
                return json!({"jsonrpc": "2.0", "id": id, "result": null});
            }
        };

        let selected_text = match extract_range(doc, start_line, start_char, end_line, end_char) {
            Some(t) => t,
            None => {
                return json!({"jsonrpc": "2.0", "id": id, "result": null});
            }
        };

        if selected_text.trim().is_empty() {
            return json!({"jsonrpc": "2.0", "id": id, "result": null});
        }

        let fonts: Vec<(&str, fn(&str) -> Option<String>)> = vec![
            ("Tiny caps (Cybermedium)", |t| FIGlet::from_content(include_str!("../fonts/Cybermedium.flf")).ok()?.convert(t).map(|f| f.to_string())),
            ("Small caps (4Max)", |t| FIGlet::from_content(include_str!("../fonts/4Max.flf")).ok()?.convert(t).map(|f| f.to_string())),
            ("Normal (Tubes)", |t| FIGlet::from_content(include_str!("../fonts/Tubes-Regular.flf")).ok()?.convert(t).map(|f| f.to_string())),
            ("Large caps (Basic)", |t| FIGlet::from_content(include_str!("../fonts/Basic.flf")).ok()?.convert(t).map(|f| f.to_string())),
            ("Huge (Georgia11)", |t| FIGlet::from_content(include_str!("../fonts/Georgia11.flf")).ok()?.convert(t).map(|f| f.to_string())),
        ];

        let range_json = json!({
            "start": { "line": start_line, "character": start_char },
            "end": { "line": end_line, "character": end_char }
        });

        let lines: Vec<&str> = selected_text.lines().collect();

        let mut actions = Vec::new();

        for (name, render) in &fonts {
            let mut banner_parts: Vec<String> = Vec::new();
            for line in &lines {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(rendered) = render(trimmed) {
                    banner_parts.push(rendered.trim_end_matches('\n').to_string());
                }
            }

            if banner_parts.is_empty() {
                continue;
            }

            let banner_text = banner_parts.join("\n\n");

            actions.push(json!({
                "title": format!("Convert to ASCII Banner: {name}"),
                "kind": "refactor.rewrite",
                "edit": {
                    "changes": {
                        uri.clone(): [{
                            "range": range_json,
                            "newText": banner_text
                        }]
                    }
                }
            }));
        }

        json!({"jsonrpc": "2.0", "id": id, "result": actions})
    }
}

fn extract_range(text: &str, start_line: usize, start_char: usize, end_line: usize, end_char: usize) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let clamped_end_line = end_line.min(lines.len().saturating_sub(1));

    if start_line == clamped_end_line {
        let line = lines[start_line];
        let s = start_char.min(line.len());
        let e = end_char.min(line.len());
        Some(line[s..e].to_string())
    } else {
        let mut result = String::new();

        let first = lines[start_line];
        let s = start_char.min(first.len());
        result.push_str(&first[s..]);
        result.push('\n');

        for line in &lines[start_line + 1..clamped_end_line] {
            result.push_str(line);
            result.push('\n');
        }

        let last = lines[clamped_end_line];
        let e = end_char.min(last.len());
        result.push_str(&last[..e]);

        Some(result)
    }
}

fn read_message() -> Option<Value> {
    let stdin = io::stdin();
    let mut locked = stdin.lock();
    let mut content_length: Option<usize> = None;
    let mut line_buf = String::new();

    loop {
        line_buf.clear();
        if locked.read_line(&mut line_buf).ok()? == 0 {
            return None;
        }
        let trimmed = line_buf.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(len_str.parse().ok()?);
        }
    }

    let len = content_length?;
    let mut buffer = vec![0u8; len];
    locked.read_exact(&mut buffer).ok()?;
    let body = str::from_utf8(&buffer).ok()?;
    serde_json::from_str(body).ok()
}

fn send_message(response: &Value) {
    let body = serde_json::to_string(response).unwrap();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "Content-Length: {}\r\n\r\n{}", body.len(), body).unwrap();
    handle.flush().unwrap();
}

fn main() {
    eprintln!("[ascii-banner-lsp] starting...");
    let mut server = LspServer::new();

    while let Some(msg) = read_message() {
        if let Some(response) = server.handle_message(&msg) {
            send_message(&response);
        }
    }

    eprintln!("[ascii-banner-lsp] exiting");
}
