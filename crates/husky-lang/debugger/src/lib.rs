mod error;
mod gui;
pub mod mock;
mod notif;
mod state;
#[cfg(test)]
mod tests;

use std::{convert::Infallible, net::ToSocketAddrs, sync::Arc};

use common::HashMap;
pub use error::{DebuggerError, DebuggerResult};

use husky_lang_compile_time::HuskyLangCompileTime;
use husky_lang_runtime::{HuskyLangRuntime, RuntimeQueryGroup};

use futures::executor::ThreadPool;
use gui::handle_query;
use notif::handle_notif;
use state::DebuggerState;
use std::sync::Mutex;
use stdx::sync::ARwLock;
use trace::{AllocateTrace, FigureProps, Trace, TraceId, TraceStalk};
use warp::Filter;

pub struct Debugger {
    runtime: Mutex<HuskyLangRuntime>,
    threadpool: ThreadPool,
    state: Mutex<DebuggerState>,
}

impl Debugger {
    pub fn new(init_compile_time: impl FnOnce(&mut HuskyLangCompileTime)) -> Self {
        Self {
            runtime: Mutex::new(HuskyLangRuntime::new(init_compile_time)),
            threadpool: ThreadPool::new().unwrap(),
            state: Default::default(),
        }
    }

    pub async fn serve(self, addr: impl ToSocketAddrs) -> DebuggerResult<()> {
        let debugger = Arc::new(self);
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        println!(
            "{}husky{}: serve on {:?}",
            common::show::CYAN,
            common::show::RESET,
            addr
        );
        let notif = warp::path!("notif")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_notif);
        let query = warp::path!("query")
            .and(warp::ws())
            .and(with_debugger(debugger.clone()))
            .and_then(handle_query);
        let routes = notif.or(query);
        warp::serve(routes).run(addr).await;
        Ok(())
    }

    pub fn change_text(&self) {}

    pub fn root_traces(&self) -> Arc<Vec<Arc<Trace>>> {
        self.runtime.lock().unwrap().root_traces()
    }

    pub async fn subtraces(
        &self,
        trace_id: TraceId,
        opt_input_id: Option<usize>,
    ) -> Arc<Vec<Arc<Trace>>> {
        self.runtime
            .lock()
            .unwrap()
            .subtraces(trace_id, opt_input_id)
    }

    pub fn expansions(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().expansions()
    }

    pub fn showns(&self) -> HashMap<TraceId, bool> {
        self.runtime.lock().unwrap().showns()
    }

    pub async fn figure(&self, id: TraceId) -> Option<FigureProps> {
        self.runtime.lock().unwrap().figure(id)
    }

    pub async fn activate(&self, id: TraceId) {
        self.state.lock().unwrap().active_trace_id = Some(id);
    }

    pub async fn toggle_expansion(&self, id: TraceId) {
        self.runtime.lock().unwrap().toggle_expansion(id)
    }

    pub async fn toggle_show(&self, id: TraceId) {
        self.runtime.lock().unwrap().toggle_show(id)
    }

    pub async fn trace(&self, id: TraceId) -> Arc<Trace> {
        self.runtime.lock().unwrap().trace(id)
    }

    pub async fn lock_input(&self, input_str: String) -> (Option<Option<usize>>, Option<String>) {
        self.runtime.lock().unwrap().lock_input(input_str)
    }

    pub async fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk> {
        self.runtime.lock().unwrap().trace_stalk(trace_id, input_id)
    }

    pub fn input_id(&self) -> Option<usize> {
        self.runtime.lock().unwrap().input_id()
    }
}

fn with_debugger(
    debugger: Arc<Debugger>,
) -> impl Filter<Extract = (Arc<Debugger>,), Error = Infallible> + Clone {
    warp::any().map(move || debugger.clone())
}
