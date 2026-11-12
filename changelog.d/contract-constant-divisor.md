kind: `fireline-contract` constant divisors are a build-time fault
---
Tracks KISS#800 (CONTRACT-6.12-0007, -0008, origin/main@73c341b5). `division_by_zero` and `ceil_div_divisor` are now ratified codes and stay bind-time declines only when the divisor names a launch-scalar symbol. A divisor with no symbol that is zero (`/`) or not positive (`ceil_div`), such as `n / 0` or `ceil_div(8, 0)`, now fails `Expr::parse` with the new `ParseError::ConstantDivisor` (code `build_time`) instead of declining at bind time. `subscript_out_of_range` is still unnamed by the Draft and stays a bind-time decline here (kiss reports it at build time; the rank is only known from the environment). Verified with `cargo test -p fireline-contract --locked` (8 integration tests and the doctest pass).

— lane: fireline
