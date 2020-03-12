use std::env;
use std::process::{exit, Command, Stdio};
use std::time::SystemTime;

const count: i32 = 100;

fn now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn run_standalone() {
    let output = Command::new("ruby")
        .args(&["run-dos-box.rb", "HELLO.BAS"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(0));
    }
}

fn dos_experiment() -> f64 {
    let start = now();
    println!("Running DOS experiment");
    for n in 1..count + 1 {
        println!("{}", n);
        run_standalone();
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (count as f64);
    println!("average {}", average);
    average
}

/// Gets the current directory, converting it to a path that Docker understands as a volume.
fn current_dir_as_msys_path() -> String {
    let original = format!("{}", env::current_dir().unwrap().display());
    original.replace("C:\\", "/c/").replace("\\", "/")
}

fn build_image() {
    println!("Building Docker image");
    let output = Command::new("docker")
        .args(&["build", "-t", "gwbasic", "-f", "Dockerfile.standalone", "."])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not build docker image");
    if !output.status.success() {
        eprintln!("Could not build docker image");
        exit(output.status.code().unwrap_or(1));
    }
}

fn run_docker_outside(volume_spec: &String) {
    let output = Command::new("docker")
        .args(&["run", "--rm", "-v", volume_spec, "gwbasic", "HELLO.BAS"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}

fn docker_outside_experiment() -> f64 {
    build_image();
    let start = now();
    println!("Running Docker (outside) experiment");
    let volume_spec = format!("{}:/basic/src", current_dir_as_msys_path());
    for n in 1..count + 1 {
        println!("{}", n);
        run_docker_outside(&volume_spec);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (count as f64);
    println!("average {}", average);
    average
}

fn docker_inside_experiment() -> f64 {
    build_image();
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
        ])
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
    let average = ((stop - start) as f64) / (count as f64);
    println!("average {}", average);
    average
}

fn build_httpd_image() {
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
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not build docker image");
    if !output.status.success() {
        eprintln!("Could not build docker image");
        exit(output.status.code().unwrap_or(1));
    }
}

fn start_httpd() {
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
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not start docker container");
    if !output.status.success() {
        eprintln!("Could not start docker container");
        exit(output.status.code().unwrap_or(1));
    }
}

fn stop_httpd() {
    println!("Stopping HTTPD");
    let output = Command::new("docker")
        .args(&["stop", "gwbasic-httpd"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Could not stop docker container");
    if !output.status.success() {
        eprintln!("Could not stop docker container");
        exit(output.status.code().unwrap_or(1));
    }
}

fn run_curl(i: i32) {
    let output = Command::new("curl")
        .args(&[
            "-f",
            "--data",
            &format!("hello {}", i),
            "-H",
            "Content-Type: text/plain",
            "http://localhost:8080/api/todo",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Failed");
        exit(output.status.code().unwrap_or(1));
    }
}

fn apache_experiment() -> f64 {
    build_httpd_image();
    start_httpd();
    let start = now();
    println!("Running Apache experiment");
    for n in 1..count + 1 {
        println!("{}", n);
        run_curl(n);
    }
    let stop = now();
    println!("{}", stop - start);
    let average = ((stop - start) as f64) / (count as f64);
    println!("average {}", average);
    stop_httpd();
    average
}

fn main() {
    let dos_average = dos_experiment();
    let docker_outside_average = docker_outside_experiment();
    let docker_inside_average = docker_inside_experiment();
    let apache_average = apache_experiment();
    println!("Summary:");
    println!("| DOS              | {} |", dos_average);
    println!("| Docker (outside) | {} |", docker_outside_average);
    println!("| Docker (inside)  | {} |", docker_inside_average);
    println!("| Apache           | {} |", apache_average);
}
