use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

use crate::arguments::MemLinkArgs;
use clap::Parser;

use ic_stable_structures::FileMemory;
use ic_stable_structures::Memory;
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::memory_manager::MemoryManager;

mod arguments;
mod downloader;

const PAGE_SIZE: usize = 65536;

pub fn get_file_memory(filename: &str) -> std::io::Result<FileMemory> {
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(filename)?;

    Ok(FileMemory::new(file))
}

fn extract_memory(stable_memory: &str, memory_id: u8, output: &str) -> Result<(), anyhow::Error> {
    let memory = get_file_memory(stable_memory)?;

    let manager = MemoryManager::init(memory);

    let mut output_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;

    let vmemory = manager.get(MemoryId::new(memory_id));

    let mut buf = [0; PAGE_SIZE];

    // copy memory info file
    for i in 0..vmemory.size() {
        vmemory.read(i * PAGE_SIZE as u64, &mut buf);

        output_file.write_all(&buf)?;
    }

    Ok(())
}

pub fn patch_memory(stable_memory: &str, memory_id: u8, input: &str) -> Result<(), anyhow::Error> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .open(stable_memory)?;

    let memory = FileMemory::new(file);
    let manager = MemoryManager::init(memory);
    let vmemory = manager.get(MemoryId::new(memory_id));

    // open the input file
    let mut input_file = OpenOptions::new().read(true).open(input)?;

    let mut buf = vec![0u8; PAGE_SIZE];
    let mut offset: u64 = 0;

    loop {
        let n = input_file.read(&mut buf)?;
        if n == 0 {
            break;
        }

        if offset + n as u64 >= vmemory.size() * PAGE_SIZE as u64 {
            // grow memory by one page
            vmemory.grow(1);
        }

        vmemory.write(offset, &buf[..n]);
        offset += n as u64;
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = MemLinkArgs::parse();

    match args {
        MemLinkArgs::Extract {
            output,
            memory_id,
            stable_memory,
        } => {
            extract_memory(&stable_memory, memory_id, &output)?;
        }

        MemLinkArgs::Patch {
            input,
            memory_id,
            stable_memory,
        } => {
            patch_memory(&stable_memory, memory_id, &input)?;
        }
    }

    Ok(())
}
