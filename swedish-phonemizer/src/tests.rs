/*!
# Swedish Phonemizer Tests

Comprehensive tests for Swedish G2P conversion, covering:
- Exception words
- Sje-ljud and tje-ljud
- Retroflexes
- Vowel length rules
- Common Swedish words
*/

use crate::swedish_text_to_phonemes;
use test_case::test_case;

// Exception word tests
#[test_case("hej" => "h ɛ j"; "greeting")]
#[test_case("och" => "ɔ"; "and")]
#[test_case("jag" => "j ɑː"; "I")]
#[test_case("det" => "d eː"; "it/that")]
#[test_case("är" => "æːr"; "is/are")]
#[test_case("har" => "h ɑːr"; "have/has")]
#[test_case("inte" => "ɪ n t e"; "not")]
fn test_exception_words(word: &str) -> String {
    swedish_text_to_phonemes(word).unwrap()
}

// Sje-ljud [ɧ] tests
#[test_case("sjö" => matches Ok(s) if s.contains("ɧ"); "sj combination")]
#[test_case("sju" => matches Ok(s) if s.contains("ɧ"); "sj before u")]
#[test_case("skjorta" => matches Ok(s) if s.contains("ɧ"); "skj combination")]
#[test_case("stjärna" => matches Ok(s) if s.contains("ɧ"); "stj combination")]
#[test_case("ske" => matches Ok(s) if s.contains("ɧ"); "sk before front vowel")]
#[test_case("sköt" => matches Ok(s) if s.contains("ɧ"); "sk before ö")]
#[test_case("skina" => matches Ok(s) if s.contains("ɧ"); "sk before i")]
#[test_case("station" => matches Ok(s) if s.contains("ɧ"); "tion ending")]
#[test_case("pension" => matches Ok(s) if s.contains("ɧ"); "sion ending")]
fn test_sje_ljud(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    swedish_text_to_phonemes(word)
}

// Tje-ljud [ɕ] tests
#[test_case("tjugo" => matches Ok(s) if s.contains("ɕ"); "tj combination")]
#[test_case("tjej" => matches Ok(s) if s.contains("ɕ"); "tj before front vowel")]
#[test_case("kjol" => matches Ok(s) if s.contains("ɕ"); "kj combination")]
#[test_case("kött" => matches Ok(s) if s.contains("ɕ"); "k before front vowel")]
#[test_case("kärlek" => matches Ok(s) if s.contains("ɕ"); "k before ä")]
#[test_case("kön" => matches Ok(s) if s.contains("ɕ"); "k before ö")]
fn test_tje_ljud(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    swedish_text_to_phonemes(word)
}

// Retroflex tests
#[test_case("barn" => matches Ok(s) if s.contains("ɳ"); "rn -> retroflex n")]
#[test_case("bord" => matches Ok(s) if s.contains("ɖ"); "rd -> retroflex d")]
#[test_case("kort" => matches Ok(s) if s.contains("ʈ"); "rt -> retroflex t")]
#[test_case("karl" => matches Ok(s) if s.contains("ɭ"); "rl -> retroflex l")]
#[test_case("kors" => matches Ok(s) if s.contains("ʂ"); "rs -> retroflex s")]
fn test_retroflexes(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    swedish_text_to_phonemes(word)
}

// Silent letter tests
#[test_case("djup" => matches Ok(s) if s.starts_with("j"); "dj -> j")]
#[test_case("gjord" => matches Ok(s) if s.starts_with("j"); "gj -> j")]
#[test_case("hjärta" => matches Ok(s) if s.starts_with("j"); "hj -> j")]
#[test_case("ljus" => matches Ok(s) if s.starts_with("j"); "lj -> j")]
fn test_silent_letters(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    swedish_text_to_phonemes(word)
}

// Consonant cluster tests
#[test_case("kung" => matches Ok(s) if s.contains("ŋ"); "ng -> ŋ")]
#[test_case("kungen" => matches Ok(s) if s.contains("ŋ"); "ng in middle")]
#[test_case("bank" => matches Ok(s) if s.contains("ŋ k"); "nk -> ŋk")]
#[test_case("tack" => matches Ok(s) if s.contains("k") && !s.contains("ck"); "ck -> k")]
#[test_case("klocka" => matches Ok(s) if s.contains("k") && !s.contains("ck"); "ck in middle")]
fn test_consonant_clusters(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    swedish_text_to_phonemes(word)
}

// Vowel length tests - these test the complete phonemization including vowel length
#[test]
fn test_long_vowels() {
    // Vowels should be long before single consonants or word-final
    let result = swedish_text_to_phonemes("apa").unwrap();
    assert!(
        result.contains("ɑː"),
        "First 'a' should be long in 'apa': {}",
        result
    );

    let result = swedish_text_to_phonemes("bil").unwrap();
    assert!(
        result.contains("iː"),
        "'i' should be long in 'bil': {}",
        result
    );

    let result = swedish_text_to_phonemes("hus").unwrap();
    assert!(
        result.contains("ʉː"),
        "'u' should be long [ʉː] in 'hus': {}",
        result
    );
}

#[test]
fn test_short_vowels() {
    // Vowels should be short before consonant clusters
    let result = swedish_text_to_phonemes("katt").unwrap();
    assert!(
        !result.contains("ɑː"),
        "'a' should be short in 'katt': {}",
        result
    );

    let result = swedish_text_to_phonemes("mitt").unwrap();
    assert!(
        result.contains("ɪ"),
        "'i' should be short [ɪ] in 'mitt': {}",
        result
    );

    let result = swedish_text_to_phonemes("hund").unwrap();
    assert!(
        result.contains("ɵ") && !result.contains("ʉː"),
        "'u' should be short [ɵ] in 'hund': {}",
        result
    );
}

// Complete word tests
#[test]
fn test_complete_words() {
    // Test complete phonemization of common Swedish words

    // "barn" -> "b ɑː ɳ" (long a, retroflex n)
    let result = swedish_text_to_phonemes("barn").unwrap();
    assert!(
        result.contains("b") && result.contains("ɑː") && result.contains("ɳ"),
        "barn should be 'b ɑː ɳ': {}",
        result
    );

    // "skjorta" -> "ɧ uː ʈ a" (sje-ljud, long u, retroflex t)
    let result = swedish_text_to_phonemes("skjorta").unwrap();
    assert!(
        result.contains("ɧ") && result.contains("ʈ"),
        "skjorta should contain ɧ and ʈ: {}",
        result
    );

    // "tjugo" -> "ɕ ʉː ɡ uː" (tje-ljud, long vowels)
    let result = swedish_text_to_phonemes("tjugo").unwrap();
    assert!(
        result.contains("ɕ") && result.contains("ʉː"),
        "tjugo should contain ɕ and ʉː: {}",
        result
    );

    // "kungen" -> "k ɵ ŋ e n" or similar (k+front vowel can vary)
    let result = swedish_text_to_phonemes("kungen").unwrap();
    assert!(result.contains("ŋ"), "kungen should contain ŋ: {}", result);
}

#[test]
fn test_multi_word_input() {
    let result = swedish_text_to_phonemes("hej världen").unwrap();
    assert!(
        result.contains("h ɛ j") && result.contains("v"),
        "Multi-word input failed: {}",
        result
    );
}

#[test]
fn test_empty_input() {
    let result = swedish_text_to_phonemes("");
    assert!(result.is_err(), "Empty input should return error");

    let result = swedish_text_to_phonemes("   ");
    assert!(result.is_err(), "Whitespace-only input should return error");
}

#[test]
fn test_normalization() {
    // Test that input normalization works
    let result1 = swedish_text_to_phonemes("HEJ").unwrap();
    let result2 = swedish_text_to_phonemes("hej").unwrap();
    assert_eq!(result1, result2, "Case normalization should work");

    // Test with punctuation (should be filtered out)
    let result = swedish_text_to_phonemes("hej!").unwrap();
    assert_eq!(
        result,
        swedish_text_to_phonemes("hej").unwrap(),
        "Punctuation should be filtered"
    );
}

// Stress test with complex Swedish words
#[test]
fn test_complex_words() {
    let words = vec![
        "nationalitet",
        "kommunikation",
        "representant",
        "organisation",
        "fundamental",
        "konstruktion",
    ];

    for word in words {
        let result = swedish_text_to_phonemes(word);
        assert!(
            result.is_ok(),
            "Failed to phonemize '{}': {:?}",
            word,
            result
        );
        let phonemes = result.unwrap();
        assert!(!phonemes.is_empty(), "Empty result for '{}'", word);
        // Just ensure it produces reasonable output
        assert!(
            phonemes.len() > word.len() / 2,
            "Suspiciously short output for '{}': '{}'",
            word,
            phonemes
        );
    }
}

// Regression tests for specific known issues
#[test]
fn test_known_patterns() {
    // Make sure specific Swedish patterns work correctly

    // "och" should be just "ɔ" (exception word)
    assert_eq!(swedish_text_to_phonemes("och").unwrap(), "ɔ");

    // Words ending in -tion should have sje-ljud
    let result = swedish_text_to_phonemes("nation").unwrap();
    assert!(result.contains("ɧ"), "nation should contain ɧ: {}", result);

    // Double consonants should make preceding vowel short
    let result = swedish_text_to_phonemes("hemma").unwrap();
    assert!(
        !result.contains("eː"),
        "First 'e' in hemma should be short: {}",
        result
    );
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_phonemization() {
        let test_words = vec![
            "hej",
            "världen",
            "svenska",
            "språket",
            "phonemizer",
            "skjorta",
            "tjugo",
            "kungen",
            "barn",
            "nationalitet",
        ];

        let start = Instant::now();
        for _ in 0..1000 {
            for word in &test_words {
                let _ = swedish_text_to_phonemes(word).unwrap();
            }
        }
        let duration = start.elapsed();

        // Should complete 10k phonemizations in reasonable time
        println!("10k phonemizations took: {:?}", duration);
        assert!(
            duration.as_secs() < 5,
            "Phonemization too slow: {:?}",
            duration
        );
    }
}
