use chumsky::extra::ParserExtra;
use chumsky::Parser;
use chumsky::prelude::Input;

pub trait BisCombinators<'ast, I: Input<'ast>, O, E: ParserExtra<'ast, I>> {
    fn bis_padded(self,
        padding: impl Parser<'ast, I, (), E> + Clone
    ) -> impl Parser<'ast, I, O, E>;
}

impl<
    'ast,
    I: Input<'ast>,
    O,
    E: ParserExtra<'ast, I>,
    Ext: Parser<'ast, I, O, E> + Clone
> BisCombinators<'ast, I, O, E> for Ext {
    fn bis_padded(self, padding: impl Parser<'ast, I, (), E> + Clone) -> impl Parser<'ast, I, O, E> + Clone {
        padding.clone().ignore_then(self).then_ignore(padding)
    }
}