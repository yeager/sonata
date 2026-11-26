/*!
# Swedish G2P Rules

This module implements the grapheme-to-phoneme conversion rules for Swedish,
including special cases for sje-ljud, tje-ljud, retroflexes, and silent letters.
*/

use crate::{PhonemeContext, SwedishPhonemizerError};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::error::Error;

/// Exception words that don't follow regular rules
static EXCEPTION_WORDS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Common Swedish exception words
    map.insert("hej", "h ɛ j");
    map.insert("och", "ɔ");
    map.insert("jag", "j ɑː");
    map.insert("det", "d eː");
    map.insert("är", "æːr");
    map.insert("har", "h ɑːr");
    map.insert("inte", "ɪ n t e");
    map.insert("att", "a t");
    map.insert("han", "h a n");
    map.insert("hon", "h ʊ n");
    map.insert("den", "d eː n");
    map.insert("här", "h æːr");
    map.insert("där", "d æːr");
    map.insert("när", "n æːr");
    map.insert("vad", "v a");
    map.insert("som", "s ɔ m");
    map.insert("med", "m eː");
    map.insert("för", "f øːr");
    map.insert("till", "t ɪ l");
    map.insert("av", "ɑː v");
    map.insert("på", "p oː");
    map.insert("i", "iː");
    map.insert("om", "ɔ m");
    map.insert("så", "s oː");
    map.insert("får", "f oːr");
    map.insert("går", "g oːr");
    map.insert("står", "s t oːr");
    map.insert("kan", "k a n");
    map.insert("ska", "s k a");
    map.insert("skulle", "s k ʉ l e");
    map.insert("kommer", "k ɔ m er");
    map.insert("blir", "b l iːr");
    map.insert("varit", "v ɑː r ɪ t");

    map
});

/// Get phonemes for exception words
pub fn get_exception_word(word: &str) -> Option<String> {
    EXCEPTION_WORDS.get(word).map(|&s| s.to_string())
}

/// Apply Swedish G2P rules to convert graphemes to phonemes
pub fn apply_g2p_rules(context: &PhonemeContext) -> Result<(String, usize), Box<dyn Error>> {
    let remaining = context.remaining;

    if remaining.is_empty() {
        return Ok((String::new(), 0));
    }

    let current = remaining[0];

    // Multi-character rules (check longest first)
    if let Some((phoneme, consumed)) = check_multi_char_rules(context)? {
        return Ok((phoneme, consumed));
    }

    // Single character rules
    let phoneme = match current {
        // Vowels
        'a' => "a",
        'e' => "e",
        'i' => "i",
        'o' => "uː", // Swedish 'o' is typically long /uː/
        'u' => "ɵ",  // Swedish 'u' is typically /ɵ/
        'y' => "y",
        'å' => "oː",
        'ä' => "æ",
        'ö' => "ø",

        // Consonants
        'b' => "b",
        'c' => "s", // Usually /s/ in Swedish
        'd' => "d",
        'f' => "f",
        'g' => handle_g(context)?,
        'h' => handle_h(context)?,
        'j' => "j",
        'k' => handle_k(context)?,
        'l' => handle_l(context)?,
        'm' => "m",
        'n' => handle_n(context)?,
        'p' => "p",
        'q' => "k", // Rare in Swedish, usually /k/
        'r' => handle_r(context)?,
        's' => handle_s(context)?,
        't' => handle_t(context)?,
        'v' => "v",
        'w' => "v", // Swedish 'w' is pronounced as 'v'
        'x' => "k s",
        'z' => "s", // Usually /s/ in Swedish

        _ => {
            return Err(Box::new(SwedishPhonemizerError::ProcessingError(format!(
                "Unknown character: {}",
                current
            ))))
        }
    };

    Ok((phoneme.to_string(), 1))
}

/// Check multi-character rules (digraphs, trigraphs, etc.)
fn check_multi_char_rules(
    context: &PhonemeContext,
) -> Result<Option<(String, usize)>, Box<dyn Error>> {
    let slice2 = context.slice(2);
    let slice3 = context.slice(3);
    let slice4 = context.slice(4);
    // Check longer patterns first

    // 4-character patterns
    if slice4 == "tion" || slice4 == "sion" {
        return Ok(Some(("ɧ uː n".to_string(), 4))); // -tion/-sion → /ɧuːn/
    }

    // 3-character patterns
    match slice3.as_str() {
        "sch" | "skj" | "stj" => return Ok(Some(("ɧ".to_string(), 3))),
        _ => {}
    }

    // 2-character patterns
    match slice2.as_str() {
        // Sje-ljud [ɧ]
        "sj" | "sh" => return Ok(Some(("ɧ".to_string(), 2))),

        // Tje-ljud [ɕ]
        "tj" | "kj" => return Ok(Some(("ɕ".to_string(), 2))),

        // Silent letters
        "dj" | "gj" | "hj" | "lj" => return Ok(Some(("j".to_string(), 2))),

        // Consonant clusters
        "ng" => return Ok(Some(("ŋ".to_string(), 2))),
        "nk" => return Ok(Some(("ŋ k".to_string(), 2))),
        "ck" => return Ok(Some(("k".to_string(), 2))),

        // Retroflexes (r + consonant)
        "rn" => return Ok(Some(("ɳ".to_string(), 2))),
        "rd" => return Ok(Some(("ɖ".to_string(), 2))),
        "rt" => return Ok(Some(("ʈ".to_string(), 2))),
        "rl" => return Ok(Some(("ɭ".to_string(), 2))),
        "rs" => return Ok(Some(("ʂ".to_string(), 2))),

        _ => {}
    }

    // Check 'sk' + front vowel → /ɧ/
    if slice2 == "sk" && context.char_at(2).is_some_and(is_front_vowel) {
        return Ok(Some(("ɧ".to_string(), 2)));
    }

    // Check 'k' + front vowel → /ɕ/ (but not if it's part of 'ck')
    if context.remaining[0] == 'k'
        && context.char_at(1).is_some_and(is_front_vowel)
        && context.char_at(-1) != Some('c')
    {
        // Not 'ck'
        return Ok(Some(("ɕ".to_string(), 1)));
    }

    Ok(None)
}

/// Handle 'g' phonemization
fn handle_g(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // 'g' before front vowels can be /j/ or /g/ - default to /g/ for simplicity
    Ok("g")
}

/// Handle 'h' phonemization
fn handle_h(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // Check if 'h' is silent in combinations (handled in multi-char rules)
    Ok("h")
}

/// Handle 'k' phonemization
fn handle_k(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // k + front vowel → /ɕ/ (handled in multi-char rules)
    Ok("k")
}

/// Handle 'l' phonemization
fn handle_l(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // rl → /ɭ/ (handled in multi-char rules)
    Ok("l")
}

/// Handle 'n' phonemization
fn handle_n(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // ng → /ŋ/, nk → /ŋk/, rn → /ɳ/ (handled in multi-char rules)
    Ok("n")
}

/// Handle 'r' phonemization
fn handle_r(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // r + consonant → retroflex (handled in multi-char rules)
    Ok("r")
}

/// Handle 's' phonemization
fn handle_s(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // sj → /ɧ/, sk+front vowel → /ɧ/, rs → /ʂ/ (handled in multi-char rules)
    Ok("s")
}

/// Handle 't' phonemization
fn handle_t(_context: &PhonemeContext) -> Result<&'static str, Box<dyn Error>> {
    // tj → /ɕ/, rt → /ʈ/ (handled in multi-char rules)
    Ok("t")
}

/// Check if a character is a front vowel
fn is_front_vowel(c: char) -> bool {
    matches!(c, 'e' | 'i' | 'y' | 'ä' | 'ö')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exception_words() {
        assert_eq!(get_exception_word("hej"), Some("h ɛ j".to_string()));
        assert_eq!(get_exception_word("och"), Some("ɔ".to_string()));
        assert_eq!(get_exception_word("unknown"), None);
    }

    #[test]
    fn test_front_vowels() {
        assert!(is_front_vowel('e'));
        assert!(is_front_vowel('i'));
        assert!(is_front_vowel('y'));
        assert!(is_front_vowel('ä'));
        assert!(is_front_vowel('ö'));
        assert!(!is_front_vowel('a'));
        assert!(!is_front_vowel('o'));
        assert!(!is_front_vowel('u'));
        assert!(!is_front_vowel('å'));
    }
}
