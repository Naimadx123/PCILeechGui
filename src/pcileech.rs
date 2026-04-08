use std::process::Command;
use std::sync::{Arc, Mutex};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub fn run_pcileech(
    command_args: String,
    output: Arc<Mutex<String>>,
    is_running: Arc<Mutex<bool>>,
) {
    *is_running.lock().unwrap() = true;
    {
        let mut out = output.lock().unwrap();
        out.push_str(&format!(">>> pcileech  {}\n", command_args));
    }

    std::thread::spawn(move || {
        let args: Vec<&str> = command_args.split_whitespace().collect();
        let mut cmd = Command::new("pcileech");
        cmd.args(args);
        #[cfg(windows)]
        {
            cmd.creation_flags(0x08000000);
        }
        let result = cmd.output();

        let mut out = output.lock().unwrap();
        match result {
            Ok(output) => {
                out.push_str(&String::from_utf8_lossy(&output.stdout));
                out.push_str(&String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                out.push_str(&format!("Error: {}\n", e));
            }
        }
        out.push_str("\n--- Finished ---\n");
        *is_running.lock().unwrap() = false;
    });
}
