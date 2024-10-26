<h2>Building an OS using Rust</h2> <br>
<p>Also learning Rust at the same time</p>

To setup machine:
1. download and install rustup
2. go inside the repo directory
3. run `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`
4. run `cargo build`
5. run `cargo install bootimage`
6. run `rustup component add llvm-tools-preview`
7. run `cargo bootimage`

To run the OS image using qemu:
`qemu-system-x86_64 -drive format=raw,file=target/x86_64-rustos/debug/bootimage-rustos.bin`
