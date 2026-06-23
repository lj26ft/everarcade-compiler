# Operator Identity Registry RC1 Review Guide

Run the happy path:

```bash
everarcade operator registry init
everarcade operator register
everarcade operator verify
everarcade operator rotate-key
everarcade operator verify --public-key ed25519:frontier-settlement-operator-rc1-rotated
everarcade operator revoke
everarcade operator registry verify
```

Expected: every command prints `PASS`.

Run duplicate ID, duplicate active key, tampered hash, and broken rotation fixtures through `everarcade operator registry verify --registry <fixture>`. Run the revoked operator fixture through `everarcade operator verify --registry <fixture>`. Expected result for every failure fixture is `FAIL`.
