/// A `warp` test server that spawns on another thread and can be manually shut
/// down.
pub struct TestServer {
    handle: tokio::task::JoinHandle<()>,
    close: tokio::sync::oneshot::Sender<()>,
}

impl TestServer {
    /// Create a new `warp` server from route filters on a new thread
    pub fn serve<F, A>(filter: F, addr: A) -> Self
    where
        A: Into<std::net::SocketAddr> + 'static,
        F: warp::Filter + Clone + Send + Sync + 'static,
        F::Extract: warp::Reply,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let (_addr, server) = warp::serve(filter).bind_with_graceful_shutdown(addr, async {
            rx.await.ok();
        });

        let handle = tokio::task::spawn(server);

        Self { handle, close: tx }
    }

    /// Tell the server to shutdown and wait for its thread to close.
    pub async fn shutdown(self) {
        let Self { handle, close } = self;
        close.send(()).expect("failed to command server shutdown");
        handle.await.expect("failed to close server thread");
    }
}
