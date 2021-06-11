use std::{env, path::PathBuf};
use uuid::Uuid;
use rand::Rng;


pub fn clean() {
    reset_container();
    create_container();
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
fn reset_container() {
    
}

// TODO
fn create_container() {
}
