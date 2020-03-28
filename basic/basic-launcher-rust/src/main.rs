use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

mod batch_file;
mod options;
mod rand_file;
mod temp_files;

use batch_file::create_batch_file;
use temp_files::TempFiles;

fn main() {
    let options = options::parse_options();
    let temp_files = TempFiles::create(&options);
    create_stdin(&options, &temp_files.stdin_file).expect("Could not create stdin");
    create_batch_file(&options, &temp_files).expect("Could not create batch file");
    run_dosbox(&options, &temp_files);
    print_stdout(&temp_files).expect("Could not read stdout");
    if options.cleanup {
        cleanup(&temp_files).expect("Could not cleanup files");
    }
}

fn create_stdin(options: &options::Options, stdin_file: &PathBuf) -> std::io::Result<()> {
    let mut f = File::create(stdin_file)?;
    if options.needs_stdin {
        let stdin = stdin();
        loop {
            let mut line = String::new();
            let num_bytes = stdin.read_line(&mut line)?;
            if num_bytes == 0 {
                break;
            }
            write!(f, "{}\r\n", line.trim_end())?;
        }
    }
    Ok(())
}

fn run_dosbox(options: &options::Options, temp_files: &TempFiles) {
    let mut batch_file = format!("{}", temp_files.batch_file.display());
    let win_prefix = "\\\\?\\";
    if batch_file.starts_with(win_prefix) {
        for _ in 0..win_prefix.len() {
            batch_file.remove(0);
        }
    }

    let log_file = File::create(&temp_files.dosbox_log_file).unwrap();
    let err_file = File::create(&temp_files.dosbox_err_file).unwrap();
    let out = Command::new(&options.dosbox)
        .args(&[&batch_file, "-exit", "-noautoexec", "-conf", &options.dosbox_conf])
        .env("SDL_VIDEODRIVER", "dummy")
        .env("TERM", "dumb")
        .stdout(log_file)
        .stderr(err_file)
        .output()
        .unwrap();

    if !out.status.success() {
        panic!("DOSBox did not return a success error code");
    }
}

fn print_stdout(temp_files: &TempFiles) -> std::io::Result<()> {
    let f = File::open(&temp_files.stdout_file)?;
    let mut reader = BufReader::new(f);
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }

        println!("{}", line.trim_end());
    }
    Ok(())
}

fn cleanup(temp_files: &TempFiles) -> std::io::Result<()> {
    remove_if_exists(&temp_files.batch_file)?;
    remove_if_exists(&temp_files.dosbox_log_file)?;
    remove_if_exists(&temp_files.dosbox_err_file)?;
    remove_if_exists(&temp_files.stdin_file)?;
    remove_if_exists(&temp_files.stdout_file)
}

fn remove_if_exists(p: &PathBuf) -> std::io::Result<()> {
    if p.exists() {
        fs::remove_file(p)
    } else {
        Ok(())
    }
}
