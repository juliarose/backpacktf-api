use criterion::{criterion_group, criterion_main, Criterion};
use backpacktf_api::response::listing::Listing;

fn criterion_benchmark(c: &mut Criterion) {
    let listing_json = include_bytes!("../src/response/listing/fixtures/listing.json");
    
    c.bench_function("deserialize listing", |b| b.iter(||
        serde_json::from_slice::<Listing>(listing_json)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);