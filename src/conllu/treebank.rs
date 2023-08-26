#[derive(Debug, Default, PartialEq)]
pub struct Treebank {
    pub sentences: Vec<Sentence>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Sentence {
    pub words: Vec<Word>,
}

#[derive(Debug, PartialEq)]
pub struct Word {
    pub id: String,
    pub form: String,
    pub lemma: String,
    pub upos: Upos,
    pub xpos: String,
    pub feats: Vec<(String, String)>,
    // TODO
    pub head: String,
    // TODO
    pub deprel: String,
    pub misc: String,
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
