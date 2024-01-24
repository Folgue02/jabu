use std::{
    process::{Command, ExitStatus}, ffi::OsStr,
};

pub fn exec_cmd(cmd_bin: &str, args: Vec<String>) -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new(cmd_bin);

    println!("CMD: ==> [{:?} {:?}]", &cmd_bin, &args);
    cmd.args(args);
    cmd.spawn()?;
    cmd.status()
}
