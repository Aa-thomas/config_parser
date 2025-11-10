use crate::shared::{errors::PathError, types::ValuePath};

pub fn validate_path_syntax(input: &str) -> Result<(), PathError> {
    // Local-only enums: visible only inside this function.
    enum State {
        BetweenSeg,
        BareKey,
        Bracket(BracketState),
    }
    enum BracketState {
        Start,              // just saw '['
        Index,              // digits for array index
        Quoted { q: char }, // inside "..." or '...'
        ExpectClose,        // finished a quoted key; require ']'
        Bare,               // unquoted key inside [], disallowed
    }

    use BracketState::*;
    use State::*;

    if input.is_empty() {
        return Err(PathError::EmptyPath);
    }
    if input.starts_with('.') {
        return Err(PathError::invalid_seg(
            ValuePath::new(),
            input,
            "path cannot start with dot",
        ));
    }
    if input.ends_with('.') {
        return Err(PathError::invalid_seg(
            ValuePath::new(),
            input,
            "path cannot end with dot",
        ));
    }
    if input.contains("..") {
        return Err(PathError::invalid_seg(
            ValuePath::new(),
            "..",
            "consecutive dots not allowed",
        ));
    }

    let mut state = BetweenSeg;

    // scratch vars used while inside [...]
    let mut saw_any_in_bracket = false;
    let mut index_only_digits = true;

    // track empty segments like "a..b"
    let mut seg_started = false;

    for (i, ch) in input.char_indices() {
        state = match (state, ch) {
            // -------- Between segments --------
            (BetweenSeg, '.') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "empty segment",
                ));
            }
            (BetweenSeg, '[') => {
                saw_any_in_bracket = false;
                index_only_digits = true;
                Bracket(Start)
            }
            (BetweenSeg, ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "unmatched closing bracket",
                ));
            }
            (BetweenSeg, c) => {
                if c == '.' || c == '[' || c == ']' {
                    return Err(PathError::invalid_seg(
                        ValuePath::new(),
                        &input[..=i],
                        "unexpected separator",
                    ));
                }
                seg_started = true;
                BareKey
            }

            // -------- Bare key (top-level) --------
            (BareKey, '.') => {
                seg_started = false;
                BetweenSeg
            }
            (BareKey, '[') => {
                saw_any_in_bracket = false;
                index_only_digits = true;
                Bracket(Start)
            }
            (BareKey, ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "unmatched closing bracket",
                ));
            }
            (BareKey, _) => BareKey,

            // -------- Inside brackets --------
            (Bracket(Start), '"') | (Bracket(Start), '\'') => {
                saw_any_in_bracket = true;
                Bracket(Quoted { q: ch })
            }
            (Bracket(Start), ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    "[]",
                    "empty brackets not allowed",
                ));
            }
            (Bracket(Start), c) => {
                saw_any_in_bracket = true;
                index_only_digits &= c.is_ascii_digit();
                if c.is_ascii_digit() {
                    Bracket(Index)
                } else {
                    Bracket(Bare)
                }
            }

            (Bracket(Index), ']') => {
                if !saw_any_in_bracket {
                    return Err(PathError::invalid_seg(
                        ValuePath::new(),
                        "[]",
                        "empty brackets not allowed",
                    ));
                }
                if !index_only_digits {
                    return Err(PathError::invalid_seg(
                        ValuePath::new(),
                        &input[..=i],
                        "index must be digits",
                    ));
                }
                BetweenSeg
            }
            (Bracket(Index), c) => {
                saw_any_in_bracket = true;
                index_only_digits &= c.is_ascii_digit();
                Bracket(Index)
            }

            (Bracket(Quoted { q }), c) if c == q => {
                // closed the quote; now the very next char must be ']'
                Bracket(ExpectClose)
            }
            (Bracket(Quoted { q: _ }), ']') => {
                // hitting ']' while still in quotes → unclosed quote
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "unclosed quote",
                ));
            }
            (Bracket(Quoted { q }), _) => Bracket(Quoted { q }),

            (Bracket(ExpectClose), ']') => BetweenSeg,
            (Bracket(ExpectClose), _) => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "quoted key must be immediately closed by ']'",
                ));
            }

            (Bracket(Bare), ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "bracket content must be either a number or quoted string",
                ));
            }
            (Bracket(Bare), _) => Bracket(Bare),
        };
    }

    // finalize
    match state {
        Bracket(Quoted { .. }) => Err(PathError::invalid_seg(
            ValuePath::new(),
            input,
            "unclosed quote",
        )),
        Bracket(Start) | Bracket(Index) | Bracket(Bare) | Bracket(ExpectClose) => Err(
            PathError::invalid_seg(ValuePath::new(), input, "unclosed bracket"),
        ),
        BetweenSeg | BareKey => Ok(()),
    }
}
