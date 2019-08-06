use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcgbg::pcgbg_buf::Buf;
use pcgbg::pcgbg_dist::DistanceEntryDistribution;
use pcgbg::pcgbg_noise::NoiseDistribution;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn bench(c: &mut Criterion) {
    c.bench_function("noise and dist_entry", |b| {
        b.iter(|| {
            let size = black_box(100);
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
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
