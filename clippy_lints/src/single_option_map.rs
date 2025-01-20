use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::peel_blocks;
use clippy_utils::ty::is_type_diagnostic_item;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, ExprKind, FnDecl, FnRetTy};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::{Span, sym};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for functions with method calls to `.map(_)` on an arg
    /// of type `Option` as the outermost expression.
    ///
    /// ### Why is this bad?
    /// Taking and returning an `Option<T>` may require additional
    /// `Some(_)` and `unwrap` if all you have is a `T`
    ///
    /// ### Example
    /// ```no_run
    /// fn double(param: Option<u32>) -> Option<u32> {
    ///     param.map(|x| x * 2)
    /// }
    /// ```
    /// Use instead:
    /// ```no_run
    /// fn double(param: u32) -> u32 {
    ///     param * 2
    /// }
    /// ```
    #[clippy::version = "1.86.0"]
    pub SINGLE_OPTION_MAP,
    nursery,
    "Checks for functions with method calls to `.map(_)` on an arg of type `Option` as the outermost expression."
}

declare_lint_pass!(SingleOptionMap => [SINGLE_OPTION_MAP]);

impl<'tcx> LateLintPass<'tcx> for SingleOptionMap {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _fn_def: LocalDefId,
    ) {
        if let FnRetTy::Return(_ret) = decl.output
            && matches!(kind, FnKind::ItemFn(_, _, _) | FnKind::Method(_, _))
        {
            let func_body = peel_blocks(body.value);
            if let ExprKind::MethodCall(method_name, callee, _args, _span) = func_body.kind
                && method_name.ident.name == sym::map
                && let callee_type = cx.typeck_results().expr_ty(callee)
                && is_type_diagnostic_item(cx, callee_type, sym::Option)
                && let ExprKind::Path(_path) = callee.kind
            {
                span_lint_and_help(
                    cx,
                    SINGLE_OPTION_MAP,
                    span,
                    "`fn` that only maps over argument",
                    None,
                    "move the `.map` to the caller or to an `_opt` function",
                );
            }
        }
    }
}
