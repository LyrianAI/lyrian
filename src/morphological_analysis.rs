//! Morphological analysis and the tokens generated by it.

use lindera::tokenizer::Tokenizer;
use serde::{Deserialize, Serialize};
use std::mem;

use crate::chars::*;

/// Token structure
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialOrd, PartialEq)]
pub struct LyrianToken {
    pub word: String,
    pub mora: String,
    pub syllable: String,
}

impl LyrianToken {
    /// Creates a new instance of [`LyrianToken`].
    pub fn new(word: String, mora: String, syllable: String) -> LyrianToken {
        LyrianToken {
            word: word,
            mora: mora,
            syllable: syllable,
        }
    }

    /// Calculates the number of pronunciation.
    ///
    /// The return value will be changed by the following arguments.
    ///
    /// - syllable: [`bool`]
    ///     - Will calculate the number by syllable unit.
    ///
    /// If you set `false` to all the arguments, you will get the number by
    /// mora unit.
    pub fn length(&self, syllable: bool) -> usize {
        if self.mora == String::from("unknown") {
            return 0;
        }

        let mut sound_len = self.mora.chars().count();

        sound_len -= dup_num(
            &self.mora.chars().collect(),
            &vec![LOWER_CASE.to_vec(), SYMBOLS.to_vec()].concat(),
        );

        if syllable {
            sound_len = self.syllable_len();
        }

        // if voiceless {
        //     sound_len -= self.count_voiceless();
        // }

        // if smooth {
        //     sound_len -= self.count_smooth();
        // }

        sound_len
    }

    /// Returns the length of the word by syllable unit.
    fn syllable_len(&self) -> usize {
        let length = self.syllable.chars().count();
        let count = dup_num(&self.syllable.chars().collect(), &SYLLABLE_CHARS.to_vec());
        length - count
    }

    // fn count_voiceless(&self) -> usize {
    //     // TODO: Processing to calc number of voiceless sound
    //     0
    // }

    // fn count_smooth(&self) -> usize {
    //     // TODO: Processing to calc number of smooth vowel sound
    //     0
    // }
}

/// Tokenizes contents in morphological analysis.
///
/// Lyrian uses [lindera](https://github.com/lindera-morphology/lindera) crate
/// for morphological analysis.
pub fn tokenize(contents: &str) -> Result<Vec<LyrianToken>, String> {
    let mut tokenizer;
    let lin_tokens;

    match Tokenizer::new() {
        Ok(v) => tokenizer = v,
        Err(e) => return Err(e.to_string()),
    }

    match tokenizer.tokenize(&*contents) {
        Ok(v) => lin_tokens = v,
        Err(e) => return Err(e.to_string()),
    }

    let mut lyr_tokens = Vec::new();
    for token in lin_tokens {
        let mut detail = if token.detail.len() != 1 {
            token.detail.split_at(7).1.to_vec() // get information of reading and phonation
        } else {
            vec![String::from("unknown"); 2]
        };

        lyr_tokens.push(LyrianToken::new(
            token.text.to_string(),
            mem::replace(&mut detail[0], String::from("")),
            mem::replace(&mut detail[1], String::from("")),
        ));
    }

    Ok(lyr_tokens)
}

#[cfg(test)]
mod morphological_analysis_test {
    use crate::morphological_analysis::{tokenize, LyrianToken};

    #[test]
    fn get_lyrian_tokens_from_text() {
        let text = "すもももももももものうち";
        match tokenize(text) {
            Ok(tokens) => {
                let expected = vec![
                    LyrianToken::new(
                        "すもも".to_string(),
                        "スモモ".to_string(),
                        "スモモ".to_string(),
                    ),
                    LyrianToken::new("も".to_string(), "モ".to_string(), "モ".to_string()),
                    LyrianToken::new("もも".to_string(), "モモ".to_string(), "モモ".to_string()),
                    LyrianToken::new("も".to_string(), "モ".to_string(), "モ".to_string()),
                    LyrianToken::new("もも".to_string(), "モモ".to_string(), "モモ".to_string()),
                    LyrianToken::new("の".to_string(), "ノ".to_string(), "ノ".to_string()),
                    LyrianToken::new("うち".to_string(), "ウチ".to_string(), "ウチ".to_string()),
                ];
                assert_eq!(tokens, expected)
            }
            Err(msg) => panic!("{}", msg),
        }
    }

    #[test]
    fn get_word_length_on_mora() {
        let token = LyrianToken::new(
            "大空".to_string(),
            "オオゾラ".to_string(),
            "オーゾラ".to_string(),
        );
        assert_eq!(token.length(false), 4)
    }

    #[test]
    fn get_word_length_on_syllable() {
        let token = LyrianToken::new(
            "大空".to_string(),
            "オオゾラ".to_string(),
            "オーゾラ".to_string(),
        );
        assert_eq!(token.length(true), 3)
    }

    #[test]
    fn get_length_of_word_that_has_lower_case() {
        let token = LyrianToken::new(
            "ジョバンニ".to_string(),
            "ジョバンニ".to_string(),
            "ジョバンニ".to_string(),
        );
        assert_eq!(token.length(false), 4)
    }

    #[test]
    fn get_symbol_length() {
        let token = LyrianToken::new("。".to_string(), "。".to_string(), "。".to_string());
        assert_eq!(token.length(false), 0)
    }

    // #[test]
    // fn get_length_of_word_that_has_voiceless_sound() {
    //     let token = LyrianToken::new("桜".to_string(), "サクラ".to_string(), "サクラ".to_string());
    //     assert_eq!(token.length(false, true, false), 2)
    // }

    // #[test]
    // fn get_length_of_word_that_has_smooth_vowel_sound() {
    //     let token = LyrianToken::new(
    //         "だいたい".to_string(),
    //         "ダイタイ".to_string(),
    //         "ダイタイ".to_string(),
    //     );
    //     assert_eq!(token.length(false, false, true), 2)
    // }
}
