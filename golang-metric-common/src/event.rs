#![no_std]
pub struct EventBase {
    pub event_type: u64,
    pub pid_tgid: u64,
    pub trace: u64,
}
pub struct Event {
    pub event_base: EventBase,
}
