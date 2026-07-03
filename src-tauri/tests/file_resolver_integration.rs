fn extract(s: &str, pos: usize) -> (String, usize) {
    let chars: Vec<char> = s.chars().collect();
    let mut i = pos;
    let token = app_lib::file_resolver::extract_path_token(&chars, &mut i);
    (token, i)
}

#[test]
fn tilde_path_consumed() {
    let (t, end) = extract("~/Documents/file.txt", 0);
    assert_eq!(t, "~/Documents/file.txt");
    assert_eq!(end, 20);
}

#[test]
fn absolute_path_consumed() {
    let (t, end) = extract("/usr/local/bin/foo", 0);
    assert_eq!(t, "/usr/local/bin/foo");
    assert_eq!(end, 18);
}

#[test]
fn relative_dot_path_consumed() {
    let (t, end) = extract("./src/main.rs", 0);
    assert_eq!(t, "./src/main.rs");
    assert_eq!(end, 13);
}

#[test]
fn parent_dotdot_path_consumed() {
    let (t, end) = extract("../sibling/file.txt", 0);
    assert_eq!(t, "../sibling/file.txt");
    assert_eq!(end, 19);
}

#[test]
fn stops_at_space() {
    let (t, end) = extract("~/Documents/file.txt rest", 0);
    assert_eq!(t, "~/Documents/file.txt");
    assert_eq!(end, 20);
    assert_eq!(end, "~/Documents/file.txt".len());
}

#[test]
fn stops_at_punctuation() {
    let (t, _) = extract("path/to/file.txt,more", 0);
    assert_eq!(t, "path/to/file.txt");
}

#[test]
fn alphanumeric_word() {
    let (t, _) = extract("hello", 0);
    assert_eq!(t, "hello");
}

#[test]
fn empty_input() {
    let (t, _) = extract("", 0);
    assert_eq!(t, "");
}

#[test]
fn stops_immediately_at_special() {
    let (t, end) = extract("/path,more", 0);
    assert_eq!(t, "/path");
    assert_eq!(end, 5);
}
