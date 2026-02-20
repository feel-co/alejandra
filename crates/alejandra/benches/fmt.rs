use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn read_case(name: &str) -> String {
    std::fs::read_to_string(format!("benches/cases/{}.nix", name)).unwrap()
}

fn bench_format(c: &mut Criterion) {
    let config = alejandra::config::Config::default();
    let small_input = read_case("small");
    let medium_input = read_case("medium");
    let large_input = read_case("large");

    let mut group = c.benchmark_group("format");

    group.bench_function("small", |b| {
        let input = small_input.clone();
        b.iter(|| {
            alejandra::format::in_memory(
                black_box("test.nix".to_string()),
                black_box(input.clone()),
                config,
            )
        });
    });

    group.bench_function("medium", |b| {
        let input = medium_input.clone();
        b.iter(|| {
            alejandra::format::in_memory(
                black_box("test.nix".to_string()),
                black_box(input.clone()),
                config,
            )
        });
    });

    group.bench_function("large", |b| {
        let input = large_input.clone();
        b.iter(|| {
            alejandra::format::in_memory(
                black_box("test.nix".to_string()),
                black_box(input.clone()),
                config,
            )
        });
    });

    group.finish();
}

criterion_group!(benches, bench_format);
criterion_main!(benches);
