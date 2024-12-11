mod builder;
pub mod dictionary;
mod expr;
pub mod helpers;
pub mod mangle;
pub mod namespace;
mod scheme;
pub mod stmt;
#[cfg(test)]
mod tests;
pub mod ty;

use self::builder::VdLeanTranspilationBuilder;
use self::scheme::IsVdLeanTranspilationScheme;
#[cfg(test)]
use self::tests::*;
use visored_llm::VdLlm;

pub trait VdTranspileToLean<S, T>: Copy
where
    S: IsVdLeanTranspilationScheme,
{
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<S>) -> T;
}
