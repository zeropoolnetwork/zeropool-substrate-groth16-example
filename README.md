# ZeroPool Substrate Pallet

# WIP

Substrate pallet, supporting  alt-bn128 functions, useful for zkSNARKs privacy.

```rust
alt_bn_128_g1_multiexp(data:Vec<(G1,Fr)>) -> G1;
alt_bn_128_g1_sum(data:Vec<(bool, G1)>) -> G1;
alt_bn_128_pairing_check(data:Vec<G1,G2>) -> bool;
```