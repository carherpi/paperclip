use serde::Deserialize;
use std::{
    env, fs,
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{Duration, Instant},
};
use tauri::{WebviewUrl, WebviewWindowBuilder};

const INSTANCE_ID: &str = "desktop";
const LOCAL_URL: &str = "http://127.0.0.1:3100";
const CLI_BOOTSTRAP_PACKAGE: &str = "paperclipai@latest";
const CLI_REPOSITORY: &str = "carherpi/paperclip";
const CLI_REF: &str = "master";

#[derive(Deserialize)]
struct ManagedInstallManifest {
    source: String,
    repo: Option<String>,
}

#[derive(Debug)]
enum StartupFailure {
    MissingPrerequisites(String),
    CliInstallFailed(String),
    ServiceFailed(String),
    ServiceUnavailable,
}

fn command_path_candidates(name: &str) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(path) = env::var_os("PATH") {
        candidates.extend(env::split_paths(&path).map(|directory| directory.join(name)));
    }
    if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
        candidates.extend([
            home.join(".local/bin").join(name),
            home.join(".npm-global/bin").join(name),
            home.join("Library/pnpm").join(name),
            home.join(".volta/bin").join(name),
            home.join(".fnm/aliases/default/bin").join(name),
            home.join(".asdf/shims").join(name),
        ]);
        let nvm_versions = home.join(".nvm/versions/node");
        if let Ok(entries) = fs::read_dir(nvm_versions) {
            candidates.extend(
                entries
                    .flatten()
                    .map(|entry| entry.path().join("bin").join(name)),
            );
        }
    }
    candidates.extend([
        PathBuf::from("/opt/homebrew/bin").join(name),
        PathBuf::from("/usr/local/bin").join(name),
    ]);
    candidates
}

fn executable_path(name: &str) -> Option<PathBuf> {
    command_path_candidates(name)
        .into_iter()
        .find(|candidate| candidate.is_file())
}

fn managed_desktop_cli() -> Option<PathBuf> {
    let home = env::var_os("HOME").map(PathBuf::from)?;
    let manifest_path = home.join(".paperclip/cli/install.json");
    let manifest: ManagedInstallManifest =
        serde_json::from_slice(&fs::read(manifest_path).ok()?).ok()?;
    if manifest.source != "git" || manifest.repo.as_deref() != Some(CLI_REPOSITORY) {
        return None;
    }
    let cli = home.join(".local/bin/paperclipai");
    cli.is_file().then_some(cli)
}

fn command_with_desktop_environment(binary: impl AsRef<Path>) -> Command {
    let mut command = Command::new(binary.as_ref());
    command.env("PAPERCLIP_SUBSCRIPTION_ONLY", "1");
    command
}

fn paperclip_cli() -> Result<PathBuf, StartupFailure> {
    // This override is for source development only. A packaged app otherwise
    // uses a normal global or managed CLI install, never a repository shim.
    if let Some(override_path) = env::var_os("PAPERCLIP_DESKTOP_CLI").map(PathBuf::from) {
        if override_path.is_file() {
            return Ok(override_path);
        }
        return Err(StartupFailure::MissingPrerequisites(format!(
            "The development CLI override does not exist: {}",
            override_path.display()
        )));
    }
    if let Some(cli) = managed_desktop_cli() {
        return Ok(cli);
    }

    let npx = executable_path("npx").ok_or_else(|| StartupFailure::MissingPrerequisites(
        "Paperclip needs Node.js 24.11 or newer and npm/npx. Install the current Node.js LTS, then open Paperclip again.".into(),
    ))?;
    let output = command_with_desktop_environment(npx)
        .args([
            "--yes",
            "--package",
            CLI_BOOTSTRAP_PACKAGE,
            "paperclipai",
            "install",
            "--yes",
            "--repo",
            CLI_REPOSITORY,
            "--ref",
            CLI_REF,
        ])
        .output()
        .map_err(|error| StartupFailure::CliInstallFailed(error.to_string()))?;
    if !output.status.success() {
        return Err(StartupFailure::CliInstallFailed(command_error(&output)));
    }
    managed_desktop_cli().ok_or_else(|| StartupFailure::CliInstallFailed(
        "The Paperclip installer completed but the Agentic Paperclip CLI was not available. Run the recovery command shown below and reopen Paperclip.".into(),
    ))
}

fn command_error(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let detail = if stderr.is_empty() { stdout } else { stderr };
    if detail.is_empty() {
        format!("command exited with {}", output.status)
    } else {
        detail
    }
}

fn ensure_background_service(cli: &Path) -> Result<(), StartupFailure> {
    let output = command_with_desktop_environment(cli)
        .args(["service", "install", "--instance", INSTANCE_ID])
        .output()
        .map_err(|error| StartupFailure::ServiceFailed(error.to_string()))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(StartupFailure::ServiceFailed(command_error(&output)))
    }
}

fn wait_for_local_service() -> bool {
    let address: SocketAddr = "127.0.0.1:3100"
        .parse()
        .expect("valid local service address");
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if TcpStream::connect_timeout(&address, Duration::from_millis(500)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    false
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

fn recovery_url(failure: &StartupFailure) -> WebviewUrl {
    let reason = match failure {
        StartupFailure::MissingPrerequisites(reason) => format!("Prerequisite missing: {reason}"),
        StartupFailure::CliInstallFailed(reason) => {
            format!("Unable to install the Paperclip CLI: {reason}")
        }
        StartupFailure::ServiceFailed(reason) => {
            format!("Unable to start the desktop service: {reason}")
        }
        StartupFailure::ServiceUnavailable => {
            "The Paperclip desktop service did not become ready within 30 seconds.".into()
        }
    };
    WebviewUrl::App(format!("recovery.html?reason={}", percent_encode(&reason)).into())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let url = match paperclip_cli()
                .and_then(|cli| ensure_background_service(&cli))
                .and_then(|_| {
                    if wait_for_local_service() {
                        Ok(())
                    } else {
                        Err(StartupFailure::ServiceUnavailable)
                    }
                }) {
                Ok(()) => {
                    WebviewUrl::External(LOCAL_URL.parse().expect("valid local Paperclip URL"))
                }
                Err(failure) => recovery_url(&failure),
            };
            WebviewWindowBuilder::new(app, "main", url)
                .title("Paperclip")
                .inner_size(1280.0, 840.0)
                .min_inner_size(960.0, 640.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Paperclip desktop");
}

#[cfg(test)]
mod tests {
    use super::percent_encode;

    #[test]
    fn percent_encodes_recovery_messages_for_an_app_url() {
        assert_eq!(
            percent_encode("service failed: 127.0.0.1"),
            "service%20failed%3A%20127.0.0.1"
        );
    }
}
