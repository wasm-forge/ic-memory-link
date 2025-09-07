use ic_stable_structures::FileMemory;
use ic_stable_structures::Memory;
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::memory_manager::MemoryManager;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

const PAGE_SIZE: usize = 65536;

pub fn print_information(stable_memory_path: &str) -> Result<(), anyhow::Error> {
    let stable_memory_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(stable_memory_path)?;

    let memory = FileMemory::new(stable_memory_file);

    let manager = MemoryManager::init(memory);

    for i in 0..255u8 {
        let vmemory = manager.get(MemoryId::new(i));
        if vmemory.size() < 1 {
            continue;
        }
        println!("Memory{i:03}.size = {}", vmemory.size());
    }

    Ok(())
}

pub fn extract_memory(
    stable_memory_path: &str,
    memory_id: u8,
    output: &str,
) -> Result<(), anyhow::Error> {
    println!("Writing into file... {output}");
    let stable_memory_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(stable_memory_path)?;

    let memory = FileMemory::new(stable_memory_file);

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
        println!("Writing page{i}");
        vmemory.read(i * PAGE_SIZE as u64, &mut buf);

        output_file.write_all(&buf)?;
    }

    let _ = output_file.flush();

    Ok(())
}

pub fn patch_memory(stable_memory: &str, memory_id: u8, input: &str) -> Result<(), anyhow::Error> {
    println!("Patching stable memory... {stable_memory} virtual memory {memory_id}");

    let file = OpenOptions::new()
        .read(true)
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
            println!("Finished processing input file");
            break;
        }

        if offset + n as u64 > vmemory.size() * PAGE_SIZE as u64 {
            // grow memory by one page
            vmemory.grow(1);
        }

        println!("Writing {n} bytes at address {offset}");

        vmemory.write(offset, &buf[..n]);
        offset += n as u64;
    }

    Ok(())
}
