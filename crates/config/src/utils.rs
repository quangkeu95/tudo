use std::path::{Path, PathBuf};

use crate::Config;

/// Returns the path of the top-level directory of the working git tree. If there is no working
/// tree, an error is returned.
pub fn find_git_root_path(relative_to: impl AsRef<Path>) -> eyre::Result<PathBuf> {
    let path = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(relative_to.as_ref())
        .output()?
        .stdout;
    let path = std::str::from_utf8(&path)?.trim_end_matches('\n');
    Ok(PathBuf::from(path))
}

/// Returns the root path to set for the project root
///
/// traverse the dir tree up and look for a `tudo.toml` file starting at the given path or cwd,
/// but only until the root dir of the current repo so that
///
/// ```text
/// -- tudo.toml
///
/// -- repo
///   |__ .git
///   |__sub
///      |__ [given_path | cwd]
/// ```
/// will still detect `repo` as root
pub fn find_project_root_path(path: Option<&PathBuf>) -> std::io::Result<PathBuf> {
    let cwd = &std::env::current_dir()?;
    let cwd = path.unwrap_or(cwd);
    let boundary = find_git_root_path(cwd)
        .ok()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| cwd.clone());
    let mut cwd = cwd.as_path();
    // traverse as long as we're in the current git repo cwd
    while cwd.starts_with(&boundary) {
        let file_path = cwd.join(Config::FILE_NAME);
        if file_path.is_file() {
            return Ok(cwd.to_path_buf());
        }
        if let Some(parent) = cwd.parent() {
            cwd = parent;
        } else {
            break;
        }
    }
    // no foundry.toml found
    Ok(boundary)
}
