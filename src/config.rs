use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    version: u32,
    layers: Layers,
    #[serde(default)]
    apps: BTreeMap<String, App>,
    actions: BTreeMap<String, Definition>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Layers {
    special: BTreeMap<String, String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct App {
    pipe: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
enum Definition {
    #[serde(rename = "app.command")]
    Pipe {
        label: String,
        app: String,
        command: String,
        #[serde(default)]
        args: serde_json::Map<String, Value>,
    },
    #[serde(rename = "process.run")]
    Process {
        label: String,
        exe: String,
        #[serde(default)]
        args: Vec<String>,
        cwd: Option<String>,
    },
    #[serde(rename = "window.focus_or_launch")]
    Window {
        label: String,
        process: String,
        exe: String,
        #[serde(default)]
        args: Vec<String>,
    },
}

#[derive(Clone)]
pub enum Action {
    Pipe(String, Vec<u8>),
    Process(String, Vec<String>, Option<String>),
    Window(String, String, Vec<String>),
}
pub struct Binding {
    pub name: String,
    pub action: Action,
}
pub type Bindings = BTreeMap<i32, Binding>;

// IDs index the firmware fixture in order of unmodified, Ctrl+Shift, then Ctrl+Alt keys.
const SLOTS: &str = "ASDFGHJKLQWERTYUIOPZXCVBNM";
pub fn compile(input: &str) -> Result<Bindings, String> {
    let config: Config = toml::from_str(input).map_err(|e| e.to_string())?;
    if config.version != 1 {
        return Err(format!("unsupported version {}", config.version));
    }
    for (name, app) in &config.apps {
        if app.pipe.trim().is_empty() || app.pipe.contains(['/', '\\', '\0']) {
            return Err(format!("apps.{name}.pipe must be a local pipe name"));
        }
    }
    let mut actions = BTreeMap::new();
    for (name, def) in config.actions {
        let (label, action) = match def {
            Definition::Pipe {
                label,
                app,
                command,
                args,
            } => {
                let endpoint = config
                    .apps
                    .get(&app)
                    .ok_or_else(|| format!("actions.{name}: unknown app {app}"))?;
                if command.trim().is_empty() {
                    return Err(format!("actions.{name}.command is empty"));
                }
                let mut data = serde_json::to_vec(&json!({"command":command,"args":args})).unwrap();
                data.push(b'\n');
                (
                    label,
                    Action::Pipe(format!(r"\\.\pipe\{}", endpoint.pipe), data),
                )
            }
            Definition::Process {
                label,
                exe,
                args,
                cwd,
            } => {
                if exe.trim().is_empty() {
                    return Err(format!("actions.{name}.exe is empty"));
                }
                (label, Action::Process(exe, args, cwd))
            }
            Definition::Window {
                label,
                process,
                exe,
                args,
            } => {
                if process.trim().is_empty() || exe.trim().is_empty() {
                    return Err(format!("actions.{name} requires process and exe"));
                }
                (label, Action::Window(process, exe, args))
            }
        };
        if label.trim().is_empty() {
            return Err(format!("actions.{name}.label is empty"));
        }
        actions.insert(name, action);
    }
    config
        .layers
        .special
        .into_iter()
        .map(|(slot, name)| {
            let id = SLOTS
                .find(&slot)
                .filter(|_| slot.len() == 1)
                .ok_or_else(|| format!("unknown special slot {slot}"))?;
            let action = actions
                .get(&name)
                .ok_or_else(|| format!("layers.special.{slot}: unknown action {name}"))?
                .clone();
            Ok((id as i32, Binding { name, action }))
        })
        .collect()
}
