//! Named after Gerard Valkyrie's "The Miracle" from Bleach, a divine ability that
//! continuously revives its wielder to become stronger after each defeat.

pub mod config;
pub mod error;
pub mod foldm;
pub mod metric;
pub mod stage;
pub mod state;

use self::{config::*, error::*, metric::*, stage::*, state::*};
use alt_maybe_result::*;
use alt_option::*;
use ordered_float::NotNan;
use sealed::sealed;

pub struct Miracle {
    inner: MiracleInner,
}

impl Miracle {
    pub fn is_uninitialized(&self) -> bool {
        matches!(self.inner, MiracleInner::Uninitialized)
    }

    pub fn exceeds_norm_limit(&self) -> bool {
        self.state().norm(&self.stage().metrics) > self.stage().max_norm
    }
}

#[derive(Debug)]
pub enum MiracleInner {
    Uninitialized,
    Initialized {
        state: MiracleState,
        stage: MiracleStage,
    },
}

impl Miracle {
    pub fn new_uninitialized() -> Self {
        Self {
            inner: MiracleInner::Uninitialized,
        }
    }
}

impl Miracle {
    pub fn state(&self) -> &MiracleState {
        match &self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { state, .. } => state,
        }
    }

    pub fn stage(&self) -> &MiracleStage {
        match &self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { stage, .. } => stage,
        }
    }

    pub fn state_mut(&mut self) -> &mut MiracleState {
        match &mut self.inner {
            MiracleInner::Uninitialized => panic!("miracle is uninitialized"),
            MiracleInner::Initialized { state, .. } => state,
        }
    }
}

pub trait HasMiracle {
    fn miracle(&self) -> &Miracle;
    fn miracle_mut(&mut self) -> &mut Miracle;
}

#[sealed]
pub trait HasMiracleFull: HasMiracle {
    fn run_stages<R>(
        &mut self,
        stages: &[MiracleStage],
        f: impl FnMut(&mut Self) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>;

    fn exec_batch<R>(
        &mut self,
        fs: &[&dyn Fn(&mut Self) -> MiracleAltMaybeResult<R>],
    ) -> MiracleAltMaybeResult<R>;

    fn exec_batch2<'a, R, F>(&'a mut self, fs: &[F]) -> MiracleAltMaybeResult<R>
    where
        F: Fn(&'a mut Self) -> MiracleAltMaybeResult<R>;

    fn split<R>(
        &mut self,
        number_of_values: u64,
        f: impl FnMut(&mut Self, u64) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>;

    fn foldm<S, I, F, R>(
        &mut self,
        init: &S,
        iter: I,
        f: &[F],
        g: &impl Fn(&mut Self, &S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
    where
        I: IntoIterator,
        I::IntoIter: Clone,
        F: Fn(&mut Self, &S, &I::Item) -> S;
}

#[sealed]
impl<Engine: HasMiracle> HasMiracleFull for Engine {
    fn run_stages<R>(
        &mut self,
        stages: &[MiracleStage],
        mut f: impl FnMut(&mut Self) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R> {
        assert!(stages.len() > 0, "stages must be non-empty");
        for stage in stages {
            stage.run(self, |g| f(g))?;
        }
        AltNothing
    }

    fn exec_batch<R>(
        &mut self,
        fs: &[&dyn Fn(&mut Self) -> MiracleAltMaybeResult<R>],
    ) -> MiracleAltMaybeResult<R> {
        for (i, f) in fs.iter().enumerate() {
            crate::state::calc_with_new_value_appended(self, i as u64, |g| f(g))?;
        }
        AltNothing
    }

    fn exec_batch2<'a, R, F>(&'a mut self, fs: &[F]) -> MiracleAltMaybeResult<R>
    where
        F: Fn(&'a mut Self) -> MiracleAltMaybeResult<R>,
    {
        todo!()
        // self.exec_batch(&fs.iter().map(|f| &f).collect::<Vec<_>>())
    }

    fn split<R>(
        &mut self,
        number_of_values: u64,
        mut f: impl FnMut(&mut Self, u64) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R> {
        for i in 0..number_of_values {
            crate::state::calc_with_new_value_appended(self, i, |g| f(g, i))?;
        }
        AltNothing
    }

    fn foldm<S, I, F, R>(
        &mut self,
        init: &S,
        iter: I,
        f: &[F],
        g: &impl Fn(&mut Self, &S) -> MiracleAltMaybeResult<R>,
    ) -> MiracleAltMaybeResult<R>
    where
        I: IntoIterator,
        I::IntoIter: Clone,
        F: Fn(&mut Self, &S, &I::Item) -> S,
    {
        crate::foldm::foldm_aux(self, init, iter.into_iter(), f, g)
    }
}

#[test]
fn run_staged_alt_option_works() {
    struct Gerald {
        miracle: Miracle,
    }

    impl HasMiracle for Gerald {
        fn miracle(&self) -> &Miracle {
            &self.miracle
        }
        fn miracle_mut(&mut self) -> &mut Miracle {
            &mut self.miracle
        }
    }

    let mut gerald = Gerald {
        miracle: Miracle::new_uninitialized(),
    };
    assert_eq!(
        gerald.run_stages(
            &[MiracleStage {
                max_norm: NotNan::new(1.0).unwrap(),
                max_heartbeats: 10,
                metrics: vec![MiracleMetric::L1 {
                    scale: NotNan::new(1.0).unwrap()
                }],
            }],
            |_| AltJustOk(1)
        ),
        AltJustOk(1)
    );
}
