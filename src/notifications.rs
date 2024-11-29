use notify_rust::Notification;

pub fn send_error_notification(body: &str) {
    Notification::new()
        .summary("Computer Statistics Error")
        .body(body)
        .icon("dialog-error")
        .timeout(0)
        .show()
        .unwrap();
}