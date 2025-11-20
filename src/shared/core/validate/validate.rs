use crate::shared::core::{errors::PathError, path::ValuePath};

pub fn validate_path_syntax(input: &str) -> Result<(), PathError> {
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

    // Basic validation
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
            input,
            "consecutive dots not allowed",
        ));
    }

    let mut state = BetweenSeg;
    let mut bracket_start_idx = 0;

    for (i, ch) in input.char_indices() {
        state = match (state, ch) {
            // -------- Between segments --------
            (BetweenSeg, '[') => {
                bracket_start_idx = i;
                Bracket(Start)
            }
            (BetweenSeg, ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[..=i],
                    "unmatched closing bracket",
                ));
            }
            (BetweenSeg, _) => BareKey,

            // -------- Bare key (top-level) --------
            (BareKey, '.') => BetweenSeg,
            (BareKey, '[') => {
                bracket_start_idx = i;
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

            // -------- Inside brackets: Start state --------
            (Bracket(Start), '"') | (Bracket(Start), '\'') => Bracket(Quoted { q: ch }),
            (Bracket(Start), ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[bracket_start_idx..=i],
                    "empty brackets not allowed",
                ));
            }
            (Bracket(Start), c) if c.is_ascii_digit() => Bracket(Index),
            (Bracket(Start), _) => Bracket(Bare),

            // -------- Inside brackets: Index state --------
            (Bracket(Index), ']') => BetweenSeg,
            (Bracket(Index), c) if c.is_ascii_digit() => Bracket(Index),
            (Bracket(Index), _) => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[bracket_start_idx..=i],
                    "array index must contain only digits",
                ));
            }

            // -------- Inside brackets: Quoted state --------
            (Bracket(Quoted { q }), c) if c == q => {
                // closed the quote; now the very next char must be ']'
                Bracket(ExpectClose)
            }
            (Bracket(Quoted { q }), _) => {
                // Any character inside quotes is valid, including ']'
                Bracket(Quoted { q })
            }

            // -------- Inside brackets: ExpectClose state --------
            (Bracket(ExpectClose), ']') => BetweenSeg,
            (Bracket(ExpectClose), _) => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[bracket_start_idx..=i],
                    "quoted key must be immediately followed by ']'",
                ));
            }

            // -------- Inside brackets: Bare (unquoted non-digit) state --------
            (Bracket(Bare), ']') => {
                return Err(PathError::invalid_seg(
                    ValuePath::new(),
                    &input[bracket_start_idx..=i],
                    "bracket content must be either a quoted string or numeric index",
                ));
            }
            (Bracket(Bare), _) => Bracket(Bare),
        };
    }

    // Finalize: ensure we ended in a valid state
    match state {
        Bracket(Quoted { .. }) => Err(PathError::invalid_seg(
            ValuePath::new(),
            input,
            "unclosed quote in bracket",
        )),
        Bracket(Start) | Bracket(Index) | Bracket(Bare) | Bracket(ExpectClose) => Err(
            PathError::invalid_seg(ValuePath::new(), input, "unclosed bracket"),
        ),
        BetweenSeg => {
            // Path ended right after a separator (. or ])
            // This is only invalid if it ended with '.' (already checked)
            // If it ended with ']', we're in BetweenSeg which is fine
            Ok(())
        }
        BareKey => Ok(()),
    }
}
