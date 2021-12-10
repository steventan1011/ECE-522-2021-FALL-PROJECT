use tree_collections::prelude::*;

use rand::{rngs::StdRng, SeedableRng};
use rand::seq::{SliceRandom, IteratorRandom};
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};


const N: usize = 64;
pub struct MyRngSeed(pub [u8; N]);
pub struct MyRng(MyRngSeed);

const TREE_SIZE: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];

fn benchmark_avl_insert(tree_size: u32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
}

fn benchmark_rbt_insert(tree_size: u32) {
    let mut rbt = RBTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
}

fn benchmark_avl(tree_size: u32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
    for v in 0..tree_size / 10 {
        // avl.contains(v);
    }
}

fn benchmark_rbt(tree_size: u32) {
    let mut rbt = RBTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
    for v in 0..tree_size / 10 {
        rbt.contains(v);
    }
}

fn benchmark_avl_insert_delete(tree_size: u32) {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<u32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    let sample = data.iter().choose_multiple(&mut rng, (tree_size / 10) as usize);

    let mut avl = AVLTree::new();
    for v in &data {
        avl.insert(*v);
    }
    for v in sample.iter() {
        avl.delete(**v);
    }
}

fn benchmark_rbt_insert_delete(tree_size: u32) {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<u32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    let sample = data.iter().choose_multiple(&mut rng, (tree_size / 10) as usize);

    let mut rbt = RBTree::new();
    for v in &data {
        rbt.insert(*v);
    }

    for v in sample.iter() {
        rbt.delete(**v);
    }
}

fn bench_compare_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare insert");
    for (i, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("AVL", i), size,
            |b, n| b.iter(|| benchmark_avl_insert(*n))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", i), size,
            |b, n| {
                b.iter(|| benchmark_rbt_insert(*n));
            }
        );
    }
    group.finish();
}

fn bench_compare_search_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare search");
    
    for (i, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("AVL", i), size,
            |b, n| b.iter(|| benchmark_avl(*n))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", i), size,
            |b, n| {
                b.iter(|| benchmark_rbt(*n));
            }
        );
    }
    group.finish();
}

fn bench_compare_insert_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare_insert_delete");
    for (idx, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("AVL", idx), size,
            |b, i| b.iter(|| benchmark_avl_insert_delete(*i))
        );
        group.bench_with_input(
            BenchmarkId::new("RBT", idx), size,
            |b, i| {
                b.iter(|| benchmark_rbt_insert_delete(*i));
            }
        );
    }
    group.finish();
}


criterion_group!(
    benches,
    bench_compare_insert,
    bench_compare_search_insert,
    bench_compare_insert_delete
);
criterion_main!(benches);