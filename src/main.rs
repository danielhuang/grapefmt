use std::{
    env::args,
    io::{stdin, Read, Write},
    process::{Command, Stdio},
};

fn main() {
    let config = Command::new("rustfmt")
        .arg("--print-config")
        .arg("current")
        .arg(".")
        .output()
        .unwrap()
        .stdout;

    let config = String::from_utf8(config).unwrap();

    let max_width = config
        .lines()
        .find_map(|x| x.strip_prefix("max_width = "))
        .unwrap();

    let max_width: usize = max_width.parse().unwrap();

    let mut stdin = stdin().lock();
    let mut current_src = String::new();
    stdin.read_to_string(&mut current_src).unwrap();

    current_src = rustfmt(&current_src, max_width);

    if current_src.lines().any(|x| x.len() > max_width) {
        current_src = rustfmt(
            &current_src,
            current_src.lines().map(|x| x.len()).max().unwrap(),
        );
        current_src = rustfmt(&current_src, max_width);
    }

    print!("{}", current_src);
}

fn rustfmt(src: &str, max_width: usize) -> String {
    let mut cmd = Command::new("rustfmt")
        .arg("--config")
        .arg(format!("max_width={max_width}"))
        .arg("--emit")
        .arg("stdout")
        .arg("--edition")
        .arg("2021")
        .args(args().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdin = cmd.stdin.as_mut().unwrap();
    child_stdin.write_all(src.as_bytes()).unwrap();

    let output = cmd.wait_with_output().unwrap();

    String::from_utf8(output.stdout).unwrap()
}
