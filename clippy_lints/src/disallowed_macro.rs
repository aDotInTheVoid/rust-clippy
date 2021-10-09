use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::fn_def_id;

use rustc_hir::{def::Res, def_id::DefIdMap, Expr};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_tool_lint, impl_lint_pass};

use crate::utils::conf;

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

#[derive(Clone, Debug)]
pub struct DisallowedMacro {
    conf_disallowed: Vec<conf::DisallowedMacro>,
    disallowed: DefIdMap<Option<String>>,
}

impl DisallowedMacro {
    pub fn new(conf_disallowed: Vec<conf::DisallowedMacro>) -> Self {
        Self {
            conf_disallowed,
            disallowed: DefIdMap::default(),
        }
    }
}

impl_lint_pass!(DisallowedMacro => [DISALLOWED_MACRO]);

impl LateLintPass<'_> for DisallowedMacro {
fn check_crate(&mut self, cx: &LateContext<'_>) {
        for conf in &self.conf_disallowed {
            let (path, reason) = match conf {
                conf::DisallowedMacro::Simple(path) => (path, None),
                conf::DisallowedMacro::WithReason { path, reason } => (
                    path,
                    reason.as_ref().map(|reason| format!("{} (from clippy.toml)", reason)),
                ),
            };
            let segs: Vec<_> = path.split("::").collect();
            if let Res::Def(_, id) = clippy_utils::path_to_res(cx, &segs) {
                self.disallowed.insert(id, reason);
            }
        }
    }

}
