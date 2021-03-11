use criterion::{black_box, Criterion};
use flatdata::MemoryResourceStorage;
use std::sync::Arc;

pub trait Serialize {
    type Archive;

    fn serialize_fd(&self) -> std::io::Result<Arc<MemoryResourceStorage>>;
}

pub fn bench<T, O, R>(name: &'static str, c: &mut Criterion, data: &T, open: O, read: R)
where
    T: Serialize,
    O: Fn(Arc<MemoryResourceStorage>) -> T::Archive,
    R: Fn(&T::Archive),
{
    let mut group = c.benchmark_group(format!("{}/flatdata", name));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(data.serialize_fd().unwrap());
        })
    });

    let storage = data.serialize_fd().unwrap();
    let archive = open(storage);

    // group.bench_function("access", |b| {
    //     b.iter(|| {
    //         black_box(flatbuffers::get_root::<<T as Serialize<'_>>::Target>(black_box(deserialize_buffer)))
    //     })
    // });

    group.bench_function("read", |b| b.iter(|| black_box(read(black_box(&archive)))));

    // println!("{}/flatbuffers/size {}", name, deserialize_buffer.len());
    // println!("{}/flatbuffers/zlib {}", name, crate::zlib_size(deserialize_buffer));

    group.finish();
}
