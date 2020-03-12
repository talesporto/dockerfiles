use std::env;
use std::io;
use std::io::Write;
use std::process::{exit, Command, Stdio};
use std::time::SystemTime;

#[derive(Debug)]
struct Args {
    count: i32,
    quiet: bool,
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
    let output = Command::new("ruby")
        .args(&["run-dos-box.rb", "HELLO.BAS"])
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
        .args(&["build", "-t", "gwbasic", "-f", "Dockerfile.standalone", "."])
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

fn run_docker_outside(volume_spec: &String, args: &Args) {
    let output = Command::new("docker")
        .args(&["run", "--rm", "-v", volume_spec, "gwbasic", "HELLO.BAS"])
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
    let volume_spec = format!("{}:/basic/src", current_dir_as_msys_path());
    for n in 1..args.count + 1 {
        progress(n, args);
        run_docker_outside(&volume_spec, args);
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
    let volume_spec = format!("{}:/basic/src", current_dir_as_msys_path());
    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "-v",
            &volume_spec,
            "--entrypoint",
            "bash",
            "gwbasic",
            "/basic/src/perf-inside.sh",
            &args.count.to_string(),
        ])
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
            "gwbasic-httpd",
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
    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "-d",
            "--name",
            "gwbasic-httpd",
            "-p",
            "8080:80",
            "-v",
            &format!("{}/rest:/basic/src", current_dir_as_msys_path()),
            "gwbasic-httpd",
        ])
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
        .args(&["stop", "gwbasic-httpd"])
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
    let output = Command::new("curl")
        .args(&[
            "-f",
            "--data",
            &format!("hello {}", i),
            "-H",
            "Content-Type: text/plain",
            "http://localhost:8080/api/todo",
        ])
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
