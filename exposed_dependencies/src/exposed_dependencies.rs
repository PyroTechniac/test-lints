#![allow(warnings)]
use clippy_utils::diagnostics::span_lint;
use if_chain::if_chain;
use rustc_hir::{Crate, Item};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_session::{declare_lint, declare_lint_pass};

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
    pub EXPOSED_DEPENDENCIES,
    Warn,
    "description goes here"
}

// declare_lint_pass!(ExposedDependencies => [EXPOSED_DEPENDENCIES]);

#[derive(Default)]
pub struct ExposedDependencies {
    stack: Vec<Item<'static>>,
}

impl_lint_pass!(ExposedDependencies => [EXPOSED_DEPENDENCIES]);

impl<'tcx> LateLintPass<'tcx> for ExposedDependencies {
    fn check_crate(&mut self, _: &LateContext<'tcx>, krate: &'tcx Crate<'tcx>) {
        // dbg!(&krate.owners);
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        let map = cx.tcx.hir();
        let iter = map.parent_owner_iter(item.hir_id());
        for owner in iter {
            dbg!(&item.ident);
            dbg!(owner.1);
        }
    }
}
