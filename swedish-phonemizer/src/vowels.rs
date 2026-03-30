/*!
# Swedish Vowel Length Rules

This module implements Swedish vowel length and quality rules:
- Vowels are long before single consonants or at word end
- Vowels are short before consonant clusters
- Specific vowel quality changes based on length
*/

use crate::SwedishPhonemizerError;
use std::error::Error;
// Note: Regex patterns removed as they're not currently used
// They may be needed for future enhancements

/// Apply Swedish vowel length rules to phonemized text
pub fn apply_vowel_length_rules(
    phonemes: &str,
    original_word: &str,
) -> Result<String, Box<dyn Error>> {
    let parts: Vec<&str> = phonemes.split_whitespace().collect();
    let mut result = Vec::new();

    for (i, &phoneme) in parts.iter().enumerate() {
        if is_vowel_phoneme(phoneme) {
            let lengthened = apply_length_to_vowel(phoneme, i, &parts, original_word)?;
            result.push(lengthened);
        } else {
            result.push(phoneme.to_string());
        }
    }

    Ok(result.join(" "))
}

/// Check if a phoneme represents a vowel
fn is_vowel_phoneme(phoneme: &str) -> bool {
    matches!(
        phoneme,
        "a" | "e"
            | "i"
            | "u"
            | "ɵ"
            | "y"
            | "oː"
            | "æ"
            | "ø"
            | "ɑ"
            | "ɛ"
            | "ɪ"
            | "ʉ"
            | "ʏ"
            | "ɔ"
            | "æː"
            | "øː"
            | "œ"
            | "aː"
            | "eː"
            | "iː"
            | "uː"
            | "ɵː"
            | "yː"
            | "ɑː"
            | "ʊ"
    )
}

/// Apply length rules to a vowel phoneme
fn apply_length_to_vowel(
    vowel: &str,
    position: usize,
    phonemes: &[&str],
    _original_word: &str,
) -> Result<String, Box<dyn Error>> {
    // Skip if vowel is already long
    if vowel.ends_with('ː') {
        return Ok(vowel.to_string());
    }

    // Analyze consonant context after this vowel
    let consonants_after = count_consonants_after(position, phonemes);
    let is_word_final = position == phonemes.len() - 1;

    let should_be_long = is_word_final || consonants_after <= 1;

    if should_be_long {
        lengthen_vowel(vowel)
    } else {
        shorten_vowel(vowel)
    }
}

/// Count consonants following a vowel position
fn count_consonants_after(vowel_pos: usize, phonemes: &[&str]) -> usize {
    let mut count = 0;
    let mut pos = vowel_pos + 1;

    while pos < phonemes.len() {
        if !is_vowel_phoneme(phonemes[pos]) {
            count += 1;
            pos += 1;
        } else {
            break; // Stop at next vowel
        }
    }

    count
}

/// Make a vowel long and adjust quality if needed
fn lengthen_vowel(vowel: &str) -> Result<String, Box<dyn Error>> {
    let long_vowel = match vowel {
        "a" => "ɑː",
        "e" => "eː",
        "i" => "iː",
        "u" => "uː",  // /o/ sound → long /uː/
        "uː" => "uː", // Already long
        "ɵ" => "ʉː",  // Swedish /u/ → long /ʉː/
        "y" => "yː",
        "æ" => "æː", // /ä/
        "ø" => "øː", // /ö/
        "ɑ" => "ɑː",
        "ɛ" => "eː",
        "ɪ" => "iː",
        "ʉ" => "ʉː",
        "ʏ" => "yː",
        "ɔ" => "oː",

        // Already long vowels
        v if v.ends_with('ː') => v,

        _ => {
            return Err(Box::new(SwedishPhonemizerError::ProcessingError(format!(
                "Unknown vowel for lengthening: {}",
                vowel
            ))))
        }
    };

    Ok(long_vowel.to_string())
}

/// Make a vowel short and adjust quality if needed  
fn shorten_vowel(vowel: &str) -> Result<String, Box<dyn Error>> {
    let short_vowel = match vowel {
        "a" | "ɑː" => "a",
        "e" | "eː" => "ɛ", // Short /e/ is often [ɛ]
        "i" | "iː" => "ɪ", // Short /i/ is [ɪ]
        "u" | "uː" => "ʊ", // Short /o/ is [ʊ]
        "ɵ" | "ʉː" => "ɵ", // Short /u/ stays [ɵ]
        "y" | "yː" => "ʏ", // Short /y/ is [ʏ]
        "æ" | "æː" => "æ", // Short /ä/
        "ø" | "øː" => "œ", // Short /ö/ is [œ]
        "ɔ" | "oː" => "ɔ", // Short /å/

        // Handle already short vowels
        "ɛ" | "ɪ" | "ʊ" | "ʏ" | "œ" => vowel,

        // Remove length marker from long vowels
        v if v.ends_with('ː') => {
            let base = v.strip_suffix('ː').unwrap();
            return shorten_vowel(base);
        }

        _ => {
            return Err(Box::new(SwedishPhonemizerError::ProcessingError(format!(
                "Unknown vowel for shortening: {}",
                vowel
            ))))
        }
    };

    Ok(short_vowel.to_string())
}

/// Analyze Swedish stress patterns (simplified)
pub fn apply_stress_rules(phonemes: &str) -> Result<String, Box<dyn Error>> {
    // Swedish has relatively simple stress - usually first syllable
    // For this implementation, we'll keep it simple and not mark stress
    // as the Piper models typically handle this internally
    Ok(phonemes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel_phoneme() {
        assert!(is_vowel_phoneme("a"));
        assert!(is_vowel_phoneme("eː"));
        assert!(is_vowel_phoneme("ɑː"));
        assert!(!is_vowel_phoneme("b"));
        assert!(!is_vowel_phoneme("ɧ"));
    }

    #[test]
    fn test_lengthen_vowel() {
        assert_eq!(lengthen_vowel("a").unwrap(), "ɑː");
        assert_eq!(lengthen_vowel("e").unwrap(), "eː");
        assert_eq!(lengthen_vowel("i").unwrap(), "iː");
        assert_eq!(lengthen_vowel("u").unwrap(), "uː");
        assert_eq!(lengthen_vowel("ɵ").unwrap(), "ʉː");
    }

    #[test]
    fn test_shorten_vowel() {
        assert_eq!(shorten_vowel("a").unwrap(), "a");
        assert_eq!(shorten_vowel("e").unwrap(), "ɛ");
        assert_eq!(shorten_vowel("i").unwrap(), "ɪ");
        assert_eq!(shorten_vowel("u").unwrap(), "ʊ");
        assert_eq!(shorten_vowel("y").unwrap(), "ʏ");
    }

    #[test]
    fn test_count_consonants_after() {
        let phonemes = vec!["a", "b", "c", "d", "e"];
        assert_eq!(count_consonants_after(0, &phonemes), 3); // a -> bcd -> e
        assert_eq!(count_consonants_after(4, &phonemes), 0); // final e
    }

    #[test]
    fn test_vowel_length_rules() {
        // Long before single consonant (test with simple phonemes)
        let result = apply_vowel_length_rules("b a l", "bal").unwrap();
        println!("bal result: '{}'", result);
        assert!(result.contains("ɑː")); // "a" should be long

        // Short before consonant cluster
        let result = apply_vowel_length_rules("k a t t", "katt").unwrap();
        println!("katt result: '{}'", result);
        assert!(result.contains("a") && !result.contains("ɑː")); // "a" should be short
    }
}
