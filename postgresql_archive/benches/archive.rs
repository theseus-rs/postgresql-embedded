use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion};
use postgresql_archive::blocking::{extract, get_archive};
use postgresql_archive::{Result, LATEST};
use std::fs::{create_dir_all, remove_dir_all};
use std::time::Duration;

fn benchmarks(criterion: &mut Criterion) {
    bench_extract(criterion).ok();
}

fn bench_extract(criterion: &mut Criterion) -> Result<()> {
    let version = &LATEST;
    let (_archive_version, archive) = get_archive(version)?;

    criterion.bench_function("extract", |bencher| {
        bencher.iter(|| {
            extract_archive(&archive).ok();
        });
    });

    Ok(())
}

fn extract_archive(archive: &Bytes) -> Result<()> {
    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(archive, &out_dir)?;
    remove_dir_all(&out_dir)?;
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(30))
        .sample_size(10);
    targets = benchmarks
);
criterion_main!(benches);
