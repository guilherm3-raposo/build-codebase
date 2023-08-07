use std::process::Command;

pub fn build(project: String) {
    let _ = Command::new(format!("./builders/{}.sh", project)).output();
}
