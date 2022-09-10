use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use lapce_plugin::{register_plugin, send_notification, start_lsp, LapcePlugin};

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    system_lsp: bool,
    options: Option<Value>,
}

register_plugin!(State);

impl LapcePlugin for State {
    fn initialize(&mut self, info: serde_json::Value) {
        eprintln!("[lapce-ruby] starting plugin");
        let info = serde_json::from_value::<PluginInfo>(info).unwrap();

        let exec_path = if !info.configuration.system_lsp {
            String::new()
        } else {
            String::from("solargraph")
        };

        eprintln!("[lapce-ruby] exec path: {}", exec_path);

        start_lsp(&exec_path, "ruby", info.configuration.options.clone(), info.configuration.system_lsp);
    }
}
