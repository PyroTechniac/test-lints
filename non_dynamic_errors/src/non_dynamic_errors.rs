use rustc_lint::LateLintPass;
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
    pub NON_DYNAMIC_ERRORS,
    Warn,
    "description goes here"
}

declare_lint_pass!(NonDynamicErrors => [NON_DYNAMIC_ERRORS]);

impl<'hir> LateLintPass<'hir> for NonDynamicErrors {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html
}
