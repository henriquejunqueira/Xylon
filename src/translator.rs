use std::collections::HashMap;

pub struct Translator {
    keywords: HashMap<String, String>,
}

impl Translator {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Adding translations for 'var'
        keywords.insert("variable".to_string(), "var".to_string()); // English, Spanish, German, French
        keywords.insert("variável".to_string(), "var".to_string()); // Brazilian Portuguese

        // Adding translations for 'const'
        keywords.insert("constant".to_string(), "const".to_string()); // English
        keywords.insert("constante".to_string(), "const".to_string()); // Brazilian Portuguese, French, Spanish
        keywords.insert("konstante".to_string(), "const".to_string()); // German

        // Adding translations for 'println!'
        keywords.insert("write".to_string(), "println!".to_string()); // English
        keywords.insert("escribir".to_string(), "println!".to_string()); // Spanish
        keywords.insert("écrire".to_string(), "println!".to_string()); // French
        keywords.insert("schreiben".to_string(), "println!".to_string()); // German
        keywords.insert("escreva".to_string(), "println!".to_string()); // Brazilian Portuguese

        // Adding translations for 'if'
        keywords.insert("if".to_string(), "if".to_string()); // English
        keywords.insert("si".to_string(), "if".to_string()); // Spanish, French
        keywords.insert("Wenn".to_string(), "if".to_string()); // German
        keywords.insert("se".to_string(), "if".to_string()); // Brazilian Portuguese

        // Adding translations for 'else'
        keywords.insert("else".to_string(), "else".to_string()); // English
        keywords.insert("si_no".to_string(), "else".to_string()); // Spanish
        keywords.insert("sinon".to_string(), "else".to_string()); // French
        keywords.insert("wenn_nicht".to_string(), "else".to_string()); // German
        keywords.insert("senão".to_string(), "else".to_string()); // Brazilian Portuguese

        Self { keywords }
    }

    pub fn translate(&self, word: &str) -> String {
        self.keywords
            .get(&word.to_lowercase()) // <- Convert to lowercase before searching
            .cloned()
            .unwrap_or_else(|| word.to_string())
    }
}
