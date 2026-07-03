use app_lib::cancel::StreamCancelState;

#[tokio::test]
async fn new_starts_unset() {
    let state = StreamCancelState::new();
    let rx = state.reset_and_subscribe();
    assert!(!*rx.borrow());
}

#[tokio::test]
async fn cancel_after_subscribe_sets_true() {
    let state = StreamCancelState::new();
    let mut rx = state.reset_and_subscribe();
    assert!(!*rx.borrow());
    state.cancel();
    let _ = rx.changed().await;
    assert!(*rx.borrow());
}

#[tokio::test]
async fn cancel_idempotent() {
    let state = StreamCancelState::new();
    let mut rx = state.reset_and_subscribe();
    state.cancel();
    state.cancel();
    state.cancel();
    let _ = rx.changed().await;
    assert!(*rx.borrow());
}

#[tokio::test]
async fn reset_clears_flag() {
    let state = StreamCancelState::new();
    let mut rx = state.reset_and_subscribe();
    state.cancel();
    let _ = rx.changed().await;
    assert!(*rx.borrow());
    let mut rx2 = state.reset_and_subscribe();
    assert!(!*rx2.borrow());
}

#[tokio::test]
async fn multiple_subscribers_independent() {
    let state = StreamCancelState::new();
    let mut rx1 = state.reset_and_subscribe();
    let mut rx2 = state.reset_and_subscribe();
    assert!(!*rx1.borrow());
    assert!(!*rx2.borrow());
    state.cancel();
    let _ = rx1.changed().await;
    let _ = rx2.changed().await;
    assert!(*rx1.borrow());
    assert!(*rx2.borrow());
}

#[tokio::test]
async fn late_subscriber_receives_cancellation() {
    let state = StreamCancelState::new();
    state.cancel();
    let rx = state.reset_and_subscribe();
    assert!(!*rx.borrow());
}
