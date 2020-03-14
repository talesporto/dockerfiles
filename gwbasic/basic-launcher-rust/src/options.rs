use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_DOSBOX: &str = "C:\\Program Files (x86)\\DOSBox-0.74\\DOSBox.exe";
const DOSBOX_ENV_VAR: &str = "DOSBOX";

#[derive(Debug)]
pub enum BasicMode {
    GWBasic,
    QBasic,
}

#[derive(Debug)]
pub struct Options {
    pub dosbox: String,
    pub basic: PathBuf,
    pub mode: BasicMode,
    pub needs_stdin: bool,
    pub program: PathBuf,
    pub cleanup: bool,
}

pub fn parse_options() -> Options {
    let x = parse_basic();
    Options {
        dosbox: parse_dosbox(),
        basic: x.0,
        mode: x.1,
        needs_stdin: parse_needs_stdin(),
        program: parse_program(),
        cleanup: parse_cleanup(),
    }
}

fn env_var_as_option(key: &str) -> Option<String> {
    let r = env::var(key);
    match r {
        Ok(v) => {
            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        }
        Err(_) => None,
    }
}

fn parse_dosbox() -> String {
    env_var_as_option(DOSBOX_ENV_VAR).unwrap_or(DEFAULT_DOSBOX.to_string())
}

fn parse_basic() -> (PathBuf, BasicMode) {
    let gwbasic = env::var("GWBASIC").unwrap_or_default();
    let result = if gwbasic.is_empty() {
        parse_qbasic()
    } else {
        (gwbasic, BasicMode::GWBasic)
    };

    let exe = match fs::canonicalize(&result.0) {
        Ok(p) => p,
        Err(e) => panic!("Could not find interpreter {}: {}", &result.0, e),
    };

    (exe, result.1)
}

fn parse_qbasic() -> (String, BasicMode) {
    let qbasic = env::var("QBASIC").unwrap_or_default();
    if qbasic.is_empty() {
        panic!("Please specify the location of the basic interpreter");
    }

    (qbasic, BasicMode::QBasic)
}

fn parse_needs_stdin() -> bool {
    !env::var("CONTENT_LENGTH").unwrap_or_default().is_empty() || env::args().any(|a| a == "-i")
}

fn parse_program() -> PathBuf {
    let v: Vec<String> = env::args().collect();
    let program: String = if v.len() > 1 {
        v[1].to_string()
    } else {
        parse_program_from_query_string(env::var("QUERY_STRING").unwrap_or_default())
    };
    if program.is_empty() {
        panic!("Please specify the basic program to run");
    }
    match fs::canonicalize(&program) {
        Ok(p) => p,
        Err(e) => panic!("Could not find BASIC file {}: {}", &program, e),
    }
}

fn parse_program_from_query_string(query_string: String) -> String {
    let query_parts: Vec<&str> = query_string
        .split("&")
        .filter(|x| x.starts_with("_bas="))
        .collect();
    if query_parts.is_empty() {
        "".to_string()
    } else {
        let kv: Vec<&str> = query_parts[0].split("=").collect();
        if kv.len() == 2 {
            kv[1].to_string()
        } else {
            "".to_string()
        }
    }
}

fn parse_cleanup() -> bool {
    match env_var_as_option("NO_CLEANUP") {
        Some(_) => false,
        None => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dosbox_without_env() {
        env::remove_var(DOSBOX_ENV_VAR);
        let dos_box = parse_dosbox();
        assert_eq!(dos_box, DEFAULT_DOSBOX);
    }

    #[test]
    fn test_dosbox_with_env() {
        env::set_var(DOSBOX_ENV_VAR, "dosbox");
        let dos_box = parse_dosbox();
        assert_eq!(dos_box, "dosbox");
    }

    #[test]
    fn test_parse_basic_gwbasic() {
        env::set_var("GWBASIC", "..\\bin\\GWBASIC.EXE");
        let b = parse_basic();
        assert_eq!(
            b.0.display().to_string(),
            "\\\\?\\C:\\Users\\ngeor\\Projects\\github\\dockerfiles\\gwbasic\\bin\\GWBASIC.EXE"
        );
        assert!(match b.1 {
            BasicMode::GWBasic => true,
            _ => false,
        });
    }

    #[test]
    fn test_parse_program_from_query_string() {
        assert_eq!(parse_program_from_query_string("".to_string()), "");
        assert_eq!(
            parse_program_from_query_string("_bas=PROGRAM.BAS".to_string()),
            "PROGRAM.BAS"
        );
        assert_eq!(
            parse_program_from_query_string("rand=123&_bas=PROGRAM.BAS".to_string()),
            "PROGRAM.BAS"
        );
        assert_eq!(
            parse_program_from_query_string("rand=123&oop=PROGRAM.BAS".to_string()),
            ""
        );
    }
}
