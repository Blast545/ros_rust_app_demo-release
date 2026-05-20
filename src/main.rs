fn main() {
    println!("ros_rust_app_demo starting");
    // Calling into A's library proves the link succeeded.
    println!("{}", ros_rust_lib_demo::greeting());
}
