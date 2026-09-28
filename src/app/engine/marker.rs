/// Extracts option name out of a `{<marker><option>}` line, if/ifnot it has one.
///
/// The marker is matched anywhere in the line, so it can sit behind whatever
/// comment syntax the template's file format uses.
pub(super) fn marker<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let start = line.find(marker)? + marker.len();
    let end = start + line[start..].find('}')?;
    Some(line[start..end].trim())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::behind_a_hash("# {if:just}", "{if:", Some("just"))]
    #[case::behind_slashes("// {slot:rust.lib.mods}", "{slot:", Some("rust.lib.mods"))]
    #[case::inside_an_html_comment("<!-- {endif:just} -->", "{endif:", Some("just"))]
    #[case::spaces_trimmed("# {if: just }", "{if:", Some("just"))]
    #[case::no_marker("cargo check --all-targets", "{if:", None)]
    #[case::never_closed("# {if:just", "{if:", None)]
    #[case::ifnot_is_not_if("# {ifnot:just}", "{if:", None)]
    #[case::endif_is_not_if("# {endif:just}", "{if:", None)]
    fn reads_the_name_out_of_a_marker_line(
        #[case] line: &str,
        #[case] kind: &str,
        #[case] expected: Option<&str>,
    ) {
        assert_eq!(marker(line, kind), expected);
    }
}
