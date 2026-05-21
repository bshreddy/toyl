use chumsky::prelude::*;

enum AST {
    eof
}

fn parser<'src>() -> impl Parser<'src, &'src str, AST> {
    end().map(|_| AST::eof)
}
