use super::{Lint, LintKind, Linter};
use crate::TokenStringExt;
use crate::{Document, Span};

/// Detect and warn that the sentence is too long.
#[derive(Debug, Clone, Copy, Default)]
pub struct LongSentences;

impl Linter for LongSentences {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut output = Vec::new();

        for sentence in document.iter_sentences() {
            let word_count = sentence.iter_words().count();

            if word_count > 40 {
                output.push(Lint {
                    span: Span::new(
                        sentence.first_word().unwrap().span.start,
                        sentence.last().unwrap().span.end,
                    ),
                    lint_kind: LintKind::Readability,
                    message: format!("This sentence is {word_count} words long."),
                    ..Default::default()
                })
            }
        }

        output
    }

    fn description(&self) -> &'static str {
        "This rule looks for run-on sentences, which can make your work harder to grok."
    }
}

#[cfg(test)]
mod tests {
    use super::LongSentences;
    use crate::Document;
    use crate::linting::Linter;

    #[test]
    fn does_not_flag_two_sentences_separated_by_single_letter_terminator() {
        let text = "They default to the Xbox controller layout, using the bottom button as Accept and the right button as Back, even on Switch Pro controllers which label the right button as A and bottom as B. There was no way to remap buttons on wireless controllers UI until Android 17.";
        let doc = Document::new_plain_english_curated(text);
        let mut linter = LongSentences;
        let lints = linter.lint(&doc);
        assert!(lints.is_empty(), "expected no lints, got: {lints:?}");
    }
}
