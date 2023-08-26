#[derive(Debug, Default, PartialEq)]
pub struct Treebank {
    pub sentences: Vec<Sentence>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Sentence {
    pub words: Vec<Word>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Word {
    pub id: String,
    pub form: Option<String>,
    pub lemma: Option<String>,
    pub upos: Option<Upos>,
    pub xpos: Option<String>,
    pub feats: Option<Vec<(String, String)>>,
    // TODO
    pub head: Option<String>,
    // TODO
    pub deprel: Option<String>,
    // TODO
    pub deps: Option<String>,
    // TODO
    pub misc: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Upos {
    Adjective,
    Adposition,
    Adverb,
    Auxiliary,
    CoordinatingConjunction,
    Determiner,
    Interjection,
    Noun,
    Numeral,
    Particle,
    Pronoun,
    ProperNoun,
    Punctuation,
    SubordinatingConjunction,
    Symbol,
    Verb,
    Other,
}
