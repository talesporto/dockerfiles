use std::env;
use std::process::Command;
use std::fs::canonicalize;

fn main() {
    let mut output_builder = Command::new("docker");
    let mut seen_v = false;
    for arg in env::args().skip(1) {
        output_builder.arg(if seen_v { process_volume(&arg) } else { String::from(&arg) });
        seen_v = arg == "-v" || arg == "--volume";
    }

    let mut output = output_builder.spawn()
        .expect("Could not start docker");
    let result = output.wait().unwrap();
    std::process::exit(result.code().unwrap_or_default());
}

fn process_volume(v: &String) -> String {
    let splits: Vec<&str> = v.rsplitn(2, ":").collect();
    if splits.len() != 2 {
        String::from(v)
    } else {
        let mut host_part = match canonicalize(splits[1]) {
            Ok(x) => format!("{}", x.display()),
            _ => String::from(splits[1])
        };
        // TODO support other drives
        host_part = host_part.replace("\\\\?\\C:\\", "/c/");
        host_part = host_part.replace("\\", "/");
        let guest_part = splits[0];
        let result = format!("{}:{}", host_part, guest_part);
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(1, 1);
    }
}
