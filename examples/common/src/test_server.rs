use std::{convert::Infallible, sync::mpsc};
use tokio::{sync::oneshot, task::JoinHandle};

/// A `warp` test server that spawns on another thread and can be manually shut
/// down.
pub struct TestServer {
    handle: JoinHandle<()>,
    close: oneshot::Sender<()>,
}

impl TestServer {
    /// Create a new `warp` server from route filters on a new thread
    pub fn serve<F, A>(filter: F, addr: A) -> Self
    where
        A: Into<std::net::SocketAddr> + 'static,
        F: warp::Filter<Error = Infallible> + Clone + Send + Sync + 'static,
        F::Extract: warp::Reply,
    {
        let (tx, rx) = oneshot::channel();
        let (_addr, server) = warp::serve(filter).bind_with_graceful_shutdown(addr, async {
            rx.await.ok();
        });

        let handle = tokio::task::spawn(server);

        Self { handle, close: tx }
    }

    pub fn serve_once<F, R, A>(filter: F, addr: A) -> JoinHandle<()>
    where
        A: Into<std::net::SocketAddr> + 'static,
        F: warp::Filter<Extract = (R,), Error = Infallible> + Clone + Send + Sync + 'static,
        F::Extract: warp::Reply,
    {
        let (tx, rx) = mpsc::sync_channel::<()>(1);
        let (_addr, server) = warp::serve(filter.with(warp::wrap_fn(move |f: F| {
            let tx = tx.clone();
            f.map(move |reply| {
                tx.clone()
                    .send(())
                    .expect("failed to command server to shutdown");
                reply
            })
        })))
        .bind_with_graceful_shutdown(addr, async move {
            rx.recv().ok();
        });

        tokio::task::spawn(server)
    }

    /// Tell the server to shutdown and wait for its thread to close.
    pub async fn shutdown(self) {
        let Self { handle, close } = self;
        close.send(()).expect("failed to command server shutdown");
        handle.await.expect("failed to close server thread");
    }
}
