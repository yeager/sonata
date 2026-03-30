/*!
# Swedish Phonemizer

A rule-based Swedish grapheme-to-phoneme (G2P) converter for the Spiel/libspiel ecosystem.
This crate provides Swedish phonemization without requiring espeak-ng, implementing
language-specific rules for Swedish pronunciation.

## Usage

```rust
use swedish_phonemizer::swedish_text_to_phonemes;

let phonemes = swedish_text_to_phonemes("hej världen").unwrap();
// Returns IPA phonemes compatible with Piper TTS
```

## Features

- Rule-based Swedish G2P conversion
- Support for Swedish-specific phonemes (sje-ljud, tje-ljud, retroflexes)
- Vowel length prediction
- Exception word handling
- Compatible with Piper phoneme_id_map format
*/

use std::error::Error;
use std::fmt;

pub mod rules;
pub mod vowels;

#[cfg(test)]
mod tests;

/// Error types for Swedish phonemization
#[derive(Debug, Clone)]
pub enum SwedishPhonemizerError {
    /// Empty or invalid input text
    InvalidInput(String),
    /// Internal processing error
    ProcessingError(String),
}

impl fmt::Display for SwedishPhonemizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwedishPhonemizerError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
            SwedishPhonemizerError::ProcessingError(msg) => {
                write!(f, "Processing error: {}", msg)
            }
        }
    }
}

impl Error for SwedishPhonemizerError {}

/// Convert Swedish text to IPA phonemes
///
/// This is the main entry point for Swedish phonemization. It processes
/// Swedish text and returns IPA phonemes compatible with the Piper TTS
/// phoneme_id_map format.
///
/// # Arguments
///
/// * `text` - Swedish text to phonemize
///
/// # Returns
///
/// * `Ok(String)` - IPA phonemes separated by spaces
/// * `Err(Box<dyn Error>)` - Error if phonemization fails
///
/// # Examples
///
/// ```rust
/// use swedish_phonemizer::swedish_text_to_phonemes;
///
/// let result = swedish_text_to_phonemes("hej");
/// assert_eq!(result.unwrap(), "h ɛ j");
///
/// let result = swedish_text_to_phonemes("sjö");
/// assert_eq!(result.unwrap(), "ɧ øː");
/// ```
pub fn swedish_text_to_phonemes(text: &str) -> Result<String, Box<dyn Error>> {
    if text.trim().is_empty() {
        return Err(Box::new(SwedishPhonemizerError::InvalidInput(
            "Empty input text".to_string(),
        )));
    }

    // Normalize the input text
    let normalized = normalize_text(text);

    // Split into words and process each
    let words: Vec<String> = normalized
        .split_whitespace()
        .map(phonemize_word)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(words.join(" "))
}

/// Normalize Swedish text for phonemization
fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}

/// Phonemize a single Swedish word
fn phonemize_word(word: &str) -> Result<String, Box<dyn Error>> {
    if word.is_empty() {
        return Ok(String::new());
    }

    // Check for exception words first
    if let Some(phonemes) = rules::get_exception_word(word) {
        return Ok(phonemes);
    }

    // Apply grapheme-to-phoneme rules
    let mut result = String::new();
    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let remaining = &chars[i..];
        let context = PhonemeContext {
            word: &chars,
            position: i,
            remaining,
        };

        let (phoneme, consumed) = rules::apply_g2p_rules(&context)?;

        if !result.is_empty() && !phoneme.is_empty() {
            result.push(' ');
        }
        result.push_str(&phoneme);
        i += consumed;
    }

    // Apply vowel length rules
    let result = vowels::apply_vowel_length_rules(&result, word)?;

    Ok(result)
}

/// Context information for phoneme conversion
#[derive(Debug)]
pub struct PhonemeContext<'a> {
    /// The complete word as characters
    pub word: &'a [char],
    /// Current position in the word
    pub position: usize,
    /// Remaining characters from current position
    pub remaining: &'a [char],
}

impl<'a> PhonemeContext<'a> {
    /// Get the character at a relative position (0 = current)
    pub fn char_at(&self, offset: isize) -> Option<char> {
        let pos = self.position as isize + offset;
        if pos >= 0 && (pos as usize) < self.word.len() {
            Some(self.word[pos as usize])
        } else {
            None
        }
    }

    /// Check if we're at the beginning of the word
    pub fn is_word_start(&self) -> bool {
        self.position == 0
    }

    /// Check if we're at the end of the word
    pub fn is_word_end(&self) -> bool {
        self.position >= self.word.len() - 1
    }

    /// Get a slice of characters from current position
    pub fn slice(&self, len: usize) -> String {
        self.remaining.iter().take(len).collect()
    }
}
