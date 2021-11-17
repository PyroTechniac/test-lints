use clippy_utils::diagnostics::span_lint;
use if_chain::if_chain;
use rustc_hir::{Impl, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::sym;

declare_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where a warning is issued
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// ```
    pub NON_DYNAMIC_ERRORS,
    Warn,
    "description goes here"
}

declare_lint_pass!(NonDynamicErrors => [NON_DYNAMIC_ERRORS]);

impl<'tcx> LateLintPass<'tcx> for NonDynamicErrors {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if_chain! {
            if let ItemKind::Impl(Impl {
                of_trait: Some(ref trait_ref),
                ..
            }) = item.kind;
            if let Some(trait_id) = trait_ref.trait_def_id();
            if cx.tcx.is_diagnostic_item(sym::Error, trait_id);
            then {
                span_lint(cx, NON_DYNAMIC_ERRORS, item.span, "test");
            }
        }
    }
}
