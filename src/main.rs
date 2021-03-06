use geforcedrvchk3::{get_available_version_information,
                     get_page,
                     get_installed_version,
                     start_browser,
                     ask_confirmation,
                     auto_install,
                     VERSION};
use std::io::{stdin, stdout, Write};

fn handle_error<T>(result: Result<T, String>) -> T {
    let mut input = String::new();

    match result {
        Ok(value) => value,
        Err(value) => {
            println!("{}", value);
            print!("\nPress Enter...");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            std::process::exit(1);
        },
    }
}

fn main() {
    let installed: String = handle_error(get_installed_version());
    let available: (String, String) = handle_error(get_available_version_information(get_page));

    let instd_ver: f64 = handle_error(installed.parse().or(Err("Cannot convert installed version number!".to_string())));
    let avail_ver: f64 = handle_error(available.0.parse().or(Err("Cannot convert available version number!".to_string())));
    let avail_url: String = available.1;

    println!("Display Driver Check version {}\n", VERSION);

    println!("Currently installed driver version: {}", instd_ver);

    if instd_ver < avail_ver {
        println!("New driver version is available:    {}\n", avail_ver);
        match ask_confirmation("Do you want to:\n  \
                                (d)ownload the latest driver,\n  \
                                (a)utomatically install and reboot or\n  \
                                (q)uit?", &vec!['d', 'a', 'q'], 0) {
            0 => start_browser(&avail_url),
            1 => auto_install(&avail_url),
            _ => (),
        }
    }
}
