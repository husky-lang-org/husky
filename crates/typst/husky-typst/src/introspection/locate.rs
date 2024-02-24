use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, func, Func, IsTypstElem, Show, TypstContent, TypstContentRefined, TypstStyleChain,
};
use crate::introspection::TypstLocatable;
use crate::syntax::TypstSynSpan;

/// Provides access to the location of content.
///
/// This is useful in combination with [queries]($query), [counters]($counter),
/// [state]($state), and [links]($link). See their documentation for more
/// details.
///
/// ```example
/// #locate(loc => [
///   My location: \
///   #loc.position()!
/// ])
/// ```
#[func]
pub fn locate(
    /// The span of the `locate` call.
    span: TypstSynSpan,
    /// A function that receives a [`location`]($location). Its return value is
    /// displayed in the document.
    ///
    /// This function is called once for each time the content returned by
    /// `locate` appears in the document. That makes it possible to generate
    /// content that depends on its own location in the document.
    func: Func,
) -> TypstContent {
    LocateElem::new(func).pack().spanned(span)
}

/// Executes a `locate` call.
#[elem(TypstLocatable, Show)]
struct LocateElem {
    /// The function to call with the location.
    #[required]
    func: Func,
}

impl Show for TypstContentRefined<LocateElem> {
    #[husky_typst_macros::time(name = "locate", span = self.span())]
    fn show(
        &self,
        engine: &mut TypstEngine,
        _: TypstStyleChain,
    ) -> TypstSourceResult<TypstContent> {
        let location = self.location().unwrap();
        Ok(self.func().call(engine, [location])?.display())
    }
}