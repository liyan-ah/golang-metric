# golang-metric

## Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

2. Update `vmlinux.rs`: `aya-tool generate task_struct thread_struct > golang-metric-ebpf/src/vmlinux.rs`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build
```

## Run

```bash
RUST_LOG=info cargo xtask run
```
