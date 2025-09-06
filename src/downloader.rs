/*

use std::process::Command;
pub fn dfx_call(
    network: Option<&str>,

    canister: &str,

    function: &str,

    additional_args: &Vec<&str>,
) -> Result<std::process::Output, String> {
    let mut command = Command::new("dfx");
    command.arg(canister);
    command.arg(function);

    for arg in additional_args {
        command.arg(arg);
    }

    command.output().map_err(|e| e.to_string())
}

pub fn download_chunks(
    file: String,
    canister: String,
    function: String,
    network: Option<String>,
    start_offset: u64,
) {
    let result = dfx_call(None, &canister, &function, &Vec::new());

    println!("{result:?}");
}

*/
