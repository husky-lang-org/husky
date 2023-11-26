/// this module is for instantiation to use
use crate::{instantiation::*, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermTemplateParameters {
    data: SmallVec<[EtherealTemplateParameter; 2]>,
}

impl<'a> IntoIterator for &'a EtherealTermTemplateParameters {
    type Item = &'a EtherealTemplateParameter;

    type IntoIter = impl Iterator<Item = &'a EtherealTemplateParameter> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl EtherealTermTemplateParameters {
    pub fn from_declarative(
        db: &dyn EtherealTermDb,
        template_parameters: &[DeclarativeTemplateParameter],
    ) -> EtherealTermResult<EtherealTermTemplateParameters> {
        Ok(EtherealTermTemplateParameters {
            data: template_parameters
                .iter()
                .map(|template_parameter| {
                    EtherealTemplateParameter::from_declarative(db, template_parameter)
                })
                .collect::<EtherealTermResult<_>>()?,
        })
    }

    #[inline(always)]
    pub fn data(&self) -> &[EtherealTemplateParameter] {
        &self.data
    }

    /// returns an empty partial instantiation
    pub fn empty_instantiation_builder(&self) -> EtherealInstantiationBuilder {
        EtherealInstantiationBuilder::new(self.iter().map(|param| param.symbol()))
    }
}

impl std::ops::Deref for EtherealTermTemplateParameters {
    type Target = [EtherealTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTemplateParameter {
    annotated_variance: Option<Variance>,
    symbol: EtherealTermSymbol,
    traits: Vec<EtherealTerm>,
}

impl EtherealTemplateParameter {
    fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_generic_paramter: &DeclarativeTemplateParameter,
    ) -> EtherealTermResult<Self> {
        Ok(Self {
            annotated_variance: declarative_generic_paramter.annotated_variance(),
            symbol: EtherealTermSymbol::from_declarative(
                db,
                declarative_generic_paramter.symbol(),
            )?,
            traits: declarative_generic_paramter
                .traits()
                .iter()
                .map(|_| todo!())
                .collect(),
        })
    }

    pub fn symbol(&self) -> EtherealTermSymbol {
        self.symbol
    }

    pub fn traits(&self) -> &[EtherealTerm] {
        self.traits.as_ref()
    }
}
