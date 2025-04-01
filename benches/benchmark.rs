use coprocessor_benchmarks::Silent_threshold_enc::STE;
use coprocessor_benchmarks::{DataType, EncryptionScheme};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};

fn generate_random_data() -> Vec<DataType> {
    let mut rng = thread_rng();

    let samples = vec![
        DataType::U32(rng.gen::<u32>()),   // Random u32
        DataType::U64(rng.gen::<u64>()),   // Random u64
        DataType::U128(rng.gen::<u128>()), // Random u128
        {
            let mut bytes = [0u8; 32];
            rng.fill(&mut bytes); // Fill with random bytes
            DataType::Bytes32(bytes)
        },
    ];

    samples
}

fn benchmark(c: &mut Criterion) {
    let node_values = [4, 8, 16];
    // let batch_sizes = [8, 16, 32, 64];

    let schemes: Vec<Box<dyn EncryptionScheme>> = vec![Box::new(STE)];

    for scheme in schemes.iter() {
        for n in node_values {
            
                let data_to_benchmark = generate_random_data();
                for &data in &data_to_benchmark {
                    let inputs = scheme.get_inputs( data, n);
                    c.bench_function(
                        &format!(
                            "scheme_{}_encryption_no_of_nodes_{}_data_{:?}",
                            scheme.scheme_name(),
                            n,
                            
                            data
                        ),
                        |b| b.iter(|| black_box(scheme.encrypt(inputs))),
                    );
                    c.bench_function(
                        &format!(
                            "scheme_{}_decryption_no_of_nodes_{}_data_{:?}",
                            scheme.scheme_name(),
                            n,
                        data
                        ),
                        |b| b.iter(|| black_box(scheme.decrypt(inputs))),
                    );
                }
            
        }
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

