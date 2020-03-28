use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::options::{BasicMode, Options};
use crate::temp_files::TempFiles;

pub fn create_batch_file(options: &Options, temp_files: &TempFiles) -> Result<(), io::Error> {
    let mut f = File::create(&temp_files.batch_file)?;
    copy_env(&mut f)?;
    write!(
        f,
        "SET STDIN={}\r\n",
        from_dos(&temp_files.stdin_file, &temp_files.batch_dir)
    )?;
    write!(f, "C:\r\n")?;
    // CD C:\SRC
    write!(
        f,
        "CD {}\r\n",
        from_dos(
            &options.program.parent().unwrap().to_path_buf(),
            &temp_files.batch_dir
        )
    )?;
    // C:\BIN\GWBASIC.EXE
    write!(f, "{}", from_dos(&options.basic, &temp_files.batch_dir))?;
    write!(
        f,
        "{}",
        match options.mode {
            BasicMode::GWBasic => " ",
            BasicMode::QBasic => " /RUN ",
        }
    )?;
    // PROGRAM.BAS
    write!(
        f,
        "{}",
        options.program.file_name().unwrap().to_str().unwrap()
    )?;
    // <C:\STDIN.TXT
    write!(
        f,
        " <{}",
        from_dos(&temp_files.stdin_file, &temp_files.batch_dir)
    )?;
    // >C:\STDOUT.TXT
    write!(
        f,
        " >{}\r\n",
        from_dos(&temp_files.stdout_file, &temp_files.batch_dir)
    )
}

fn from_dos(f: &PathBuf, batch_dir: &PathBuf) -> String {
    let mut result: String = String::new();
    let mut p: PathBuf = f.to_path_buf();
    while p != *batch_dir {
        if !result.is_empty() {
            result.insert(0, '\\');
        }

        result.insert_str(0, p.file_name().unwrap().to_str().unwrap());
        p = p.parent().unwrap().to_path_buf();
    }

    if !result.starts_with("\\") {
        result.insert(0, '\\');
    }
    result.insert_str(0, "C:");
    result
}

fn copy_env(f: &mut File) -> Result<(), io::Error> {
    for kv in env::vars() {
        if is_valid_env_key(&kv.0) && is_valid_env_value(&kv.1) {
            write!(f, "SET {}={}\r\n", kv.0, kv.1)?;
        }
    }
    Ok(())
}

/// Environment variables that are allowed to appear in the Batch file.
const WHITE_LIST_KEYS: &[&str] = &[
    "CONTENT_TYPE",
    "QUERY_STRING",
    "REQUEST_METHOD",
    "STDIN",
];

fn is_valid_env_key(key: &str) -> bool {
    match WHITE_LIST_KEYS.binary_search(&key) {
        Ok(_) => true,
        _ => false
    }
}

fn is_valid_env_value(val: &str) -> bool {
    !val.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dos_same_level() {
        let f = PathBuf::from("/home/test/PROGRAM.BAS");
        let dir = PathBuf::from("/home/test");
        let dos = from_dos(&f, &dir);
        assert_eq!(dos, "C:\\PROGRAM.BAS");
    }

    #[test]
    fn test_from_dos_one_level() {
        let f = PathBuf::from("/home/test/PROGRAM.BAS");
        let dir = PathBuf::from("/home");
        let dos = from_dos(&f, &dir);
        assert_eq!(dos, "C:\\test\\PROGRAM.BAS");
    }
}
