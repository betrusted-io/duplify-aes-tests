# Duplify AES Tests

This is a run-once script to create test vectors from the official Rust test vectors
which are in turn derived from https://www.cosic.esat.kuleuven.be/nessie/testvectors/.

The test vectors in the Rust test bench are stored in a de-duplified form that uses
the "Blobby" crate to read them in and iterate through them. This is very clever,
but we don't have an allocator in this `no_std` environment, and therefore we don't
have Blobby. This script takes the test vectors in, re-duplifies them, and stores them
in a static array that is a bit easier to include into the test frameworks that run
on Xous.
