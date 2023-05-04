use antlr_rust::token::Token;

pub(crate) trait TokenExt: Token {
    fn get_line_as_usize(&self) -> usize {
        usize::try_from(self.get_line()).unwrap_or_default()
    }

    fn get_column_as_usize(&self) -> usize {
        usize::try_from(self.get_column()).unwrap_or_default()
    }
}

impl<T: Token + ?Sized> TokenExt for T {}
