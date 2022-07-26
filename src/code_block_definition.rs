use std::ops::Range;

use crate::hidden_ranges::HiddenRanges;

#[derive(Debug, PartialEq)]
enum Annotation {
    HideLines(HiddenRanges),
    Other(String),
}

impl From<&str> for Annotation {
    fn from(text: &str) -> Self {
        let is_hide_lines = text.starts_with("hide_lines=");

        if is_hide_lines {
            Annotation::HideLines(
                text.get(11..)
                    .unwrap_or("")
                    .split(' ')
                    .filter(|r| r.trim() != "")
                    .map(|range| {
                        let is_range = range.contains('-');

                        if is_range {
                            let range = range.split('-').collect::<Vec<_>>();
                            let start = range[0].parse().unwrap();
                            let end = range[1].parse().unwrap();

                            Range { start, end }
                        } else {
                            let line_no = range.parse::<usize>().unwrap();
                            Range {
                                start: line_no,
                                end: line_no,
                            }
                        }
                    })
                    .collect(),
            )
        } else {
            Annotation::Other(String::from(text))
        }
    }
}

impl Annotation {
    fn into_string(&self) -> String {
        match self {
            Annotation::HideLines(ranges) => {
                let ranges = ranges
                    .iter()
                    .map(|r| {
                        if r.start == r.end {
                            format!("{}", r.start)
                        } else {
                            format!("{}-{}", r.start, r.end)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("hide_lines={}", ranges)
            }
            Annotation::Other(content) => content.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeBlockDefinition {
    annotations: Vec<Annotation>,
    hide_lines_idx: Option<usize>,
}

impl CodeBlockDefinition {
    pub fn new(line: &str) -> Option<CodeBlockDefinition> {
        if !line.starts_with("```rust") {
            return None;
        }

        let mut hide_lines_idx = None;
        let annotations = line
            .get(7..)
            .unwrap_or("")
            .split(',')
            .filter(|a| a.trim() != "")
            .enumerate()
            .map(|(idx, a)| {
                let annotation = Annotation::from(a);

                if let Annotation::HideLines(_) = annotation {
                    hide_lines_idx = Some(idx);
                }

                annotation
            })
            .collect();

        Some(CodeBlockDefinition {
            annotations,
            hide_lines_idx,
        })
    }

    pub fn get_hidden_ranges(&self) -> Option<&HiddenRanges> {
        self.hide_lines_idx.map(|idx| match &self.annotations[idx] {
            Annotation::HideLines(ranges) => ranges,
            Annotation::Other(_) => panic!("Unreachable?"),
        })
    }

    pub fn into_string(&self) -> String {
        let mut out = String::from("```rust");

        if !self.annotations.is_empty() {
            out.push(',');
        }

        out.push_str(
            &self
                .annotations
                .iter()
                .map(|a| a.into_string())
                .collect::<Vec<String>>()
                .join(","),
        );

        out
    }

    pub fn set_hidden_ranges(&mut self, hidden_ranges: HiddenRanges) {
        if hidden_ranges.is_empty() {
            // Remove
            match self.hide_lines_idx {
                Some(idx) => {
                    self.annotations.remove(idx);
                    self.hide_lines_idx = None;
                }
                None => (),
            }
        } else {
            // Add
            let annotation = Annotation::HideLines(hidden_ranges);

            match self.hide_lines_idx {
                Some(idx) => self.annotations[idx] = annotation,
                None => {
                    self.annotations.push(annotation);
                    self.hide_lines_idx = Some(self.annotations.len() - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ignore_malformed_lines() {
        let cases = vec!["text line", "```", "```js"];

        for case in cases {
            let definition = CodeBlockDefinition::new(case);
            assert_eq!(definition.is_none(), true);
        }
    }

    #[test]
    fn should_parse_simple_lines() {
        let line = "```rust";
        let definition = CodeBlockDefinition::new(line).unwrap();

        assert_eq!(
            definition,
            CodeBlockDefinition {
                annotations: vec![],
                hide_lines_idx: None,
            }
        );

        assert_eq!(definition.into_string(), String::from(line));
    }

    #[test]
    fn should_parse_other_annotations() {
        let line = "```rust,linenos,linenostart=10  , hl_lines=3-4 8-9";
        let definition = CodeBlockDefinition::new(line).unwrap();

        assert_eq!(
            definition,
            CodeBlockDefinition {
                annotations: vec![
                    Annotation::Other(String::from("linenos")),
                    Annotation::Other(String::from("linenostart=10  ")),
                    Annotation::Other(String::from(" hl_lines=3-4 8-9")),
                ],
                hide_lines_idx: None,
            }
        );

        assert_eq!(definition.into_string(), String::from(line));
    }

    fn new_range(start: usize, end: usize) -> Range<usize> {
        Range { start, end }
    }

    #[test]
    fn should_parse_hide_lines_annotations() {
        let line = "```rust,hide_lines=3-4 9";
        let definition = CodeBlockDefinition::new(line).unwrap();

        assert_eq!(
            definition,
            CodeBlockDefinition {
                annotations: vec![Annotation::HideLines(vec![
                    new_range(3, 4),
                    new_range(9, 9),
                ])],
                hide_lines_idx: Some(0),
            }
        );

        assert_eq!(definition.into_string(), String::from(line));
    }

    #[test]
    fn should_parse_annotations() {
        let line = "```rust,   linenos,hide_lines=3-9   ,linenostart=10  ,hl_lines=10-12";
        let definition = CodeBlockDefinition::new(line).unwrap();

        assert_eq!(
            definition,
            CodeBlockDefinition {
                annotations: vec![
                    Annotation::Other(String::from("   linenos")),
                    Annotation::HideLines(vec![new_range(3, 9)]),
                    Annotation::Other(String::from("linenostart=10  ")),
                    Annotation::Other(String::from("hl_lines=10-12")),
                ],
                hide_lines_idx: Some(1),
            }
        );

        assert_eq!(
            definition.into_string(),
            "```rust,   linenos,hide_lines=3-9,linenostart=10  ,hl_lines=10-12"
        );
    }
}
