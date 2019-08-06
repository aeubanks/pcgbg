use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcgbg::pcgbg_buf::Buf;
use pcgbg::pcgbg_dist::DistanceEntryDistribution;
use pcgbg::pcgbg_noise::NoiseDistribution;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn iter(size: usize) -> Buf {
    let size = black_box(size);
    let scale = 0.5;
    let mut rng = SmallRng::seed_from_u64(1);

    let noise_distribution = NoiseDistribution { scale };
    let noise = noise_distribution.sample(&mut rng);
    let dist_entry_distribution = DistanceEntryDistribution {
        width: size,
        height: size,
    };
    let dist_entry = dist_entry_distribution.sample(&mut rng);

    let mut buf = Buf::new(size, size);
    buf.add(&noise, &[1.4, 0.1, 0.0]);
    buf.add(&dist_entry, &[0.0, 0.1, 0.5]);
    buf.normalize();
    buf
}

fn bench(c: &mut Criterion) {
    c.bench_function("noise and dist_entry basic", |b| b.iter(|| iter(100)));
}

fn inputs(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "noise and dist_entry by size",
        |b, &&size| b.iter(|| iter(size)),
        &[1usize, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
    );
}

criterion_group!(benches, bench, inputs);
criterion_main!(benches);
