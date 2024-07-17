use criterion::{black_box, criterion_group, criterion_main, Criterion};

use curieosa::{CurieUtil, TrieCurieUtil};

fn benchmark_trie_curie_util(c: &mut Criterion) {
    let tcu = TrieCurieUtil::default();

    c.bench_function("TrieCurieUtil::get_curie_data", |b| {
        b.iter(|| {
            let cp = tcu.get_curie_data("http://purl.obolibrary.org/obo/HP_0001250");
            black_box(cp);
        })
    });
}

criterion_group!(benches, benchmark_trie_curie_util);
criterion_main!(benches);
