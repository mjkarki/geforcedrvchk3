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

use std::env;
use std::process::Command;
use regex::Regex;
use curl::easy::Easy;
use json;
use std::io::Write;             // Just for flush()
use std::io::{stdin, stdout};

const NVIDIA_SMI_PATH: &str = r"NVIDIA Corporation\NVSMI\nvidia-smi.exe";
const NVIDIA_URL: &str = r"https://gfwsl.geforce.com/services_toolkit/services/com/nvidia/services/AjaxDriverService.php?func=DriverManualLookup&psid=101&pfid=859&osID=57&languageCode=1033&beta=0&isWHQL=1&dltype=-1&sort1=0&numberOfResults=1";

/// Fetches contents of the URL and returns them as a string. It is assumed
/// that the contents are UTF-8 encoded.
/// 
/// If there is an error, then an error message is returned as a result.
fn get_page(url: &str) -> Result<String, String> {
    let mut handle = Easy::new();
    let mut result_vector: Vec<u8> = Vec::new();
    
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            result_vector.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().or(Err("Unable to access the online resources!"))?;
    }

    let result = String::from_utf8(result_vector).or(Err("The page has invalid UTF-8 characters!"))?;
    Ok(result)
}

/// Retrieves the latest available driver installation package version number
/// and a download URL as a tuple. The version number should be formatted as
/// "XXX.YY", so it should be possible to convert it to a double.
/// 
/// If the information cannot be retrieved, then an error message is provided
/// as a result.
pub fn get_available_version_information() -> Result<(String, String), String> {
    let page = get_page(NVIDIA_URL)?;
    let data = json::parse(&page).or(Err("Incorrect information at the online resource!"))?;
    let json_version = &data["IDS"][0]["downloadInfo"]["Version"];
    let json_url = &data["IDS"][0]["downloadInfo"]["DownloadURL"];
    let version = json_version.as_str().ok_or("Cannot find version information from the online resource!")?;
    let url = json_url.as_str().ok_or("Cannot find download URL information from the online resource!")?;
    Ok((String::from(version), String::from(url)))
}

/// Retrieves installed display driver version as a string. The version number
/// should be formatted as "XXX.YY", so it should be possible to convert it to
/// a double.
/// 
/// If the version number is not available (e.g. nvidia-smi.exe could not be
/// found), then an error message is provided as a result.
pub fn get_installed_version() -> Result<String, String> {
    let nvidiasmi = format!("{}\\{}", env::var("ProgramFiles").unwrap(), NVIDIA_SMI_PATH);
    let output = Command::new(nvidiasmi).output().or(Err("Cannot execute nvidia-smi.exe!"))?;
    let pattern = Regex::new(r"Driver Version: ([0-9]+\.[0-9]+)").unwrap();
    let nvsmi = String::from_utf8_lossy(&output.stdout);
    let captures = pattern.captures(&nvsmi).ok_or("Cannot find installed version information!")?;
    Ok(String::from(&captures[1]))
}

/// Starts the default web browser if a valid URL is given. Note that the
/// operation is executed simply by calling "start" command at the
/// commmand-line and the URL is not sanitized in any way. It's possible to run
/// arbitrary commands with this function.
pub fn start_browser(url: &str) {
    Command::new(env::var("ComSpec").unwrap()).arg("/c").arg("start").arg(url).spawn().unwrap();
}

/// Asks message from user and lists options. The default option is zero-based
/// index pointing to the item in the options list. The default option is
/// displayed in brackets and is selected if user presses Enter without
/// selecting any choice. If user selects an option, which is not listed in
/// options, then the question is repeated. User can enter several characters
/// to the input field, but only the first character is counted as a selection.
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
        print!("{} (", message);
        for option in options {
            print!("{}", option);
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
            let pos = &options.iter().position(|&x| x == input.chars().next().unwrap());
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

    #[test]
    fn get_page_success() {
        assert_eq!(get_page("http://example.com/").is_ok(), true);
    }

    #[test]
    fn get_page_success_ssl() {
        assert_eq!(get_page("https://example.com/").is_ok(), true);
    }

    #[test]
    fn get_page_fail() {
        assert_eq!(get_page("http://nonexistingdomain.local/").is_err(), true);
    }

    #[test]
    fn get_installed_version_test() {
        assert_eq!(get_installed_version().is_ok(), true);
    }

    #[test]
    fn get_available_version_information_test() {
        assert_eq!(get_available_version_information().is_ok(), true);
    }
}
