use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about=format!("IC Memory Link V{}", env!("CARGO_PKG_VERSION")), long_about = None)]
pub enum MemLinkArgs {
    /// Download canister memory into a local file
    Download {
        /// Canister name
        canister: String,

        /// Canister method to use for upload
        method: String,

        /// Output file where to store the downloaded data
        output_file: String,

        /// Network type (optional)
        #[arg(long, short)]
        network: Option<String>,
    },
    /// Upload file into a canister memory
    Upload {
        /// Canister name
        canister: String,

        /// Canister method to use for upload
        method: String,

        /// Input file to upload
        input_file: String,

        /// Network type (optional)
        #[arg(long, short)]
        network: Option<String>,
    },
    /// Print out existing virtual memories
    Info {
        /// Snapshot file of the stable memory
        #[arg(long, short)]
        stable_memory: String,
    },
    /// Extract virtual memory from an existing stable memory snapshot
    Extract {
        /// Snapshot file of the stable memory
        #[arg(long, short)]
        stable_memory: String,

        /// Memory id that you wish to extract
        #[arg(long, short)]
        memory_id: u8,

        /// Output file where to store the extracted memory
        output_file: String,
    },
    /// Patch stable memory snapshot
    Patch {
        // TODO: the tool should rather work with the snapshot directory and not the individual stable memory file
        /// Snapshot file of the stable memory
        #[arg(long, short)]
        stable_memory: String,

        /// Memory id that you wish to extract
        #[arg(long, short)]
        memory_id: u8,

        /// Source of the virtual memory to patch
        input_file: String,
    },
}
