use crate::*;

const TEST_FILE: &str = "cookie_log.csv";

#[test]
fn cookie_creation() {
    let cookie_log = CookieLog::new(
        "AtY0laUfhglK3lC7".to_string(),
        "2018-12-09T14:19:00+00:00".to_string(),
    );
    assert_eq!(cookie_log.cookie, "AtY0laUfhglK3lC7");
    assert_eq!(cookie_log.utc_time, "2018-12-09T14:19:00+00:00");
}

#[test]
fn cookie_date_retrieval() {
    let cookie_log = CookieLog::new(
        "AtY0laUfhglK3lC7".to_string(),
        "2018-12-09T14:19:00+00:00".to_string(),
    );

    assert_eq!(cookie_log.get_date(), "2018-12-09")
}

#[test]
fn cookie_filter_on_date() {
    let cookie_logs = get_cookie_logs(open_csv_file(TEST_FILE).unwrap());

    let filtered_cookie_logs = get_cookies_on_date(cookie_logs, &"2018-12-08".to_string());

    assert_eq!(filtered_cookie_logs.len(), 3);
    assert_eq!(filtered_cookie_logs[0].get_date(), "2018-12-08");
}

#[test]
fn most_active_cookies_overall() {
    let cookie_logs = get_cookie_logs(open_csv_file(TEST_FILE).unwrap());

    let most_active_cookie = most_active_cookies(cookie_logs);

    assert!(most_active_cookie.contains(&"SAZuXPGUrfbcn5UA".to_string()));
    assert!(most_active_cookie.contains(&"AtY0laUfhglK3lC7".to_string()));
    assert!(most_active_cookie.contains(&"4sMM2LxV07bPJzwf".to_string()));
}

#[test]
fn most_active_cookies_on_date() {
    let cookie_logs = get_cookie_logs(open_csv_file(TEST_FILE).unwrap());

    let cookies_on_date = get_cookies_on_date(cookie_logs, &"2018-12-07".to_string());

    let most_active_cookie = most_active_cookies(cookies_on_date);

    assert!(most_active_cookie.contains(&"4sMM2LxV07bPJzwf".to_string()));
}
