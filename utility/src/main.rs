extern crate clap;
extern crate shellexpand;

use clap::{Arg, App, SubCommand};
use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn docker_name(pwnable_name: String) -> String {
    format!("pwndock_{}", pwnable_name)
}

fn canonicalize(path: &str) -> PathBuf {
    fs::canonicalize(shellexpand::tilde(path).into_owned()).unwrap()
}

fn main() {
    let matches = App::new("Pwndock")
        .about("Manages docker containers which simplify the debugging of binaries using a specific libc")
        .subcommand(SubCommand::with_name("start")
            .about("Starts a docker container")
            .arg(Arg::with_name("pwnable")
                .required(true)
                .index(1)
                .value_name("FILE"))
            .arg(Arg::with_name("libc")
                .short("l")
                .takes_value(true)
                .value_name("FILE")
                .default_value("/usr/lib/libc.so"))
            .arg(Arg::with_name("work-dir")
                .short("w")
                .takes_value(true)
                .value_name("DIR")
                .default_value("."))
            .arg(Arg::with_name("socket-dir")
                .short("s")
                .takes_value(true)
                .value_name("DIR")
                .default_value("sockets"))
            .arg(Arg::with_name("rr-dir")
                .short("r")
                .takes_value(true)
                .value_name("DIR")
                .default_value("~/.local/share/rr")))
        .subcommand(SubCommand::with_name("stop")
            .about("Stops the docker container and cleans up")
            .arg(Arg::with_name("pwnable")
                .required(true)
                .index(1)
                .value_name("FILE"))
            .arg(Arg::with_name("socket-dir")
                .short("s")
                .takes_value(true)
                .value_name("DIR")
                .default_value("sockets")))
        .get_matches();
    // docker run -v "/home/nico/git/pwndock/docker/sockets:/home/pwndock/sockets" -v "/home/nico/git/pwndock/docker/pwnable:/home/pwndock/pwnable" -v "/home/nico/.local/share/rr:/home/pwndock/traces" -v "/home/nico/git/pwndock/docker:/home/pwndock/workdir" --cap-add=SYS_PTRACE --security-opt seccomp=unconfined pwndock
    if let Some(matches) = matches.subcommand_matches("start") {
        fs::create_dir_all(matches.value_of("work-dir").unwrap()).unwrap();
        fs::create_dir_all(matches.value_of("socket-dir").unwrap()).unwrap();
        fs::create_dir_all(matches.value_of("rr-dir").unwrap()).unwrap();
        
        let workdir = canonicalize(matches.value_of("work-dir").unwrap());
        let socketdir = canonicalize(matches.value_of("socket-dir").unwrap());
        let rrdir = canonicalize(matches.value_of("rr-dir").unwrap());
        let pwnable = canonicalize(matches.value_of("pwnable").unwrap());
        let libc = canonicalize(matches.value_of("libc").unwrap());
        

        println!("Docker exited with status code {}", Command::new("docker")
            .arg("run")
            .arg("--cap-add=SYS_PTRACE")
            .arg("--security-opt").arg("seccomp=unconfined")
            .arg("-v").arg(format!("{}:/home/pwndock/sockets", socketdir.to_str().unwrap()))
            .arg("-v").arg(format!("{}:/home/pwndock/pwnable", pwnable.to_str().unwrap()))
            .arg("-v").arg(format!("{}:/home/pwndock/workdir", workdir.to_str().unwrap()))
            .arg("-v").arg(format!("{}:/home/pwndock/traces", rrdir.to_str().unwrap()))
            .arg("-v").arg(format!("{}:/usr/lib/libc.so", libc.to_str().unwrap()))
            .arg("--name").arg(docker_name(String::from(pwnable.file_name().unwrap().to_str().unwrap())))
            .arg("-d")
            .arg("--rm")
            .arg("pwndock")
            .status()
            .expect("Docker run execution failed"));
    }

    if let Some(matches) = matches.subcommand_matches("stop") {
        let socketdir = canonicalize(matches.value_of("socket-dir").unwrap());
        let pwnable = canonicalize(matches.value_of("pwnable").unwrap_or("sockets"));
        println!("Docker exited with {}", Command::new("docker")
            .arg("stop")
            .arg(docker_name(String::from(pwnable.file_name().unwrap().to_str().unwrap())))
            .status()
            .expect("Docker run execution failed"));
    }

}
