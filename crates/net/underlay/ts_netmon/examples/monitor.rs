//! Log all changes captured by netmon.
//!
//! Pass `--polling` to use [`ts_netmon::PollingMon`] (Unix only).

use std::io;

use tokio_stream::StreamExt;
use ts_netmon::Netmon;

#[tokio::main]
async fn main() -> io::Result<()> {
    ts_cli_util::init_tracing();

    let mon: Box<dyn Netmon> = match std::env::args().nth(1).as_deref() {
        #[cfg(unix)]
        Some("--polling") => Box::new(ts_netmon::PollingMon::default()),
        Some(_) => return Err(io::ErrorKind::InvalidInput.into()),

        // Default cfg uses platform-specific implementation
        None => Box::new(ts_netmon::platform_mon().ok_or(io::ErrorKind::Unsupported)?),
    };
    tracing::info!(ty = %mon.ty(), "monitoring");

    let stream = mon.with_default_route_events()?;

    let mut stream = core::pin::pin![stream];

    while let Some(evt) = stream.next().await {
        tracing::info!(evt = ?evt?);
    }

    tracing::warn!("stream ended");

    Ok(())
}
