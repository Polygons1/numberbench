use std::{hint::black_box, time::Instant};

macro_rules! bench {
    ($t:tt) => {{
        let now = Instant::now();
        let mut count = $t::MIN;
        while count < $t::MAX {
            count = black_box(count + 1);
        }
        assert_eq!(count, $t::MAX, "\x1b[31m\u{10102}\x1b[0m test failed");
        let elapsed = now.elapsed();
        println!(
            "\x1b[32m\u{2714}\x1b[0m it took {}s to count to {} from {} ({})",
            elapsed.as_secs_f64(),
            $t::MAX,
            $t::MIN,
            stringify!($t)
        )
    }};
}

fn main() {
    bench!(u8);
    bench!(u16);
    bench!(u32);
    bench!(u64);
    bench!(u128);
}
