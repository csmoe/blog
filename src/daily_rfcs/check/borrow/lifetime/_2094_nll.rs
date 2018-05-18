//! Non-lexical lifetimes: lifetimes that are based on the control-flow graph, rather than lexical scopes.
//!
//! ## What is a lifetime?
//! Lifetime is a set of points in the Control-Flow Graph(includes "skolemized" lifetimes,
//! which correspond to named lifetime parameters declared on a function).
//! ## Liveness
//! * Value: a variable is live if the current value that it holds may be used later.
//! * Lifetime: a lifetime L is live at a point P if there is some variable `p` which is live at `P`, and `L` appears in the type of `p`.
