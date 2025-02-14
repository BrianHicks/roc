0. Download the latest nightly from the assets [here](https://github.com/rtfeldman/roc/releases).
0. Untar the archive:
    ```
    tar -xf roc_nightly-linux_x86_64-<VERSION>.tar.gz
    ```
0. Some fresh installs require executing `sudo apt update`, it is not needed to execute `sudo apt upgrade` after this.
0. To be able to run examples:
    - for the Rust example:
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    - for the zig example:
    ```
    wget https://ziglang.org/download/0.8.1/zig-linux-x86_64-0.8.1.tar.xz
    tar -xf zig-linux-x86_64-0.8.1.tar.xz
    sudo ln -s  $(pwd)/zig-linux-x86_64-0.8.1/zig /usr/local/bin/zig
    ```
    - for the C example:
    ```
    sudo apt install build-essential clang
    ```
0. Run examples with:
    ```
    # Rust. If you installed rust in this terminal you'll need to open a new one first!
    ./roc examples/platform-switching/rust-platform/rocLovesRust.roc
    # Zig
    ./roc examples/platform-switching/zig-platform/rocLovesZig.roc
    # C
    ./roc examples/platform-switching/c-platform/rocLovesC.roc
    ```
0. See [here](../README.md#examples) for the other examples.
