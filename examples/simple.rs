use std::{thread::sleep, time::Duration};

use typed_log::log_any;
use typed_log_parry_rerun::Points;
use typed_log_rerun::IncrementTime;

fn main() {
    typed_log_rerun::register_typed_loggers();
    typed_log_parry_rerun::register_typed_loggers();
    typed_log_rerun::init_rr();

    for i in 0..10 {
        log_any(&IncrementTime(1));
        let points = [
            [0f32, 0f32, i as f32 / 10f32].into(),
            [0f32, 1f32, i as f32 / 10f32].into(),
            [0f32, 2f32, i as f32 / 10f32].into(),
        ];
        log_any(&Points {
            record: Some("aligned points"),
            points: &points,
        });
    }
    // TODO: find a way to wait for rerun streaming to end.
    sleep(Duration::from_secs_f32(1f32));
}
