pub fn panic_location(proc_macro_name: &'static str) {
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Some(location) = panic_info.location() {
            eprintln!("{proc_macro_name} panic occurred in {}:{}:{}", location.file(), location.line(), location.column());
        } else {
            eprintln!("{proc_macro_name} panic occurred but can't get location information...");
        }
    }));
}