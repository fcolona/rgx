use rgx::service::filter_by_regex;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

fn filter_by_regex_benchmark(c: &mut Criterion){
    let path = "/home/felipe/Desktop".to_owned();
    let regex = r"([A-Z])\w+".to_owned();

    c.bench_function("filter_by_regex", 
        |b| b.iter(|| filter_by_regex(&path, &regex))
    ); 
}

criterion_group!(benches, filter_by_regex_benchmark);
criterion_main!(benches);
