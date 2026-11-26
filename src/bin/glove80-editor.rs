#[path = "../chord.rs"]
mod chord;
#[path = "../config.rs"]
#[allow(dead_code)]
mod config;

use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    env, fs,
    hash::{DefaultHasher, Hash, Hasher},
    io::Read,
    path::Path,
};
use tiny_http::{Header, Method, Response, Server, StatusCode};
use toml_edit::{value, DocumentMut, Item};

fn revision(source: &str) -> String {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn load(path: &Path) -> Result<Value, String> {
    let source = fs::read_to_string(path).map_err(|e| e.to_string())?;
    config::compile(&source)?;
    let doc: toml::Value = toml::from_str(&source).map_err(|e| e.to_string())?;
    Ok(json!({
        "file": path.file_name().and_then(|name| name.to_str()),
        "revision": revision(&source),
        "bindings": doc["layers"]["special"],
        "actions": doc["actions"], "apps": doc.get("apps")
    }))
}

#[derive(Deserialize)]
struct Update {
    revision: String,
    bindings: BTreeMap<String, String>,
    #[serde(default)]
    actions: BTreeMap<String, Value>,
    #[serde(default)]
    apps: BTreeMap<String, Value>,
}

fn save(path: &Path, update: Update) -> Result<Value, String> {
    let source = fs::read_to_string(path).map_err(|e| e.to_string())?;
    if revision(&source) != update.revision {
        return Err("The file changed on disk. Reload it before saving.".into());
    }
    let mut doc: DocumentMut = source
        .parse()
        .map_err(|e: toml_edit::TomlError| e.to_string())?;
    let bindings = doc["layers"]["special"]
        .as_table_like_mut()
        .ok_or("Missing layers.special table")?;
    let removed: Vec<_> = bindings
        .iter()
        .map(|(key, _)| key.to_owned())
        .filter(|key| !update.bindings.contains_key(key))
        .collect();
    for key in removed {
        bindings.remove(&key);
    }
    for (key, action) in update.bindings {
        match bindings.get_mut(&key).and_then(Item::as_value_mut) {
            Some(current) if current.as_str() == Some(&action) => {}
            Some(current) => {
                let decor = current.decor().clone();
                *current = toml_edit::Value::from(action);
                *current.decor_mut() = decor;
            }
            None => {
                bindings.insert(&key, value(action));
            }
        }
    }
    for (section, entries) in [("actions", update.actions), ("apps", update.apps)] {
        if !entries.is_empty() && doc.get(section).is_none() {
            doc[section] = Item::Table(toml_edit::Table::new());
        }
        for (name, definition) in entries {
            let text = toml_edit::ser::to_string(&definition).map_err(|e| e.to_string())?;
            let entry: DocumentMut = text
                .parse()
                .map_err(|e: toml_edit::TomlError| e.to_string())?;
            let destination = &mut doc[section][&name];
            let table = destination.as_table().cloned();
            let mut next = entry.as_table().clone();
            if let Some(mut previous) = table {
                previous.retain(|key, _| next.contains_key(key));
                for (key, item) in next.iter_mut() {
                    if previous.get(key.get()).map(ToString::to_string) != Some(item.to_string()) {
                        previous.insert(key.get(), item.clone());
                    }
                }
                next = previous;
            }
            *destination = Item::Table(next);
        }
    }
    let output = doc.to_string();
    config::compile(&output)?;
    let temporary = path.with_extension(format!("editor-{}.tmp", std::process::id()));
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    let write = (|| {
        use std::io::Write;
        let mut file = file;
        file.write_all(output.as_bytes())?;
        file.sync_all()
    })();
    if let Err(error) = write.and_then(|_| fs::rename(&temporary, path)) {
        let _ = fs::remove_file(&temporary);
        return Err(error.to_string());
    }
    load(path)
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut args = env::args_os().skip(1);
    let Some(file) = args.next() else {
        return Err("usage: glove80-editor <config.toml> [port]".into());
    };
    let path = fs::canonicalize(file)?;
    let port: u16 = args
        .next()
        .map_or(Ok(8765), |p| p.to_string_lossy().parse())?;
    let server = Server::http(("127.0.0.1", port))?;
    let address = format!("127.0.0.1:{}", server.server_addr().to_ip().unwrap().port());
    println!("Glove80 editor at http://{address}");
    for mut request in server.incoming_requests() {
        let header = |name: &'static str| {
            request
                .headers()
                .iter()
                .find(|h| h.field.equiv(name))
                .map(|h| h.value.as_str())
        };
        let (code, kind, body) = if header("Host") != Some(&address) {
            (403, "text/plain", "Invalid host".into())
        } else {
            match (request.method(), request.url()) {
                (&Method::Get, "/") => {
                    (200, "text/html", include_str!("../../ui/index.html").into())
                }
                (&Method::Get, "/keyboard.json") => (
                    200,
                    "application/json",
                    include_str!("../../ui/keyboard.json").into(),
                ),
                (&Method::Get, "/moonlander.json") => (
                    200,
                    "application/json",
                    include_str!("../../ui/moonlander.json").into(),
                ),
                (&Method::Get, "/legends.js") => (
                    200,
                    "text/javascript",
                    include_str!("../../ui/legends.js").into(),
                ),
                (&Method::Get, "/config") => api(load(&path)),
                (&Method::Post, "/config") if header("X-Glove80-Editor") == Some("1") => {
                    let mut body = String::new();
                    let result = request
                        .as_reader()
                        .take(16_385)
                        .read_to_string(&mut body)
                        .map_err(|e| e.to_string())
                        .and_then(|_| {
                            if body.len() > 16_384 {
                                return Err("Update is too large".into());
                            }
                            let update = serde_json::from_str(&body).map_err(|e| e.to_string())?;
                            save(&path, update)
                        });
                    api(result)
                }
                _ => (404, "text/plain", "Not found".into()),
            }
        };
        let response = Response::from_string(body)
            .with_status_code(StatusCode(code))
            .with_header(
                Header::from_bytes("Content-Type", format!("{kind}; charset=utf-8")).unwrap(),
            )
            .with_header(Header::from_bytes("Cache-Control", "no-store").unwrap());
        let _ = request.respond(response);
    }
    Ok(())
}

fn api(result: Result<Value, String>) -> (u16, &'static str, String) {
    match result {
        Ok(value) => (200, "application/json", value.to_string()),
        Err(error) => (400, "application/json", json!({"error": error}).to_string()),
    }
}
