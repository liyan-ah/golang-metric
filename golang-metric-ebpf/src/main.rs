#![no_std]
#![no_main]

use aya_bpf::{
    macros::uprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[uprobe]
pub fn golang_metric(ctx: ProbeContext) -> u32 {
    match try_golang_metric(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_golang_metric(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function newproc1 called by /home/work/bin");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
