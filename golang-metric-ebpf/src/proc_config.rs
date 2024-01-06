#![no_std]
#![no_main]

use golang_metric_common::config;

use aya_bpf::macros::map;
use aya_bpf::maps::HashMap;

#[map]
pub static PROC_CONFIG: HashMap<u32, config::ProcConfig> =
    HashMap::with_max_entries(1024, 0);
