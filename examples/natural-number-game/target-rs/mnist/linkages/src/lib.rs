use husky_core::*;
use ad_hoc_task_dependency::*;
use mnist::*;

#[rustfmt::skip]
linkage_impls![
    mnist::BinaryImage28,
    mnist::BinaryGrid28,
    <mnist::BinaryImage28 as Visualize>::visualize,
    mnist::BinaryImage28::new_zeros,
    <mnist::BinaryGrid28 as Visualize>::visualize,
    mnist::BinaryGrid28::new_zeros,
];