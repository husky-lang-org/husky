use crate::{request::SglangRequest, response::SglangResponse, *};
use disk_cache::DiskCache;
use enum_index::full_map::EnumFullVecMap;
use eterned::db::EternerDb;
use model::SglangModel;
use reqwest::Client;
use std::path::PathBuf;

pub struct SglangClient<'db> {
    caches: EnumFullVecMap<SglangModel, DiskCache<&'db EternerDb, SglangRequest, SglangResponse>>,
    client: Client,
}

impl<'db> SglangClient<'db> {
    pub fn new(db: &'db EternerDb, cache_dir: PathBuf) -> SglangResult<Self> {
        if !cache_dir.is_dir() {
            return Err(SglangError::InvalidCacheDir(cache_dir));
        }

        Ok(Self {
            caches: EnumFullVecMap::try_new(|model: SglangModel| {
                if !cache_dir.is_dir() {
                    return Err(SglangError::InvalidCacheDir(cache_dir.clone()));
                }
                DiskCache::new(db, cache_dir.join(model.as_str())).map_err(Into::into)
            })?,
            client: Client::new(),
        })
    }
}
