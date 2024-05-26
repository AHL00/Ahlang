use chumsky::prelude::*;

use crate::token::{Literal, Token};

pub type Span = SimpleSpan<usize>;

fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<(Token<'a>, Span)>, extra::Err<Rich<'a, char, Span>>>
{
    let ignore_underscores = just('_').repeated().then(text::digits(10)).or_not();

    let num = text::digits(10)
        // .ignore_then(just('_'))
        // .or_not()
        .then(ignore_underscores)
        .then(
            just('.')
                .then(text::digits(10))
                .then(ignore_underscores)
                .or_not(),
        )
        .to_slice()
        // .from_str()
        // .unwrapped()
        .map(|x| Token::Literal(Literal::Integer(x)));

    // // A parser for strings
    let string = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .to_slice()
        .map(|x| Token::Literal(Literal::String(x)));

    // let char = just("'")
    //     .ignore_then(none_of('"').repeated())
    //     .then_ignore(just('"'))
    //     .to_slice()
    //     .map(|x| Token::Literal(Literal::Char(x)));

    // // A parser for operators
    // let op = one_of("+*-/!=")
    //     .repeated()
    //     .at_least(1)
    //     .to_slice()
    //     .map(Token::Op);

    // // A parser for control characters (delimiters, semicolons, etc.)
    // let ctrl = one_of("()[]{};,").map(Token::Ctrl);

    // A parser for identifiers and keywords
    // let ident = text::ascii::ident().map(|ident: &str| match ident {
    //     "fn" => Token::Fn,
    //     "let" => Token::Let,
    //     "print" => Token::Print,
    //     "if" => Token::If,
    //     "else" => Token::Else,
    //     "true" => Token::Bool(true),
    //     "false" => Token::Bool(false),
    //     "null" => Token::Null,
    //     _ => Token::Ident(ident),
    // });

    // A single token can be one of the above
    let token = num.or(string);
    // .or(str_).or(op).or(ctrl).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    #[test]
    fn numbers() {
        let src = "
        // a weird number
        120_05.03_2";

        let (tokens, errs) = super::lexer().parse(src).into_output_errors();

        println!("Errors: {:?}", errs);
        println!("Tokens: {:?}", tokens);

        assert_eq!(errs.len() ,0);
    }

    #[test]
    fn strings() {
        let src = "
        \"Hello there!\"
        ";

        let (tokens, errs) = super::lexer().parse(src).into_output_errors();

        println!("Errors: {:?}", errs);
        println!("Tokens: {:?}", tokens);

        assert_eq!(errs.len(), 0);
    }
}
