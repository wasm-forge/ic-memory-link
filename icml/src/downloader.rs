use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    process::Command,
};

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

pub fn blob_to_vec_u8(ret: &str) -> Vec<u8> {
    // Example input: (blob "\01\02\03")
    if let Some(start) = ret.find('"')
        && let Some(end) = ret.rfind('"')
    {
        let content = &ret[start + 1..end];
        let mut bytes = Vec::new();
        let mut i = 0;
        while i < content.len() {
            if &content[i..i + 1] == "\\" {
                let hex = &content[i + 1..i + 3];
                if let Ok(val) = u8::from_str_radix(hex, 16) {
                    bytes.push(val);
                }
                i += 3;
            } else {
                // Direct characters (rare case)
                bytes.push(content.as_bytes()[i]);
                i += 1;
            }
        }
        return bytes;
    }
    Vec::new()
}

pub fn download_chunks(
    output: &str,
    canister: &str,
    method: &str,
    network: &Option<String>,
) -> Result<(), anyhow::Error> {
    let mut output_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;

    let mut offset: usize = 0;

    loop {
        let param = format!("({offset}: nat64)");
        let params: Vec<String> = vec![param];

        let output =
            dfx_call(network, canister, method, &params).map_err(|e| anyhow::anyhow!(e))?;

        let block_str = String::from_utf8(output.stdout)?;
        let blob = blob_to_vec_u8(&block_str);

        if blob.is_empty() {
            println!("Download finished");
            break;
        }

        // move file cursor
        output_file.seek(SeekFrom::Start(offset as u64))?;
        output_file.write_all(&blob)?;

        offset += blob.len();

        println!("Downloaded size: {offset}");
    }

    Ok(())
}
