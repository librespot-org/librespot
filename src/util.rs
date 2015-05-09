use rand::{Rng,Rand};

pub fn rand_vec<G: Rng, R: Rand>(rng: &mut G, size: usize) -> Vec<R> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(R::rand(rng));
    }

    return vec
}

pub fn alloc_buffer(size: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }

    vec
}

pub mod version {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));

    pub fn version_string() -> String {
        format!("librespot-{}", short_sha())
    }
}

