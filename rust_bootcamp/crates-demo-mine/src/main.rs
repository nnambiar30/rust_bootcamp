mod config;
mod data;
mod services;
use services::processor::process_data;


fn main() {
    println!("Starting app!");
    process_data();
}
