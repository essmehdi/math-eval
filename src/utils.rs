pub fn exit_with_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}