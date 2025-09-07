use crate::arguments::MemLinkArgs;
use crate::downloader::download_chunks;
use clap::Parser;

use crate::snapshot::extract_memory;
use crate::snapshot::patch_memory;
use crate::snapshot::print_information;

mod arguments;
mod common;
mod downloader;
mod snapshot;

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

        MemLinkArgs::Info { stable_memory } => {
            print_information(&stable_memory)?;
        }

        MemLinkArgs::Download {
            output,
            canister,
            method,
            network,
        } => {
            download_chunks(&output, &canister, &method, &network)?;
        }
    }

    Ok(())
}
