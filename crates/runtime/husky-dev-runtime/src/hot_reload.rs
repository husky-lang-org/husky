use monad::Monad;

use crate::*;

pub struct HuskyRuntimeHotReloadM;

impl Monad for HuskyRuntimeHotReloadM {}

impl HuskyDevRuntime {
    pub fn hot_reload(&mut self) -> HuskyRuntimeHotReloadM {
        CompilerInstance::new(
            RelativePathBuf::from_path(&self.config.comptime.package_dir).unwrap(),
        )
        .compile_all();
        self.load_package();
        self.load_linkages();
        HuskyRuntimeHotReloadM
    }
}

impl std::ops::Try for HuskyRuntimeHotReloadM {
    type Output = ();

    type Residual = HuskyRuntimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl std::ops::FromResidual<HuskyRuntimeHotReloadR> for HuskyRuntimeHotReloadM {
    fn from_residual(residual: HuskyRuntimeHotReloadR) -> Self {
        todo!()
    }
}

pub struct HuskyRuntimeHotReloadR;
