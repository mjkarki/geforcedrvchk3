//! This library provides tools for querying NVIDIA GeForce graphics driver
//! version information from the installed driver and from the driver
//! download page.
//!
//! The library fetches information for GTX 1070 Ti card for 64-bit Windows
//! operating system. The driver should be the same for other modern NVIDIA
//! cards.
//!
//! The page this library is using for fetching information is this:
//! <https://www.geforce.com/drivers>

use std::{env, path::Path, path::PathBuf, process::Command};
use regex::Regex;
use json;
use reqwest::blocking;

use std::io::Write;             // Just for flush()
use std::io::{stdin, stdout};

pub const VERSION: &str = "0.5.1";
pub const SMI: &str = r"nvidia-smi.exe";
const NVIDIA_URL: &str = r"https://gfwsl.geforce.com/services_toolkit/services/com/nvidia/services/AjaxDriverService.php?func=DriverManualLookup&psid=101&pfid=859&osID=57&languageCode=1033&beta=0&isWHQL=0&dltype=-1&dch=1&upCRD=0&qnf=0&sort1=0&numberOfResults=10";

/// Fetches contents of the URL and returns them as a string. It is assumed
/// that the contents are UTF-8 encoded.
///
/// If there is an error, then an error message is returned as a result.
pub fn get_page(url: &str) -> Result<String, &'static str> {
    let response = blocking::get(url);
    match response {
        Ok(resp) => resp.text().or(Err("The page has invalid UTF-8 characters!")),
        Err(_) => Err("Unable to access the online resources!"),
    }
}

/// Retrieves the latest available driver installation package version number
/// and a download URL as a tuple. The version number should be formatted as
/// "XXX.YY", so it should be possible to convert it to a double.
///
/// Takes as an argument a function that is able to retrieve data from the server and
/// return is as a string (JSON). Just use get_page() here.
///
/// If the information cannot be retrieved, then an error message is provided
/// as a result.
pub fn get_available_version_information(get_page: fn (&str) -> Result<String, &'static str>) -> Result<(String, String), &'static str> {
    let page = get_page(NVIDIA_URL)?;
    let data = json::parse(&page).or(Err("Incorrect information at the online resource!"))?;
    let json_version = &data["IDS"][0]["downloadInfo"]["Version"];
    let json_url = &data["IDS"][0]["downloadInfo"]["DownloadURL"];
    let version = json_version.as_str().ok_or("Cannot find version information from the online resource!")?;
    let url = json_url.as_str().ok_or("Cannot find download URL information from the online resource!")?;
    Ok((version.to_string(), url.to_string()))
}

/// Retrieves installed display driver version as a string. The version number
/// should be formatted as "XXX.YY", so it should be possible to convert it to
/// a double.
///
/// If the version number is not available (e.g. nvidia-smi.exe could not be
/// found), then an error message is provided as a result.
pub fn get_installed_version(executable_name: &str) -> Result<String, &'static str> {
    let nvidiasmi = get_nvidia_smi_location(&executable_name)?;
    let output = Command::new(nvidiasmi).output().or(Err("Couldn't detect installed version. Maybe the driver is not installed?"))?;
    let pattern = Regex::new(r"Driver Version: ([0-9]+\.[0-9]+)").unwrap();
    let nvsmi = String::from_utf8_lossy(&output.stdout);
    let captures = pattern.captures(&nvsmi).ok_or("Cannot find installed version information!")?;
    Ok(String::from(&captures[1]))
}

/// Find nvidia-smi.exe and return full path.
fn get_nvidia_smi_location(executable_name: &str) -> Result<String, &'static str> {
    let nvidia_smi_path_old: PathBuf = ["NVIDIA Corporation", "NVSMI", &executable_name].iter().collect();
    let nvidia_smi_path_new: PathBuf = ["System32", &executable_name].iter().collect();
    let mut nvidiasmi = PathBuf::new();
    nvidiasmi.push(env::var("windir").expect("Environment variable 'windir' not found!"));
    nvidiasmi.extend(&nvidia_smi_path_new);
    if Path::new(&nvidiasmi).exists() == false {
        let mut nvidiasmi = PathBuf::new();
        nvidiasmi.push(env::var("ProgramFiles").expect("Environment variable 'ProgramFiles' not found!"));
        nvidiasmi.extend(&nvidia_smi_path_old);
        if nvidiasmi.exists() == false {
            Err("Couldn't detect location for nvidia-smi. Maybe the driver is not installed?")
        }
        else {
            Ok(String::from(nvidiasmi.to_string_lossy()))
        }
    }
    else {
        Ok(String::from(nvidiasmi.to_string_lossy()))
    }
}

/// Starts the default web browser if a valid URL is given. Note that the
/// operation is executed simply by calling "start" command at the
/// command-line and the URL is not sanitized in any way. It's possible to run
/// arbitrary commands with this function.
pub fn start_browser(url: &str) {
    Command::new(env::var("ComSpec").expect("Environment variable 'ComSpec' not found!")).arg("/c").arg("start").arg(url).spawn().unwrap();
}

/// Asks message from user and lists options. The default option is zero-based
/// index pointing to the item in the options list. The default option is
/// displayed in brackets and is selected if user presses Enter without
/// selecting any choice. If user selects an option, which is not listed in
/// options, then the question is repeated. User can enter several characters
/// to the input field, but only the first character is counted as a selection.
///
/// Input is not case-sensitive.
///
/// Example:
///
/// ``ask_confirmation("Are you sure?", ['y', 'n'], 1)``
///
/// Displays:
///
/// ``Are you sure? (y,n)[n]``
///
/// The return value is the index of the selection based on the options list.
pub fn ask_confirmation(message: &str, options: &[char], default: usize) -> usize {
    let mut input = String::new();

    loop {
        print!("{message} (");
        for option in options {
            print!("{option}");
            if Some(option) != options.last() {
                print!(",");
            }
        }
        print!(")[{}] ", options[default]);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        if input.trim().len() == 0 {
            break default;
        }
        else {
            let pos = &options.iter().position(|&x| x.to_lowercase().next() == input.chars().next().unwrap().to_lowercase().next());
            match pos {
                Some(value) => break *value,
                None => input.clear(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that get_page() is able to fetch a web page via http connection.
    #[test]
    fn get_page_success() {
        assert_eq!(get_page("http://example.com/").is_ok(), true);
    }

    /// Test that get_page() is able to fetch a web page via https connection.
    #[test]
    fn get_page_ssl_success() {
        assert_eq!(get_page("https://example.com/").is_ok(), true);
    }

    /// Test that get_page() handles non-existent URL correctly.
    #[test]
    fn get_page_fail() {
        assert_eq!(get_page("http://nonexistingdomain.local/").is_err(), true);
    }

    /// Test that fetching installed driver version works.
    /// This test requires that display drivers are installed.
    #[test]
    fn get_installed_version_success() {
        std::env::set_var("windir", ".");
        std::env::set_var("ProgramFiles", ".");
        assert_eq!(get_installed_version("smi-stub.bat").unwrap(), "123.45");
    }

    /// Test that fetching available driver data works.
    #[test]
    fn get_available_version_information_success() {
        assert_eq!(get_available_version_information(get_test_page).is_ok(), true);
    }

    /// Test that fetching available driver version works.
    #[test]
    fn get_available_version_number_success() {
        assert_eq!(get_available_version_information(get_test_page).unwrap().0, "123.45");
    }

    /// Test that fetching available driver URL works.
    #[test]
    fn get_available_version_url_success() {
        assert_eq!(get_available_version_information(get_test_page).unwrap().1, "https://example.com/test.exe");
    }

    /// Stub function for unit tests. Imitates get_page() function.
    fn get_test_page(_url: &str) -> Result<String, &'static str> {
        let json = r#"{ "Success" : "1", "IDS" : [ { "downloadInfo": { "Version" : "123.45", "DownloadURL" : "https://example.com/test.exe" } } ] }"#;
        Ok(json.to_string())
    }
}
