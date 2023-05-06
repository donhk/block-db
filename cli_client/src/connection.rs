///
/// Initializes the connection to a given server
/// # Arguments
/// * `url` a url to connect
pub fn connect(url: &str) {
    let url_parts = url.split(' ').collect::<Vec<&str>>();
    if url_parts.get(1).is_none() {
        println!("Provide a server!");
        return;
    }
    let server = url_parts.get(1).unwrap();
    println!("Connecting to server '{}'", server);
}