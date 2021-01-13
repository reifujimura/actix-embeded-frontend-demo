use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::str;

fn execute(command: &str, args: &[&str], cwd: &str) {
    let cmd = Command::new(command)
        .current_dir(cwd)
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .expect(format!("Command \"{}\" launch failed.", command).as_str());
    let output = cmd.wait_with_output().expect("Command failed.");
    if !output.status.success() {
        for line in str::from_utf8(output.stderr.as_slice())
            .unwrap()
            .split("\n")
        {
            println!("cargo:error={}", line);
        }
    }
}

fn main() {
    execute("yarn", vec![].as_slice(), "./frontend");
    execute("yarn", vec!["build"].as_slice(), "./frontend");
    execute(
        "zip",
        vec!["-r", "build.zip", "."].as_slice(),
        "./frontend/build",
    );
}
