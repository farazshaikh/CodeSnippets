use anyhow::Result;
use governor::{DefaultDirectRateLimiter, RateLimiter};
use nonzero_ext::nonzero;

async fn rate_limited_print(idx: u32, rl: &DefaultDirectRateLimiter) {
    for i in 0..100 {
        println!("{idx} {i}");
        rl.until_ready_with_jitter().await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    use governor::Quota;
    let rl = RateLimiter::direct(Quota::per_second(nonzero!(2u32)));

    let clock = governor::clock::DefaultClock::default();
    let drl: DefaultDirectRateLimiter =
        RateLimiter::direct_with_clock(Quota::per_second(nonzero!(2u32)), &clock);

    let f1 = rate_limited_print(1, &drl);
    let f2 = rate_limited_print(2, &drl);
    let f3 = rate_limited_print(3, &drl);
    let f4 = rate_limited_print(4, &drl);
    futures::join!(f1, f2, f3, f4);
    println!("Hello, world!");
    Ok(())
}
