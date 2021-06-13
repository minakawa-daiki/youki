use std::{env, path::PathBuf,fs};
use uuid::Uuid;
use rand::Rng;
use std::process::{Command,Stdio};

pub fn initialize_test(project_path: &PathBuf) {
    prepare_test_workspace(project_path);
}

pub fn clean_test(project_path: &PathBuf) {
    delete_test_workspace(project_path);
}

pub fn create_project_path() -> PathBuf {
    let current_dir_path_result = env::current_dir();
    return match current_dir_path_result {
        Ok(path_buf) => path_buf,
        Err(_) => panic!("directory is not found"),
    };
}

pub fn generate_uuid() -> Uuid {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789abcdefABCDEF";

    let rand_string: String = (0..32)
    .map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    })
    .collect();

    return match Uuid::parse_str(&rand_string) {
        Ok(uuid) => uuid,
        Err(e) => panic!("{}", e),
    }
}

// TODO
fn prepare_test_workspace(project_path: &PathBuf) {
    let _result = fs::create_dir_all(project_path.join("integration-test-ws/rootfs"));
    let _result2 = env::set_current_dir(project_path.join("integration-test-ws"));
    let cmd = Command::new("docker")
        .arg("export")
        .arg("$(docker create busybox)")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    Command::new("tar")
        .stdin(cmd.stdout.unwrap())
        .arg("-C")
        .arg("rootfs")
        .arg("-xvf")
        .arg("-")
        .stdout(Stdio::null())
        .status()
        .expect("failed to execute process");
}


// TODO
fn delete_test_workspace(project_path: &PathBuf) {
    
}
