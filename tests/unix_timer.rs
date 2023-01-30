#![cfg(all(unix, feature = "os-poll", feature = "os-ext", feature = "net"))]
use mio::unix::Timer;
use mio::{Interest, Token};
use std::time::Duration;

mod util;
use util::{
    assert_send, assert_socket_close_on_exec, assert_socket_non_blocking, assert_sync,
    assert_would_block, expect_events, expect_no_events, init, init_with_poll, temp_file,
    ExpectEvent, Readiness,
};

#[test]
fn unix_timer_register() {
    let (mut poll, mut events) = init_with_poll();

    let mut timer = Timer::new(Duration::new(5, 0)).unwrap();

    poll.registry()
        .register(&mut timer, Token(0), Interest::READABLE)
        .unwrap();
}
