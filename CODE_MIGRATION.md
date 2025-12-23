# Code Migration Issues

The following implementations from moonmath should be migrated to mathlib or curvelib.

## Target: mathlib (https://github.com/Tranduy1dol/mathlib)

| File | Function/Struct | Description | Priority |
|------|-----------------|-------------|----------|
| `utils/src/extended_gcd.rs` | `extended_gcd` | Extended Euclidean Algorithm | HIGH |
| `utils/src/modular_operations.rs` | `Modular` | Modular arithmetic struct (add, sub, mul, div, inv) | HIGH |
| `arithmetic/src/long_division.rs` | `poly_long_division` | Polynomial long division | MEDIUM |
| `arithmetic/src/lagrange_interpolation.rs` | `lagrange_interpolation` | Lagrange polynomial interpolation | MEDIUM |
| `arithmetic/src/chinese_remainder.rs` | `chinese_remainder_solver` | Chinese Remainder Theorem solver | MEDIUM |
| `algebra/src/hash_to_zn.rs` | `hash_to_zn` | SHA256-based hash to Z_n | LOW |
| `algebra/src/cyclic_group_exponentiation.rs` | `cge` | Modular exponentiation (g^x mod n) | LOW |
| `algebra/src/efficient_scalar_multiplication.rs` | `esm` | Double-and-add scalar multiplication | LOW |

**Note**: `mathlib` already contains `FieldElement`, `DensePolynomial`, and `U1024`. Check for redundancy before migrating.

## Target: curvelib (https://github.com/Tranduy1dol/curvelib)

| File | Function/Struct | Description | Priority |
|------|-----------------|-------------|----------|
| `utils/src/elliptic_curve_operations.rs` | `EllipticCurve` | Generic EC implementation (Short Weierstrass, Montgomery, Twisted Edwards) | HIGH |
| `utils/src/elliptic_curve_operations.rs` | `Point` | Projective point representation | HIGH |
| `utils/src/elliptic_curve_operations.rs` | `projective_add` | Point addition in projective coordinates | HIGH |
| `utils/src/elliptic_curve_operations.rs` | `esm` | EC scalar multiplication | HIGH |

**Note**: `curvelib` has specific instances like `TinyJubjub`. The `EllipticCurve` struct in `moonmath` is more generic and educational.
