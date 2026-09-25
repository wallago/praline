/// Extracts option name out of a `{<marker><option>}` line, if/ifnot it has one.
///
/// The marker is matched anywhere in the line, so it can sit behind whatever
/// comment syntax the template's file format uses.
pub(super) fn marker<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let start = line.find(marker)? + marker.len();
    let end = start + line[start..].find('}')?;
    Some(line[start..end].trim())
}
