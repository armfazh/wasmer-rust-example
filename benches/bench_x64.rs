use criterion::{ criterion_group, criterion_main, Criterion};

use redox_ecc::instances::{P256};
use redox_ecc::instances::GetCurve;
use redox_ecc::ellipticcurve::EllipticCurve;
use redox_ecc::weierstrass::Point;

fn do_generator()->Point{
  let ec = P256.get();
  let g0 = ec.get_generator();
  let g1 = ec.get_generator();
  return g0+g1
}

pub fn criterion_benchmark(c: &mut Criterion) {
 c.bench_function("do_generator", |b| b.iter(||   {
      do_generator();
   }
));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
