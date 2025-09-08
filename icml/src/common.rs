use std::process::Command;

pub const CHUNK_SIZE: usize = 2000000usize;

pub fn dfx_call(
    network: &Option<String>,
    canister: &str,
    method: &str,
    args: &Vec<String>,
) -> Result<std::process::Output, String> {
    let mut command = Command::new("dfx");

    command.arg("canister");
    command.arg("call");

    command.arg(canister);
    command.arg(method);

    if let Some(net) = network {
        command.arg("--network");
        command.arg(net);
    }

    for arg in args {
        command.arg(arg);
    }

    command.output().map_err(|e| e.to_string())
}
