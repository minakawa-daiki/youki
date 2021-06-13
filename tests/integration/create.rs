use std::path::PathBuf;
use std::process::{Command,Stdio};

// TODO return to Result Structure
// ADD args
pub fn exec(project_path: &PathBuf, id: &str) -> bool {
    let status = Command::new(project_path.join(PathBuf::from("youki")))
        .stdout(Stdio::null())
        .arg("create")
        .arg(id)
        .arg("--bundle")
        .arg("tutorial")
        .status()
        .expect("failed to execute process");
    return status.success();
}
