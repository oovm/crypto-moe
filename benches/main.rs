#[macro_use]
extern crate criterion;
use criterion::Criterion;
use std::time::Duration;

pub mod encoding;
use encoding::marysue_benches;
criterion_group! {
    name = marysue;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(1000));
    targets = marysue_benches
}

criterion_main!(marysue);
