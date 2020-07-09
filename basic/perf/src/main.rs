use std::env;
use std::io;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::time::SystemTime;

#[derive(Debug)]
struct Args {
    count: i32,
    quiet: bool,
    qbasic: bool,
}

fn copy_env(key: &str) -> String {
    format!("{}={}", key, env::var(key).unwrap_or_default())
}

fn copy_basic_mode() -> String {
    copy_env("BLR_BASIC_MODE")
}

impl Args {
    fn parse(&mut self) {
        let mut iterator = env::args().skip(1);
        loop {
            let next = iterator.next();
            match next {
                Some(value) => {
                    if value == "--count" {
                        let next2 = iterator.next();
                        self.count = match next2 {
                            Some(x) => x.parse().unwrap(),
                            None => panic!("--count requires an argument"),
                        };
                    } else if value == "--quiet" {
                        self.quiet = true;
                    } else {
                        panic!(format!("Unexpected parameter {}", value));
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn run_standalone(args: &Args) {
    let program = if args.qbasic { "./src/HELLOQB.BAS" } else { "./src/HELLO.BAS" };
    let output = Command::new("./basic-launcher-rust/target/release/basic-launcher-rust.exe")
        .args(&[program])
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(0));
    }
}

fn progress(n: i32, args: &Args) {
    if args.quiet {
        print!(".");
        io::stdout().flush().unwrap();
    } else {
        println!("{}", n);
    }
}

fn dos_experiment(args: &Args) -> f64 {
    let start = now();
    println!("Running DOS experiment");
    for n in 1..args.count + 1 {
        progress(n, args);
        run_standalone(args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

/// Gets the current directory, converting it to a path that Docker understands as a volume.
fn current_dir_as_msys_path() -> String {
    let original = format!("{}", env::current_dir().unwrap().display());
    original.replace("C:\\", "/c/").replace("\\", "/")
}

fn build_image(args: &Args) {
    println!("Building Docker image");
    let output = Command::new("docker")
        .args(&["build", "-t", "basic", "-f", "Dockerfile.standalone", "."])
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not build docker image");
    if !output.status.success() {
        eprintln!("Could not build docker image");
        exit(output.status.code().unwrap_or(1));
    }
}

fn run_docker_outside(args: &Args) {
    let bin_volume_spec = format!("{}/bin:/basic/bin", current_dir_as_msys_path());
    let src_volume_spec = format!("{}/src:/basic/src", current_dir_as_msys_path());
    let program = if args.qbasic { "HELLOQB.BAS" } else { "HELLO.BAS" };
    let basic_mode = copy_basic_mode();
    let run_args = vec![
        "run",
        "--rm",
        "-v",
        &bin_volume_spec,
        "-v",
        &src_volume_spec,
        "-e",
        &basic_mode,
        "basic",
        program,
    ];
    let output = Command::new("docker")
        .args(run_args)
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}

fn docker_outside_experiment(args: &Args) -> f64 {
    build_image(args);
    let start = now();
    println!("Running Docker (outside) experiment");
    for n in 1..args.count + 1 {
        progress(n, args);
        run_docker_outside(args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

fn docker_inside_experiment(args: &Args) -> f64 {
    build_image(args);
    let start = now();
    println!("Running Docker (inside) experiment");
    let bin_volume_spec = format!("{}/bin:/basic/bin", current_dir_as_msys_path());
    let src_volume_spec = format!("{}/src:/basic/src", current_dir_as_msys_path());
    let perf_volume_spec = format!("{}/perf:/usr/local/perf/bin:ro", current_dir_as_msys_path());
    let fmt_count = args.count.to_string();
    let basic_mode = copy_basic_mode();

    let mut run_args = vec![
        "run",
        "--rm",
        "-v",
        &bin_volume_spec,
        "-v",
        &src_volume_spec,
        "-v",
        &perf_volume_spec,
        "-e",
        &basic_mode,
        "--entrypoint",
        "bash",
        "basic",
        "/usr/local/perf/bin/perf-inside.sh",
        &fmt_count,
    ];
    if args.quiet {
        run_args.push("--quiet");
    }
    let output = Command::new("docker")
        .args(run_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    average
}

fn build_httpd_image(args: &Args) {
    println!("Building Docker HTTPD image");
    let output = Command::new("docker")
        .args(&[
            "build",
            "-t",
            "basic-httpd",
            "-f",
            "Dockerfile.httpd",
            ".",
        ])
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not build docker image");
    if !output.status.success() {
        eprintln!("Could not build docker image");
        exit(output.status.code().unwrap_or(1));
    }
}

fn start_httpd(args: &Args) {
    println!("Starting HTTPD");
    let bin_volume_spec = format!("{}/bin:/basic/bin", current_dir_as_msys_path());
    let src_volume_spec = if args.qbasic {
        format!("{}/rest-qb:/basic/src", current_dir_as_msys_path())
    } else {
        format!("{}/rest:/basic/src", current_dir_as_msys_path())
    };
    let basic_mode = copy_basic_mode();

    let run_args = vec![
        "run",
        "--rm",
        "-d",
        "--name",
        "basic-httpd",
        "-p",
        "8080:80",
        "-e",
        &basic_mode,
        "-v",
        &bin_volume_spec,
        "-v",
        &src_volume_spec,
        "basic-httpd",
    ];
    let output = Command::new("docker")
        .args(run_args)
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not start docker container");
    if !output.status.success() {
        eprintln!("Could not start docker container");
        exit(output.status.code().unwrap_or(1));
    }
}

fn stop_httpd(args: &Args) {
    println!("Stopping HTTPD");
    let output = Command::new("docker")
        .args(&["stop", "basic-httpd"])
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not stop docker container");
    if !output.status.success() {
        eprintln!("Could not stop docker container");
        exit(output.status.code().unwrap_or(1));
    }
}

fn run_curl(i: i32, args: &Args) {
    let payload = format!("hello {}", i);
    let mut run_args = vec![
        "-f",
        "--data",
        &payload,
        "-H",
        "Content-Type: text/plain",
        "http://localhost:8080/api/todo",
    ];

    if args.quiet {
        run_args.insert(0, "--silent");
    }
    let output = Command::new("curl")
        .args(run_args)
        .stdout(if args.quiet {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}

fn apache_experiment(args: &Args) -> f64 {
    build_httpd_image(args);
    start_httpd(args);
    let start = now();
    println!("Running Apache experiment");
    for n in 1..args.count + 1 {
        progress(n, args);
        run_curl(n, args);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (args.count as f64);
    println!("average {}", average);
    stop_httpd(args);
    average
}

fn main() {
    let mut args = Args {
        count: 100,
        quiet: false,
        qbasic: env::var("BLR_BASIC_MODE").unwrap_or_default() == "qbasic"
    };
    args.parse();
    let dos_average = dos_experiment(&args);
    let docker_outside_average = docker_outside_experiment(&args);
    let docker_inside_average = docker_inside_experiment(&args);
    let apache_average = apache_experiment(&args);
    println!("Summary:");
    println!("| DOS              | {} |", dos_average);
    println!("| Docker (outside) | {} |", docker_outside_average);
    println!("| Docker (inside)  | {} |", docker_inside_average);
    println!("| Apache           | {} |", apache_average);
}
