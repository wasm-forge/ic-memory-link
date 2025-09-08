use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

use crate::common::dfx_call;

pub fn blob_to_vec_u8(ret: &str) -> Vec<u8> {
    let s = ret.trim();

    // --- Case 1: blob string ---
    if s.starts_with('(')
        && s.contains("blob")
        && let Some(start) = ret.find('"')
        && let Some(end) = ret.rfind('"')
    {
        let content = &s[start + 1..end];
        let mut bytes = Vec::new();
        let mut i = 0;
        while i < content.len() {
            if &content[i..i + 1] == "\\" {
                // Expect two hex digits after "\"
                if i + 3 <= content.len() {
                    let hex = &content[i + 1..i + 3];
                    if let Ok(val) = u8::from_str_radix(hex, 16) {
                        bytes.push(val);
                    }
                }
                i += 3;
            } else {
                // Direct ASCII char
                bytes.push(content.as_bytes()[i]);
                i += 1;
            }
        }
        return bytes;
    }

    // --- Case 2: vec { ... } ---
    if s.starts_with('(')
        && s.contains("vec")
        && let Some(start) = s.find('{')
        && let Some(end) = s.rfind('}')
    {
        let inside = &s[start + 1..end];
        let mut bytes = Vec::new();
        for token in inside.split(';') {
            let t = token.trim();

            if !t.is_empty()
                && let Ok(val) = t.parse::<u8>()
            {
                bytes.push(val);
            }
        }
        return bytes;
    }

    // fallback: unsupported format
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
