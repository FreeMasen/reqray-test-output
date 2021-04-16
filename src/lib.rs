use tracing::{info, instrument};

#[instrument]
fn repeated(repetition: i32) {
    info!("repetition: {}", repetition);
}

#[instrument]
fn random() {}

#[instrument]
fn nested() {
    random();
    for i in 1..=1000 {
        repeated(i);
    }
}

#[instrument]
fn nested2() {
    random();
    for i in 1..=1000 {
        repeated(i);
    }
    nested();
}

#[instrument]
pub fn request() {
    nested();
    repeated(-1);
    repeated(-2);
    nest_deeply(100);
    // Even though the name is the same, this is a different span.
    // let name_clash_span = info_span!("nested");
    // let _enter = name_clash_span.enter();
    nested2();
}

#[instrument]
fn nest_deeply(nest: usize) {
    if nest == 0 {
        return;
    }

    nest_deeply(nest - 1);
}

#[instrument]
pub async fn delayed() {
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    request();
}
