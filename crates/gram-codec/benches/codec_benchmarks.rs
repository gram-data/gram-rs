// Performance benchmarks for gram codec
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use gram_codec::{parse_gram_notation, serialize_pattern, to_gram};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

/// Generate a simple node pattern: (node_N)
fn generate_simple_node(n: usize) -> String {
    format!("(node_{})", n)
}

/// Generate multiple simple node patterns
fn generate_simple_nodes(count: usize) -> String {
    (0..count)
        .map(|i| generate_simple_node(i))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a chain of relationships: (a)-->(b)-->(c)-->...
fn generate_relationship_chain(count: usize) -> String {
    let nodes: Vec<String> = (0..count).map(|i| format!("(n{})", i)).collect();
    nodes.join("-->")
}

/// Generate a subject pattern with many elements: [root | e1, e2, e3, ...]
fn generate_subject_pattern(element_count: usize) -> String {
    let elements: Vec<String> = (0..element_count).map(|i| format!("(e{})", i)).collect();
    format!("[root | {}]", elements.join(", "))
}

/// Generate a node with properties
fn generate_node_with_properties(prop_count: usize) -> String {
    let properties: Vec<String> = (0..prop_count)
        .map(|i| format!("prop{}: \"value{}\"", i, i))
        .collect();
    format!("(node {{{}}})", properties.join(", "))
}

/// Create a pattern for serialization
fn create_pattern(id: &str) -> Pattern<Subject> {
    Pattern::point(Subject {
        identity: Symbol(id.to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    })
}

/// Create multiple patterns for serialization
fn create_patterns(count: usize) -> Vec<Pattern<Subject>> {
    (0..count)
        .map(|i| create_pattern(&format!("node_{}", i)))
        .collect()
}

/// Benchmark parsing simple nodes
fn bench_parse_simple_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_simple_nodes");

    for size in [10, 100, 1000].iter() {
        let input = generate_simple_nodes(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| parse_gram_notation(black_box(input)))
        });
    }

    group.finish();
}

/// Benchmark parsing relationship chains
fn bench_parse_relationships(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_relationships");

    for size in [10, 100, 500].iter() {
        let input = generate_relationship_chain(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| parse_gram_notation(black_box(input)))
        });
    }

    group.finish();
}

/// Benchmark parsing subject patterns
fn bench_parse_subject_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_subject_patterns");

    for size in [10, 100, 500].iter() {
        let input = generate_subject_pattern(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| parse_gram_notation(black_box(input)))
        });
    }

    group.finish();
}

/// Benchmark parsing nodes with properties
fn bench_parse_with_properties(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_with_properties");

    for prop_count in [5, 10, 20].iter() {
        let input = generate_node_with_properties(*prop_count);
        group.throughput(Throughput::Elements(*prop_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(prop_count),
            &input,
            |b, input| b.iter(|| parse_gram_notation(black_box(input))),
        );
    }

    group.finish();
}

/// Benchmark serializing single patterns
fn bench_serialize_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_single");

    let pattern = create_pattern("test_node");
    group.bench_function("single_pattern", |b| {
        b.iter(|| serialize_pattern(black_box(&pattern)))
    });

    group.finish();
}

/// Benchmark serializing multiple patterns
fn bench_serialize_multiple(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_multiple");

    for size in [10, 100, 1000].iter() {
        let patterns = create_patterns(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &patterns,
            |b, patterns| b.iter(|| to_gram(black_box(patterns))),
        );
    }

    group.finish();
}

/// Benchmark round-trip (parse → serialize)
fn bench_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip");

    for size in [10, 100, 500].iter() {
        let input = generate_simple_nodes(*size);
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let patterns = parse_gram_notation(black_box(input)).unwrap();
                to_gram(black_box(&patterns))
            })
        });
    }

    group.finish();
}

/// Benchmark complex patterns with mixed syntax
fn bench_parse_complex(c: &mut Criterion) {
    let complex_patterns = vec![
        "(alice:Person {name: \"Alice\", age: 30})-[:KNOWS {since: 2020}]->(bob:Person {name: \"Bob\", age: 25})",
        "[team:Team {name: \"Engineering\", size: 50} | (alice), (bob), (charlie), (dave), (eve)]",
        "@type(node) @depth(2) [nested | [inner:Group | (a), (b)], [inner2:Group | (c), (d)]]",
        "(a)-->(b)-->(c)-->(d)-->(e)-->(f)-->(g)-->(h)-->(i)-->(j)",
    ];

    let mut group = c.benchmark_group("parse_complex");

    for (i, pattern) in complex_patterns.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(i), pattern, |b, pattern| {
            b.iter(|| parse_gram_notation(black_box(pattern)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_simple_nodes,
    bench_parse_relationships,
    bench_parse_subject_patterns,
    bench_parse_with_properties,
    bench_serialize_single,
    bench_serialize_multiple,
    bench_round_trip,
    bench_parse_complex,
);
criterion_main!(benches);
