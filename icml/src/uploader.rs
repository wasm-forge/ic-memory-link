use std::fs::OpenOptions;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

use crate::common::{CHUNK_SIZE, dfx_call};

fn prepare_args(offset: u64, data: &[u8]) -> String {
    let data_blob: String = data.iter().map(|&byte| format!("\\{:02X}", byte)).collect();
    format!("({} : nat64, blob \"{}\")", offset, data_blob)
}

pub fn upload_chunks(
    input: &str,
    canister: &str,
    method: &str,
    network: &Option<String>,
) -> Result<(), anyhow::Error> {
    let mut input_file = OpenOptions::new().read(true).open(input)?;

    let mut offset: u64 = 0;

    loop {
        // read up to CHUNK_SIZE bytes
        let mut buf = vec![0u8; CHUNK_SIZE];
        let n = input_file.read(&mut buf)?;

        if n == 0 {
            println!("Upload finished");
            break;
        }

        let args = prepare_args(offset, &buf[..n]);

        // write args to a temp file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.as_file_mut().write_all(args.as_bytes())?;
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let params = vec!["--argument-file".to_string(), temp_path];

        dfx_call(network, canister, method, &params).map_err(|e| anyhow::anyhow!(e))?;

        offset += n as u64;

        println!("Uploaded size: {offset}");
    }

    Ok(())
}
