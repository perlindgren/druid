use std::sync::Arc;

use super::clipboard::Clipboard;

pub struct Application;

impl Application {
    pub fn init() {
        println!("application init");
        // No-op.
    }

    pub fn quit() {
        // No-op.
    }

    pub fn get_connection() -> Arc<xcb::Connection> {
        XCB_CONNECTION.connection.clone()
    }

    pub fn get_screen_num() -> i32 {
        XCB_CONNECTION.screen_num
    }

    pub fn clipboard() -> Clipboard {
        // TODO(x11/clipboard): implement Application::clipboard
        unimplemented!();
    }

    pub fn get_locale() -> String {
        // TODO(x11/locales): implement Application::get_locale
        "en-US".into()
    }
}

struct XcbConnection {
    connection: Arc<xcb::Connection>,
    screen_num: i32,
}

impl XcbConnection {
    fn new() -> Self {
        let (mut conn, screen_num) = xcb::Connection::connect_with_xlib_display().unwrap();

        // Various setup stuff for the connection.

        // Need to set up the randr query version, or else querying randr for certain screen
        // information (refresh rate, etc) won't work correctly.
        unsafe {
            xcb::ffi::randr::xcb_randr_query_version(conn.get_raw_conn(), 1, 1);
        }

        Self {
            connection: Arc::new(conn),
            screen_num,
        }
    }
}

lazy_static! {
    static ref XCB_CONNECTION: XcbConnection = XcbConnection::new();
}