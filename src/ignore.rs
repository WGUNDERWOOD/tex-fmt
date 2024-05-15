pub fn is_ignored(line: &str) -> bool {
    line.ends_with("%tex-fmt-ignore-line")
}
