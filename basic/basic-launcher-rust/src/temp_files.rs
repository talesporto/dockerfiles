use std::path::Path;
use std::path::PathBuf;

use crate::options::Options;
use crate::rand_file::make_unique_random_filename;

#[derive(Debug)]
pub struct TempFiles {
    pub batch_dir: PathBuf,
    pub batch_file: PathBuf,
    pub stdin_file: PathBuf,
    pub stdout_file: PathBuf,
    pub dosbox_log_file: PathBuf,
    pub dosbox_err_file: PathBuf,
}

impl TempFiles {
    pub fn create(options: &Options) -> TempFiles {
        let batch_dir = batch_dir(&options);
        TempFiles {
            batch_dir: batch_dir.to_path_buf(),
            batch_file: make_unique_random_filename(batch_dir, "BAT"),
            stdin_file: make_unique_random_filename(batch_dir, "INP"),
            stdout_file: make_unique_random_filename(batch_dir, "OUT"),
            dosbox_log_file: make_unique_random_filename(batch_dir, "LOG"),
            dosbox_err_file: make_unique_random_filename(batch_dir, "ERR"),
        }
    }
}

fn batch_dir(options: &Options) -> &Path {
    let basic_dir: &Path = options.basic.parent().unwrap();
    let program_dir: &Path = options.program.parent().unwrap();
    common_ancestor(basic_dir, program_dir)
}

fn common_ancestor<'a>(left: &'a Path, right: &'a Path) -> &'a Path {
    if left == right {
        left
    } else {
        if left.to_str().unwrap().len() > right.to_str().unwrap().len() {
            common_ancestor(left.parent().unwrap(), right)
        } else {
            common_ancestor(left, right.parent().unwrap())
        }
    }
}
