use geforcedrvchk3::{get_available_version_information,
                     get_installed_version,
                     start_browser,
                     handle_error,
                     ask_confirmation};

const VERSION: &str = "0.1";

fn main() {
    let installed: String = handle_error(get_installed_version());
    let available: (String, String) = handle_error(get_available_version_information());

    let instd_ver: f64 = handle_error(installed.parse().or(Err(String::from("Cannot convert installed version number!"))));
    let avail_ver: f64 = handle_error(available.0.parse().or(Err(String::from("Cannot convert available version number!"))));
    let avail_url: String = available.1;

    println!("NVIDIA GeForce Driver Check v{}\n", VERSION);

    println!("Currently installed driver version: {}", instd_ver);

    if instd_ver < avail_ver {
        println!("New driver version is available:    {}\n", avail_ver);
        if ask_confirmation("Do you want to download the latest driver?", &vec!['y', 'n'], 0) == 0 {
            start_browser(&avail_url);
        }
    }
}
