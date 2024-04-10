use std::{
    process::{Command, ExitStatus}, ffi::OsStr,
};

/// Executes a binary with the given arguments, this function returns a result, if 
/// it's `Err`, it means that the binary couldn't be executed (*i.e. the binary didn't 
/// exist or there are no permissions*). On the other hand, if `Ok` is returned, an
/// `ExitStatus` is given, in the case that there is no code to the exit status, this
/// might mean that the process was interrupted by a signal (*i.e. SIGINT*)
/// 
/// # See
/// - [`std::process::ExitStatus`] (*its platform behaviour*)
/// - [`std::process::Command`] 
pub fn exec_cmd(cmd_bin: &str, args: Vec<String>) -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new(cmd_bin);

    println!("==> [CMD]: [{:?} {:?}]", &cmd_bin, &args);
    cmd.args(args);
    cmd.status()
}
