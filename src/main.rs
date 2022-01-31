use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod tests;

/// Struct that represents a cookie log
/// # Fields
/// * `cookie` - The cookie string
/// * `utc_time` - UTC timestamp of the cookie
#[derive(Debug)]
struct CookieLog {
    cookie: String,
    utc_time: String,
}

impl CookieLog {
    /// Creates a new CookieLog struct
    fn new(cookie: String, utc_time: String) -> CookieLog {
        CookieLog { cookie, utc_time }
    }

    /// Returns the date of the cookie log
    fn get_date(&self) -> String {
        let mut day = self.utc_time.clone();
        day.truncate(10);
        day
    }
}

/// Opens newline-delimited csv file and returns the result as a vec of strings
/// # Arguments
/// * `file_name` - The name of the file to open
/// # Returns
/// * `Vec<String>` - The contents of the file
fn open_csv_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    // Read the file contents into a string
    file.read_to_string(&mut contents)?;

    // Split the string on newlines, converting each line to an owned String
    let mut lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    // Remove header
    lines.remove(0);

    Ok(lines)
}

/// Parses vec of strings into a vec of CookieLog structs
/// # Arguments
/// * `lines` - The vec of strings to parse
/// # Returns
/// * `Vec<CookieLog>` - The parsed vec of CookieLog structs
fn get_cookie_logs(lines: Vec<String>) -> Vec<CookieLog> {
    let mut cookie_logs: Vec<CookieLog> = Vec::new();

    for line in lines {
        let mut split_line = line.split(",");
        let cookie = split_line.next().unwrap();
        let utc_time = split_line.next().unwrap();

        cookie_logs.push(CookieLog::new(cookie.to_string(), utc_time.to_string()));
    }

    cookie_logs
}

/// Filters the vec of CookieLog structs by the given date
/// # Arguments
/// * `cookie_logs` - The vec of CookieLog structs to filter
/// * `date` - The date to filter by
/// # Returns
/// * `Vec<CookieLog>` - The filtered vec of CookieLog structs
fn get_cookies_on_date(cookies: Vec<CookieLog>, date: &String) -> Vec<CookieLog> {
    let mut cookies_on_date: Vec<CookieLog> = Vec::new();

    for cookie in cookies {
        if &cookie.get_date() == date {
            cookies_on_date.push(cookie);
        }
    }

    cookies_on_date
}

/// Returns the most active cookie(s) in a list
/// # Arguments
/// * `cookie_logs` - The vec of CookieLog structs to filter
/// # Returns
/// * `Vec<String>` - The most active cookie(s)
fn most_active_cookies(cookies: Vec<CookieLog>) -> Vec<String> {
    let mut cookie_counts: HashMap<String, usize> = HashMap::new();

    // Count each cookie reference
    for cookie in cookies {
        let count = cookie_counts.entry(cookie.cookie).or_insert(0);
        *count += 1;
    }

    // Get the cookie with the most references
    let mut most_active_cookies: Vec<String> = Vec::new();
    let mut max_count = 0;

    // Loop over all cookie counts
    for (cookie, count) in cookie_counts {
        // If the count is greater than the current max, set the max to the new count
        if count > max_count {
            max_count = count;
            most_active_cookies.clear();
            most_active_cookies.push(cookie);
        } else if count == max_count {
            // If the count is equal to the current max, add the cookie to the list
            most_active_cookies.push(cookie);
        }
    }

    most_active_cookies

}

fn main() -> Result<(), Box<dyn Error>> {
    // First, parse the command line arguments
    let matches = App::new("Cookie Log")
        .version("0.1")
        .author("Ethan Vazquez")
        .about("A simple program to parse csv cookie logs")
        .arg(
            Arg::new("file")
                .index(1)
                .value_name("FILE")
                .help("Sets the file to parse")
                .takes_value(true),
        )
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .help("Takes UTC date as input")
                .takes_value(true),
        );

    let matches = matches.get_matches();

    // Get the file name from the command line arguments
    let file_name = matches.value_of("file");
    let date_to_check = matches.value_of("date");

    // If the file name is not specified, print an error and exit
    if file_name.is_some() && date_to_check.is_some() {
        // Unwrap date
        let date = date_to_check.unwrap().to_string();
        
        // Open the file
        let lines = open_csv_file(file_name.unwrap())?;

        // Get the cookie logs
        let cookie_logs = get_cookie_logs(lines);

        // Get the cookies on the date
        let cookies_on_date = get_cookies_on_date(cookie_logs, &date);

        // Get the most active cookies on the date
        let most_active_cookies = most_active_cookies(cookies_on_date);

        // Print the most active cookies
        for cookie in most_active_cookies {
            println!("{}", cookie);
        }

    } else {
        println!("Invalid argument(s)!");
    }

    Ok(())
}
