# icml
`icml` (Internet Computer Memory Link) is a command-line utility to download and upload canister's memory as a way to backup an SQLite database to your local drive and upload it later.


## Installation

Install the tool from command line:
```bash
cargo install icml
```

**Note:** [`dfx`](https://internetcomputer.org/docs/building-apps/getting-started/install) needs to be installed for the tool to work.


## Download Database

```bash
icml download <CANISTER> <METHOD> <OUTPUT_FILE>
```

Expected Canister's download function signature: `(offset: nat64) -> blob`

This function will be called multiple times with the increasing offset until an empty blob is returned.

Basic implementation example:
```Rust

// Basic implementation for downloading the database using the icml tool
// The real implementation should keep canister in "service" mode to prevent database updates during download,
// also make sure only the owner of the canister can call this method
#[ic_cdk::query]
fn db_download(offset: u64) -> Vec<u8> {
    ic_rusqlite::close_connection();

    let mut file = match File::open(DB_FILENAME) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    // Get file length
    let file_len = match file.metadata() {
        Ok(meta) => meta.len(),
        Err(_) => return Vec::new(),
    };

    if offset >= file_len {
        return Vec::new();
    }

    // Seek to the requested offset
    if file.seek(SeekFrom::Start(offset)).is_err() {
        return Vec::new();
    }

    let mut buffer = Vec::with_capacity(CHUNK_SIZE);
    let mut handle = file.take(CHUNK_SIZE as u64);

    if handle.read_to_end(&mut buffer).is_err() {
        return Vec::new();
    }

    buffer
}
```


## Upload Database

You can use `icml` to upload your local file into a canister
```bash
icml upload <CANISTER> <METHOD> <INPUT-FILE>
```

Expected Canister's upload function signature: `(offset: nat64, data: blob) -> ()`

This function will be called multiple times with the increasing offset until a complete file is uploaded.

Basic implementation example:
```Rust

// Basic implementation to upload the database using the icml tool
// The real implementation should keep canister in "service" mode to prevent database updates during upload
// also make sure only the owner of the canister can call this method
#[ic_cdk::update]
fn db_upload(offset: u64, content: Vec<u8>) {
    ic_rusqlite::close_connection();

    // open file for writing
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // create file if it doesn't exist
        .open(DB_FILENAME)
    {
        if file.seek(SeekFrom::Start(offset)).is_ok() {
            // write bytes at given offset
            let _ = file.write_all(&content);
        }
    }
}

```


## Working with the Snapshot Memory

### Info

Download your canister snapshot with commands `dfx canister snapshot download`.

Once you have access to the downloaded memory file `stable_memory.bin`, you can list all the virtual memories with a non-zero length.

```bash
icml info -s stable_memory.bin 
```


### Extract memory from a snapshot

Download your canister snapshot with commands `dfx canister snapshot download`.

Once you have access to the downloaded memory file `stable_memory.bin`, you can extract one of the virtual memories into a local file.

In this example you have an SQLite database stored inside a virtual memory `120`:

```bash
icml extract -s stable_memory.bin -m 120 -o my_base.sqlite
```

### Patch memory of a stable memory snapshot

In a similar fashion you can patch existing stable memory snapshot. 

```bash
icml patch -s stable_memory.bin -m 120 -i my_base.sqlite
```

**Note:** currently patching only affects the stable memory file and will not change `.json` in case the stable memory file size has changed.
