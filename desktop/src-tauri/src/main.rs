use std::{env, path::PathBuf, process::Command};
use tauri::{WebviewUrl, WebviewWindowBuilder};

const INSTANCE_ID: &str = "desktop";
const LOCAL_URL: &str = "http://127.0.0.1:3100";

fn paperclip_command() -> Command {
    let binary = env::var("PAPERCLIP_DESKTOP_CLI").unwrap_or_else(|_| {
        let managed_shim = env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".local/bin/paperclipai"));
        managed_shim
            .filter(|path| path.is_file())
            .unwrap_or_else(|| PathBuf::from("paperclipai"))
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| "paperclipai".into())
    });
    let mut command = Command::new(binary);
    command.env("PAPERCLIP_SUBSCRIPTION_ONLY", "1");
    command
}

fn ensure_background_service() {
    // The CLI owns launchd creation and the server lifecycle. A missing CLI or
    // failed start does not prevent the native shell from opening the local URL.
    let _ = paperclip_command()
        .args(["service", "install", "--instance", INSTANCE_ID])
        .output();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            ensure_background_service();
            let url = LOCAL_URL.parse().expect("valid local Paperclip URL");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Paperclip")
                .inner_size(1280.0, 840.0)
                .min_inner_size(960.0, 640.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Paperclip desktop");
}
