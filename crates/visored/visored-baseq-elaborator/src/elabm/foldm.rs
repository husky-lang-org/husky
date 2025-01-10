use super::*;

pub fn mapm_collect<'db, 'sess, S, A, I, MA>(
    iter: I,
    f: impl Fn(I::Item) -> MA + Clone,
) -> impl ElabM<'db, 'sess, S>
where
    'db: 'sess,
    S: Default + Extend<A> + Clone,
    I: IntoIterator,
    I::IntoIter: Clone,
    MA: ElabM<'db, 'sess, A>,
{
    miracle::foldm::mapm_collect(iter.into_iter(), move |item| {
        let f = f.clone();
        move |engine, heuristic| f(item).eval(engine, heuristic)
    })
}
