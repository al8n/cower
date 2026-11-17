use cower::Cow;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn cow(c: &mut Criterion) {
  c.bench_function("borrowed str", |b| {
    b.iter(|| {
      let value: Cow<'_, str> = Cow::Borrowed(black_box("hello"));
      black_box(value.len())
    })
  });

  c.bench_function("str into owned", |b| {
    b.iter(|| {
      let value: Cow<'_, str> = Cow::Borrowed(black_box("hello"));
      black_box(value.into_owned())
    })
  });
}

criterion_group!(benches, cow);
criterion_main!(benches);
