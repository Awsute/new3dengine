use std::time::{Duration, Instant};

pub fn start_timer() -> Instant {
    let timer = Instant::now();
    return timer;
}

pub fn end_timer(timer : Instant) -> f32{
    return timer.elapsed().as_secs_f64() as f32
}