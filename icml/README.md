# icml
`icml` (Internet Computer Memory Link) is a command-line utility to download and upload canister's memory as a way to backup an SQLite database to your local drive and upload it later.


## Installation

Install the tool from command line:
```bash
cargo install icml
```

**Note:** [`dfx`](https://internetcomputer.org/docs/building-apps/getting-started/install) needs to be installed for the tool to work.


## Download database

```bash
icml download <CANISTER> <METHOD> <OUTPUT_FILE>
```

Expected Canister's download function signature: `(offset: nat64) -> blob`

This function will be called multiple times with the increasing offset until an empty blob is returned.


## Upload Database

You can use `icml` to upload your local file into a canister
```bash
icml upload <CANISTER> <METHOD> <INPUT-FILE>
```

Expected Canister's upload function signature: `(offset: nat64, data: blob) -> ()`

This function will be called multiple times with the increasing offset until a complete file is uploaded.


## Working with the snapshot memory directly

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
