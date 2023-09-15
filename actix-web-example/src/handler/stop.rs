use actix_web::{dev::ServerHandle, post, web, HttpResponse};
use actix_web_lab::extract::Path;
use parking_lot::Mutex;

#[post("/stop/{graceful}")]
pub async fn stop(Path(graceful): Path<bool>, stop_handle: web::Data<StopHandle>) -> HttpResponse {
    info!("graceful: {:?}", graceful);
    stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

#[derive(Default)]
pub struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        #[allow(clippy::let_underscore_future)]
            let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}
