use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_hir::*;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub DISALLOWED_MACRO,
    nursery,
    "default lint description"
}

declare_lint_pass!(DisallowedMacro => [DISALLOWED_MACRO]);

impl LateLintPass<'_> for DisallowedMacro {}
