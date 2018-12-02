// -*- mode: rust; -*-
//
// This file is part of x25519-dalek.
// Copyright (c) 2017 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

//! Benchmark the Diffie-Hellman operation.

#[macro_use]
extern crate criterion;
extern crate curve25519_dalek;
extern crate rand;
extern crate x25519_dalek;

use criterion::Criterion;

use curve25519_dalek::montgomery::MontgomeryPoint;

use rand::OsRng;

use x25519_dalek::EphemeralPublic;
use x25519_dalek::EphemeralSecret;

fn bench_diffie_hellman(c: &mut Criterion) {
    let mut csprng: OsRng = OsRng::new().unwrap();
    let alice_secret: EphemeralSecret = EphemeralSecret::new(&mut csprng);
    let bob_secret: EphemeralSecret = EphemeralSecret::new(&mut csprng);
    let bob_public: EphemeralPublic = EphemeralPublic::from(&bob_secret);

    c.bench_function("diffie_hellman", move |b| {
        b.iter(||
               EphemeralSecret::diffie_hellman(&alice_secret, &bob_public)
        )
    });
}

criterion_group!{
    name = x25519_benches;
    config = Criterion::default();
    targets =
        bench_diffie_hellman,
}
criterion_main!{
    x25519_benches,
}
