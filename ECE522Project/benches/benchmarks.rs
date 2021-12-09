use ECE522Project::avlTree::AVLTree;
use ECE522Project::rbTree::RBTree;

use rand::{rngs::StdRng, SeedableRng};
use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};


const N: usize = 64;
pub struct MyRngSeed(pub [u8; N]);
pub struct MyRng(MyRngSeed);

const TREE_SIZE: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];

fn benchmark_avl(tree_size: u32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
    // for v in 0..tree_size / 10 {
    //     avl.contains(v);
    // }
}

fn benchmark_rbt(tree_size: u32) {
    let mut rbt = RBTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
    // for v in 0..tree_size / 10 {
    //     rbt.contains(v);
    // }
}

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare");
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

criterion_group!(
    benches,
    bench_compare
);
criterion_main!(benches);