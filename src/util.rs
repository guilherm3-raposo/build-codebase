use std::{
    fs,
    io::{BufRead, BufReader},
    sync::atomic::Ordering,
    thread,
};

use run_script::ScriptOptions;

use crate::{BUILD_IN_PROGRESS, CONFIG};

pub fn build(project: String) {
    let p = format!("builders/{}.sh", project);
    let options = ScriptOptions {
        runner: Some(String::from("bash")),
        env_vars: None,
        exit_on_error: true,
        input_redirection: run_script::types::IoOptions::Inherit,
        output_redirection: run_script::types::IoOptions::Inherit,
        print_commands: true,
        runner_args: None,
        working_directory: None,
    };

    match fs::read_to_string(p) {
        Ok(content) => match run_script::spawn_script!(content, &options) {
            Ok(mut child) => {
                thread::spawn(move || {
                    let _ = child.wait();
                    match child.stderr.take() {
                        Some(stderr) => {
                            let mut buf = BufReader::new(stderr);
                            let mut err = String::new();
                            while let Ok(n) = buf.read_line(&mut err) {
                                if n > 0 {
                                    println!("{}", err);
                                } else {
                                    break;
                                }
                            }
                        }
                        None => (),
                    };
                    match child.stdout.take() {
                        Some(stdout) => {
                            let mut buf = BufReader::new(stdout);
                            let mut out = String::new();
                            while let Ok(n) = buf.read_line(&mut out) {
                                if n > 0 {
                                    println!("{}", out);
                                } else {
                                    break;
                                }
                            }
                        }
                        None => (),
                    };
                    unlock_build();
                });
            }
            Err(_) => {
                unlock_build();
            }
        },
        Err(_) => {
            unlock_build();
        }
    }
}

pub fn lock_build() -> bool {
    let res = BUILD_IN_PROGRESS.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire);

    res.is_err() || res.err().is_some()
}

pub fn unlock_build() -> bool {
    let res = BUILD_IN_PROGRESS.compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire);

    res.is_err() || res.err().is_some()
}

pub fn deploy() -> bool {
    lock_build();

    match fs::read_to_string(CONFIG.deploy_script.clone()) {
        Ok(content) => match run_script::spawn_script!(content) {
            Ok(mut child) => {
                let res = child.wait();
                unlock_build();
                match res {
                    Ok(status) => status.code().unwrap_or(1) == 0,
                    Err(_) => false,
                }
            }
            Err(_) => {
                unlock_build();
                false
            }
        },
        Err(_) => {
            unlock_build();
            false
        }
    }
}
