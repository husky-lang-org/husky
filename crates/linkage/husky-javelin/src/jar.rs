use crate::*;

#[salsa::jar(db = JavelinDb)]
pub struct JavelinJar(
    crate::javelin::Javelin,
    crate::version_stamp::JavelinVersionStamp,
    crate::template_argument::ty::JavelinTypePathLeading,
    crate::template_argument::ty::JavelinTypeRitchie,
    crate::amazon::package_amazon_javelins,
    crate::valkyrie::package_valkyrie_javelin_pantheon,
    crate::valkyrie::item_valkyrie_rides,
    crate::valkyrie::javelin_generated_valkyrie_javelins,
);
