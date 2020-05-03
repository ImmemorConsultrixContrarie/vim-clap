use {
    memchr::{memchr2, memrchr2},
    std::iter::{DoubleEndedIterator, FusedIterator, Iterator},
};

/// Newline char.
const NL: u8 = b'\n';
/// `\r` char.
const CR: u8 = b'\r';

#[derive(Clone)]
pub struct ByteLines<'a> {
    text: &'a [u8],
}

impl<'a> ByteLines<'a> {
    #[inline]
    pub fn new(text: &'a [u8]) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for ByteLines<'a> {
    type Item = &'a [u8];

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // If there's only newlines in the text, no items will be produced, so lower bound is 0.
        // But the maximum of items takes every second ASCII char to be a newline char.
        let high = self.text.len() / 2;
        (0, Some(high))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text;

        if text.is_empty() {
            return None;
        }

        let line = match memchr2(NL, CR, text) {
            Some(newline_idx) => {
                self.text = skip_newline(&text[newline_idx..]);
                &text[..newline_idx]
            }

            None => {
                // This line is the last one
                self.text = &[];
                text
            }
        };

        Some(line)
    }
}

impl DoubleEndedIterator for ByteLines<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let text = self.text;

        if text.is_empty() {
            return None;
        }

        let line = match memrchr2(NL, CR, text) {
            Some(newline_idx) => {
                self.text = skip_newline_back(&text[..newline_idx + 1]);
                &text[newline_idx + 1..]
            }

            None => {
                // This line is the last one
                self.text = &[];
                text
            }
        };

        Some(line)
    }
}

impl FusedIterator for ByteLines<'_> {}

#[inline]
fn skip_newline(text: &[u8]) -> &[u8] {
    match *text {
        [CR, NL, ..] | [NL, CR, ..] => &text[2..],
        _ => &text[1..],
    }
}

#[inline]
fn skip_newline_back(text: &[u8]) -> &[u8] {
    &match *text {
        [.., CR, NL] | [.., NL, CR] => &text[..text.len() - 2],
        _ => &text[..text.len() - 1],
    }
}

//XXX Actually, `lines()` produce even empty lines,
//XXX thus newlines should not be shrinked.
/*
/// Finds the first byte that is not newline or CR
/// and returns the slice starting with that byte.
#[inline]
fn shrink_newlines(mut text: &[u8]) -> &[u8] {
    // This match checks if there's just one or two newlines,
    // as those are the most frequent patterns.
    text = match *text {
        [CR, NL, CR, NL, not_cr, ..] => {
            if not_cr != CR {
                return &text[4..];
            }
            &text[4..]
        }
        [NL, NL, not_nl, ..] => {
            if not_nl != NL {
                return &text[2..];
            }
            &text[2..]
        }
        [CR, NL, not_cr_nl, ..] => {
            if not_cr_nl != CR && not_cr_nl != NL {
                return &text[2..];
            }
            &text[2..]
        }
        [NL, not_nl_or_cr, ..] => {
            if not_nl_or_cr != NL && not_nl_or_cr != CR {
                return &text[1..];
            }
            &text[1..]
        }
        _ => text,
    };

    let mut first_not_n_or_r_idx = None;
    for (idx, &byte) in text.iter().enumerate() {
        match byte {
            NL | CR => (),
            _ => {
                first_not_n_or_r_idx = Some(idx);
                break;
            }
        }
    }

    match first_not_n_or_r_idx {
        Some(idx) => &text[idx..],
        None => &[],
    }
}

/// Like `shrink_newlines`, but from the other end of slice.
#[inline]
fn shrink_newlines_back(mut text: &[u8]) -> &[u8] {
    // This match checks if there's just one or two newlines,
    // as those are the most frequent patterns.
    text = match *text {
        [.., not_nl, CR, NL, CR, NL] => {
            let l4 = text.len() - 4;
            if not_nl != NL {
                return &text[..l4];
            }
            &text[..l4]
        }
        [.., not_nl, NL, NL] => {
            let l2 = text.len() - 2;
            if not_nl != NL {
                return &text[..l2];
            }
            &text[..l2]
        }
        [.., not_cr_nl, CR, NL] => {
            let l2 = text.len() - 2;
            if not_cr_nl != NL && not_cr_nl != CR {
                return &text[..l2];
            }
            &text[..l2]
        }
        [.., not_nl_cr, NL] => {
            let l1 = text.len() - 1;
            if not_nl_cr != NL && not_nl_cr != CR {
                return &text[..l1];
            }
            &text[..l1]
        }
        _ => return text,
    };

    let mut first_not_n_or_r_idx = None;
    for (idx, &byte) in text.iter().enumerate().rev() {
        match byte {
            NL | CR => (),
            _ => {
                first_not_n_or_r_idx = Some(idx);
                break;
            }
        }
    }

    match first_not_n_or_r_idx {
        Some(idx) => &text[..=idx],
        None => &[],
    }
}
*/
