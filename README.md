# icml
`icml` (Internet Computer Memory Link) is a command-line utility to download and upload canister memory as a way to backup and restore SQLite database.


## Installation

```bash
cargo install icml
```


# Download database

```bash
icml download -o my_base.sqlite <CANISTER-NAME> <DOWNLOAD-FUNCTION>
```

Expected download function signature: `(address: nat64, length: nat64) -> blob`


# Upload database

This function is currently not implemented, use `ic-file-uploader`.


# Working with snapshot memory directly



## Extract memory from a snapshot

Download your canister snapshot with commands `dfx canister snapshot download`.

Once you have access to the downloaded memory file `stable_memory.bin`, you can extract one of the virtual memories into a local file.

In this example you have an SQLite database stored inside a virtual memory `120`:

```bash
icml extract -s stable_memory.bin -m 120 -o my_base.sqlite
```

## Patch memory of a stable memory snapshot

In a similar fashion you can patch existing stable memory snapshot. 

```bash
icml patch -s stable_memory.bin -m 120 -i my_base.sqlite
```

