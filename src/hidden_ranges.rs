use std::ops::Range;

pub type HiddenRanges = Vec<Range<usize>>;

// To allow both `&[String]` (vec slice of `String`) and `&[&str]` (vec slice of `&str`), see:
// https://stackoverflow.com/a/41180422/379923
pub fn get_hidden_ranges<T: AsRef<str>>(code: &[T]) -> HiddenRanges {
    let mut ranges = vec![];
    let mut curr_range: Option<Range<usize>> = None;

    for (idx, line) in code.iter().enumerate() {
        let n = idx + 1;
        let line = line.as_ref();
        let is_hidden = line.starts_with("# ") || line == "#";
        // let is_empty = line.trim() == "";

        if is_hidden {
            if let Some(range) = curr_range.as_mut() {
                range.end = n;
            } else {
                curr_range = Some(Range { start: n, end: n });
            }
        } else {
            if curr_range.is_some() {
                ranges.push(curr_range.clone().unwrap());
            }

            curr_range = None;
        }
    }

    if curr_range.is_some() {
        ranges.push(curr_range.unwrap());
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn split_lines(code: &str) -> Vec<&str> {
        code.split("\n").collect::<Vec<_>>()
    }

    #[test]
    fn empty_block() {
        let code = split_lines(indoc! {r#""#});

        assert_eq!(get_hidden_ranges(&code), vec![]);
    }

    #[test]
    fn no_hidden() {
        let code = split_lines(indoc! {r#"
            1
            2
            3
            4
            5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![]);
    }

    #[test]
    fn single_range() {
        let code = split_lines(indoc! {r#"
            # 1
            # 2
            # 3
            4
            5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![Range { start: 1, end: 3 }]);
    }

    #[test]
    fn multi_range() {
        let code = split_lines(indoc! {r#"
            # 1
            # 2
            3
            # 4
            # 5
        "#});

        assert_eq!(
            get_hidden_ranges(&code),
            vec![Range { start: 1, end: 2 }, Range { start: 4, end: 5 }]
        );
    }

    #[test]
    fn single_line_range() {
        let code = split_lines(indoc! {r#"
            1
            2
            3
            4
            # 5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![Range { start: 5, end: 5 }]);
    }
}
