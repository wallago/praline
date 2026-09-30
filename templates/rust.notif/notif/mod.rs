pub(super) async fn run(source: tokio::sync::broadcast::Receiver<String>) {
    while let Ok(msg) = source.recv().await {
        if let Err(err) = notify_rust::Notification::new()
                .summary(summary)
                .body(body)
                .icon("dialog-information")
                .timeout(5000) // ms
                .show_async()
                .await
        {
            tracing::warn!("Notification failed: {err}");
        }
    }
}
