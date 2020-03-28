use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_DOSBOX: &str = "C:\\Program Files (x86)\\DOSBox-0.74\\DOSBox.exe";
const DEFAULT_DOSBOX_CONF: &str = "dosbox.conf";

// Environment variable names are prefixed with BLR for basic-launcher-rust
const EV_DOSBOX: &str = "BLR_DOSBOX";
const EV_GWBASIC: &str = "BLR_GWBASIC";
const EV_QBASIC: &str = "BLR_QBASIC";
const EV_NO_CLEANUP: &str = "BLR_NO_CLEANUP";
const EV_BASIC_MODE: &str = "BLR_BASIC_MODE";
const EV_PROGRAM: &str = "BLR_PROGRAM";
const EV_DOSBOX_CONF: &str = "BLR_DOSBOX_CONF";

#[derive(Debug)]
pub enum BasicMode {
    GWBasic,
    QBasic,
}

#[derive(Debug)]
pub struct Options {
    pub dosbox: String,
    pub dosbox_conf: String,
    pub basic: PathBuf,
    pub mode: BasicMode,
    pub needs_stdin: bool,
    pub program: PathBuf,
    pub cleanup: bool,
}

pub fn parse_options() -> Options {
    let args: Vec<String> = env::args().skip(1).collect();
    let x = parse_basic();
    Options {
        dosbox: parse_dosbox(),
        dosbox_conf: parse_dosbox_conf(),
        basic: x.0,
        mode: x.1,
        needs_stdin: parse_needs_stdin(&args),
        program: parse_program(&args),
        cleanup: parse_cleanup(),
    }
}

fn parse_dosbox() -> String {
    let v = get_redirect_env(EV_DOSBOX);
    if v.is_empty() {
        DEFAULT_DOSBOX.to_string()
    } else {
        v
    }
}

fn parse_dosbox_conf() -> String {
    let v = get_redirect_env(EV_DOSBOX_CONF);
    if v.is_empty() {
        DEFAULT_DOSBOX_CONF.to_string()
    } else {
        v
    }
}

fn parse_basic() -> (PathBuf, BasicMode) {
    let non_canonic =
        parse_non_canonic().expect("Please specify the location of the basic interpreter");
    let exe = match fs::canonicalize(&non_canonic.0) {
        Ok(p) => p,
        Err(e) => panic!("Could not find interpreter {}: {}", &non_canonic.0, e),
    };

    (exe, non_canonic.1)
}

fn parse_non_canonic() -> Option<(String, BasicMode)> {
    if is_explicit_qb() {
        parse_qbasic()
    } else {
        parse_gwbasic().or_else(parse_qbasic)
    }
}

fn is_explicit_qb() -> bool {
    let v = get_redirect_env(EV_BASIC_MODE);
    v == "qbasic"
}

fn parse_gwbasic() -> Option<(String, BasicMode)> {
    let gwbasic = get_redirect_env(EV_GWBASIC);
    if gwbasic.is_empty() {
        None
    } else {
        Some((gwbasic, BasicMode::GWBasic))
    }
}

fn parse_qbasic() -> Option<(String, BasicMode)> {
    let qbasic = get_redirect_env(EV_QBASIC);
    if qbasic.is_empty() {
        None
    } else {
        Some((qbasic, BasicMode::QBasic))
    }
}

fn parse_needs_stdin(args: &Vec<String>) -> bool {
    !env::var("CONTENT_LENGTH").unwrap_or_default().is_empty() || args.contains(&"-i".to_owned())
}

fn parse_program(args: &Vec<String>) -> PathBuf {
    let program: String = if !args.is_empty() {
        args[0].to_string()
    } else {
        get_redirect_env(EV_PROGRAM)
    };
    if program.is_empty() {
        panic!("Please specify the basic program to run");
    }
    match fs::canonicalize(&program) {
        Ok(p) => p,
        Err(e) => panic!("Could not find BASIC file {}: {}", &program, e),
    }
}

fn parse_cleanup() -> bool {
    get_redirect_env(EV_NO_CLEANUP).is_empty()
}

/// Gets the value of an environment variable, taking into account
/// Apache's REDIRECT variable conventions.
///
/// When requesting variable ABC, the function will try to find a
/// redirected variable REDIRECT_ABC as well as its grandparent
/// REDIRECT_REDIRECT_ABC. The highest defined variable wins (even if empty).
fn get_redirect_env(key: &str) -> String {
    if key.is_empty() {
        panic!("Environment variable name was empty");
    }

    _get_redirect_env(key, 0, 2).unwrap_or_default()
}

fn _get_redirect_env(key: &str, depth: u8, max_depth: u8) -> Option<String> {
    if depth < max_depth {
        let parent_key = format!("REDIRECT_{}", key);
        let parent_result = _get_redirect_env(&parent_key, depth + 1, max_depth);
        match parent_result {
            Some(_) => parent_result,
            _ => _env_var_to_option(key),
        }
    } else {
        _env_var_to_option(key)
    }
}

fn _env_var_to_option(key: &str) -> Option<String> {
    let e = env::var(key);
    match e {
        Ok(v) => Some(v),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_get_redirect_env_with_empty_key_should_panic() {
        get_redirect_env("");
    }

    #[test]
    fn test_get_redirect_env_highest_redirect_wins() {
        // arrange
        env::set_var("ABC", "whatever");
        env::set_var("REDIRECT_ABC", "something");
        env::set_var("REDIRECT_REDIRECT_ABC", "winner");

        // act
        let result = get_redirect_env("ABC");

        // cleanup
        env::remove_var("REDIRECT_REDIRECT_ABC");
        env::remove_var("REDIRECT_ABC");
        env::remove_var("ABC");

        // assert
        assert_eq!(result, "winner");
    }

    #[test]
    fn test_get_redirect_env_no_base_variable() {
        // arrange
        env::remove_var("ABC");
        env::set_var("REDIRECT_ABC", "something");
        env::remove_var("REDIRECT_REDIRECT_ABC");

        // act
        let result = get_redirect_env("ABC");

        // cleanup
        env::remove_var("REDIRECT_ABC");

        // assert
        assert_eq!(result, "something");
    }

    #[test]
    fn test_parse_dosbox_without_env() {
        env::remove_var(EV_DOSBOX);
        let dos_box = parse_dosbox();
        assert_eq!(dos_box, DEFAULT_DOSBOX);
    }

    #[test]
    fn test_parse_dosbox_with_env() {
        env::set_var(EV_DOSBOX, "dosbox");
        let dos_box = parse_dosbox();
        env::remove_var(EV_DOSBOX);
        assert_eq!(dos_box, "dosbox");
    }

    #[test]
    fn test_parse_basic_gwbasic() {
        env::set_var(EV_GWBASIC, "..\\bin\\GWBASIC.EXE");
        let b = parse_basic();
        env::remove_var(EV_GWBASIC);
        assert_eq!(
            b.0.display().to_string(),
            "\\\\?\\C:\\Users\\ngeor\\Projects\\github\\dockerfiles\\basic\\bin\\GWBASIC.EXE"
        );
        assert!(match b.1 {
            BasicMode::GWBasic => true,
            _ => false,
        });
    }
}
