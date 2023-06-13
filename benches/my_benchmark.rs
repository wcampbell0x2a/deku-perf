use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deku_redo::{NewDekuRead, Test};
use deku::DekuRead;
use bitvec::prelude::Msb0;
use bitvec::view::BitView;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("deku read new", |b| b.iter(|| {
        let bytes = [0x00, 0x03];
        let b = bytes.view_bits::<Msb0>();
        let test = Test::read_new(b, ());
    }));
    c.bench_function("deku", |b| b.iter(|| {
        let bytes = [0x00, 0x03];
        let b = bytes.view_bits::<Msb0>();
        let test = Test::read(b, ());
    }));
    c.bench_function("manual", |b| b.iter(|| {
        let bytes = [0x00, 0x03];
        let test = Test::from_bytes_custom(black_box(&mut bytes.as_slice()));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
