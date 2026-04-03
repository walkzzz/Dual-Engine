use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shared_types::{InputValidator, RateLimiter};

fn benchmark_validator(c: &mut Criterion) {
    let validator = InputValidator::new();
    let valid_input = "Hello, world! 你好世界";
    let long_input = "a".repeat(1000);

    c.bench_function("validator_valid_input", |b| {
        b.iter(|| validator.validate(black_box(valid_input)))
    });

    c.bench_function("validator_long_input", |b| {
        b.iter(|| validator.validate(black_box(&long_input)))
    });

    c.bench_function("validator_sanitize", |b| {
        b.iter(|| validator.sanitize(black_box(valid_input)))
    });
}

fn benchmark_rate_limiter(c: &mut Criterion) {
    let limiter = RateLimiter::new(100, 100);

    c.bench_function("rate_limiter_try_acquire", |b| {
        b.iter(|| limiter.try_acquire())
    });

    c.bench_function("rate_limiter_stats", |b| {
        b.iter(|| limiter.stats())
    });
}

criterion_group!(benches, benchmark_validator, benchmark_rate_limiter);
criterion_main!(benches);