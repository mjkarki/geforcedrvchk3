use geforcedrvchk3::{get_available_version_information,
                     get_page,
                     get_installed_version,
                     start_browser,
                     ask_confirmation,
                     VERSION,
                     SMI};
use std::io::{stdin, stdout, Write};
use curl::Version;

fn handle_error<T>(result: Result<T, String>) -> T {
    let mut input = String::new();

    match result {
        Ok(value) => value,
        Err(value) => {
            println!("{value}");
            print!("\nPress Enter...");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            std::process::exit(1);
        },
    }
}

fn main() {
    println!("Display Driver Check version {VERSION}");
    println!("{}\n", Version::num());

    let installed: String = handle_error(get_installed_version(SMI));
    let available: (String, String) = handle_error(get_available_version_information(get_page));

    let instd_ver: f64 = handle_error(installed.parse().or(Err("Cannot convert installed version number!".to_string())));
    let avail_ver: f64 = handle_error(available.0.parse().or(Err("Cannot convert available version number!".to_string())));
    let avail_url: String = available.1;

    println!("Currently installed driver version: {instd_ver}");

    if instd_ver == avail_ver {
        println!("New driver version is available:    {avail_ver}\n");
        match ask_confirmation("Do you want to \
                                (d)ownload the latest driver, or \
                                (q)uit?", &vec!['d', 'q'], 0) {
            0 => start_browser(&avail_url),
            _ => (),
        }
    }
}
