// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
}
