use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about=format!("IC Memory Link V{}", env!("CARGO_PKG_VERSION")), long_about = None)]
pub enum MemLinkArgs {
    /// Print out information on the existing virtual memories
    Info {
        /// snapshot
        #[arg(long, short)]
        stable_memory: String,
    },
    /// Extract virtual memory from an existing stable memory snapshot
    Extract {
        /// snapshot
        #[arg(long, short)]
        stable_memory: String,

        /// Memory id that you wish to extract
        #[arg(long, short)]
        memory_id: u8,

        /// Output file to store the extracted memory
        #[arg(long, short)]
        output: String,
    },
    /// Patch stable memory snapshot
    Patch {
        /// snapshot
        #[arg(long, short)]
        stable_memory: String,

        /// Memory id that you wish to extract
        #[arg(long, short)]
        memory_id: u8,

        /// Output file to store the extracted memory
        #[arg(long, short)]
        input: String,
    },
    /// Download canister memory into a local file
    Download {
        /// Output file where to store the downloaded data
        output: String,

        /// Canister name
        #[arg(long, short)]
        canister: String,

        /// Canister method to use for upload
        #[arg(long, short)]
        method: String,

        /// Network type (optional)
        #[arg(long, short)]
        network: Option<String>,
    },
    /*
    /// Upload file to
    Upload {
        /// File to upload
        #[arg(long)]
        file: String,

        /// Canister name
        #[arg(long)]
        canister: String,

        /// Canister method to use for upload
        #[arg(long)]
        method: String,

        /// Network type (optional)
        #[arg(long)]
        network: Option<String>,

        /// Start address
        #[arg(short, long, default_value = "0")]
        start_offset: u64,
    },

    */
}
