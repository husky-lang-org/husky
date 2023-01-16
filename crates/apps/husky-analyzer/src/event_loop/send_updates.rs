use crate::server::client_comm::ClientCommunicator;
use crate::*;
use husky_vfs::VfsDb;
use salsa::DebugWithDb;
use vec_like::VecSet;

pub(crate) fn send_updates(db: &AnalyzerDB, comm: &ClientCommunicator) {
    let live_packages = db.live_packages().unwrap();
    eprintln!(
        "send updates for live packages {:?}",
        &((&live_packages as &VecSet<_>).debug(db))
    );
    for package_path in live_packages.iter() {
        let collect_probable_modules = db.collect_probable_modules(*package_path);
        for module_path in collect_probable_modules {
            comm.send_diagnostics(db, module_path)
        }
    }
}
