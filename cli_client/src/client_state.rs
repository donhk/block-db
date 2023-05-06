pub mod client_state {
    // private static mutable variable
    static mut INSTANCE: Option<String> = None;

    pub fn get_instance() -> &'static mut Option<String> {
        unsafe {
            // use lazy initialization to create the instance if it doesn't exist yet
            INSTANCE.get_or_insert_with(|| String::from("Hello, world!"));
            &mut INSTANCE
        }
    }
}