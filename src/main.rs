fn main() {
    println!("Starting");
    log::set_max_level(log::Level::max().to_level_filter());
    println!("Crashing....");
    let _ = local_ip_address::local_ip().map(|h| h.to_string());
    println!("Or not!");
}