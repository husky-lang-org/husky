use salsa::{storage::HasJar, DbWithJar};

use crate::*;

pub trait WordDb: DbWithJar<WordJar> {
    fn it_word_owned(&self, data: String) -> Word;

    fn it_word_borrowed(&self, data: &str) -> Word;

    fn dt_word(&self, data: Word) -> &str;

    fn word_jar(&self) -> &WordJar;

    fn word_menu(&self) -> &WordMenu;
}

impl<T> WordDb for T
where
    T: DbWithJar<WordJar>,
{
    fn it_word_owned(&self, data: String) -> Word {
        Word::from_owned(self, data)
    }

    fn it_word_borrowed(&self, data: &str) -> Word {
        Word::from_ref(self, data)
    }

    fn dt_word(&self, word: Word) -> &str {
        word.data(self)
    }

    fn word_jar(&self) -> &WordJar {
        &<Self as HasJar<WordJar>>::jar(self).0
    }

    fn word_menu(&self) -> &WordMenu {
        self.word_jar()
            .word_menu_cell()
            .get_or_init(|| WordMenu::new(self))
    }
}
