#![no_std]
#![no_main]

use aya_bpf::{
    helpers::{bpf_get_current_task, bpf_probe_read},
    macros::uprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;
use golang_metric_common::vmlinux;

#[uprobe]
pub fn golang_metric(ctx: ProbeContext) -> u32 {
    match try_golang_metric(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_golang_metric(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function newproc1 called by /home/work/bin");
    unsafe {
        let task = bpf_get_current_task() as *const vmlinux::task_struct;
        let fsbase = task.thread.fsbase;
        let goid = bpf_probe_read(fsbase - 8) as u64;
        Ok(goid as u32)
    };
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
