use std::path::PathBuf;
use std::process::{Command,Stdio};

// TODO return to Result Structure
pub fn exec(project_path: &PathBuf, id: &str) -> bool {
    let status = Command::new(project_path.join(PathBuf::from("youki")))
        .stdout(Stdio::null())
        .arg("state")
        .arg(id)
        .status()
        .expect("failed to execute process");
    return status.success();
}
