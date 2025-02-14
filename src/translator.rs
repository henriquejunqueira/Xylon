use std::collections::HashMap;

pub struct Translator {
    keywords: HashMap<String, String>,
}

impl Translator {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Adicionando traduções para 'var'
        keywords.insert("variable".to_string(), "var".to_string()); // Inglês, Espanhol, Alemão, Francês
        keywords.insert("variável".to_string(), "var".to_string()); // Português do Brasil

        // Adicionando traduções para 'const'
        keywords.insert("constant".to_string(), "const".to_string()); // Inglês
        keywords.insert("constante".to_string(), "const".to_string()); // Português, Francês, Espanhol
        keywords.insert("Konstante".to_string(), "const".to_string()); // Alemão

        Self { keywords }
    }

    pub fn translate(&self, word: &str) -> String {
        self.keywords
            .get(word)
            .cloned()
            .unwrap_or_else(|| word.to_string())
    }
}
