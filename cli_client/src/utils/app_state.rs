#[allow(dead_code)]
pub mod client_state {
    // private static mutable variable
    static mut CLIENT_CONN: Option<String> = None;
    static mut LOCATION: Option<String> = None;

    pub fn get_client_conn() -> &'static mut Option<String> {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            return &mut CLIENT_CONN;
        }
    }

    pub fn set_client_conn(client_conn: String) {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            let _ = CLIENT_CONN.insert(client_conn);
        }
    }

    pub fn get_location() -> &'static mut Option<String> {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            return &mut LOCATION;
        }
    }

    pub fn set_location(location: String) {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            let _ = LOCATION.insert(location);
        }
    }
}