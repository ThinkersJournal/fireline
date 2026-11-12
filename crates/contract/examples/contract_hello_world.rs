// SPDX-License-Identifier: MIT OR Apache-2.0
//! Hello world: parse a Dispatch expression once (build time), bind it for two launches (bind time).

use fireline_contract::{BindDecline, Environment, Expr};

fn main() {
    // Build time: the contract carries `count_to_grid`; a malformed one is rejected here, never at launch.
    let grid = Expr::parse("ceil_div(n, 256)").expect("a well-formed Dispatch expression");

    for n in [1_000, 4_096] {
        let mut env = Environment::new();
        env.set_scalar("n", n);
        // Bind time: the same expression, a different value per launch.
        println!(
            "n = {n}: {} workgroups",
            grid.bind(&env, None).expect("n is assigned")
        );
    }

    // An unassigned symbol declines; no default is substituted.
    let declined = grid.bind(&Environment::new(), None);
    assert_eq!(declined, Err(BindDecline::UnboundSymbol("n".into())));
    println!("empty environment: {}", declined.unwrap_err());
}
