use crate::arguments::MemLinkArgs;
use crate::downloader::download_chunks;
use crate::uploader::upload_chunks;
use clap::Parser;

use crate::snapshot::extract_memory;
use crate::snapshot::patch_memory;
use crate::snapshot::print_information;

mod arguments;
mod common;
mod downloader;
mod snapshot;
mod uploader;

fn main() -> Result<(), anyhow::Error> {
    let args = MemLinkArgs::parse();

    match args {
        MemLinkArgs::Extract {
            memory_id,
            stable_memory,
            output_file,
        } => {
            extract_memory(&stable_memory, memory_id, &output_file)?;
        }

        MemLinkArgs::Patch {
            input_file,
            memory_id,
            stable_memory,
        } => {
            patch_memory(&stable_memory, memory_id, &input_file)?;
        }

        MemLinkArgs::Info { stable_memory } => {
            print_information(&stable_memory)?;
        }

        MemLinkArgs::Download {
            output_file,
            canister,
            method,
            network,
        } => {
            download_chunks(&output_file, &canister, &method, &network)?;
        }

        MemLinkArgs::Upload {
            input_file,
            canister,
            method,
            network,
        } => {
            upload_chunks(&input_file, &canister, &method, &network)?;
        }
    }

    Ok(())
}
