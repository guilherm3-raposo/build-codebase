use std::fs::{self};

pub fn build(project: String) {
    let p = format!("builders/{}.sh", project);
    match fs::read_to_string(p) {
        Ok(content) => {
            let _child = run_script::spawn_script!(content);
        }
        Err(e) => println!("{e:?}"),
    }
}
