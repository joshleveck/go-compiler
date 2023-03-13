use crate::lexer;
use crate::token::TokenTypes;

fn assert_tokens(src: &str, expected: Vec<TokenTypes>) {
    let data: Vec<u8> = src.as_bytes().to_vec();
    let lexer = lexer::Lexer::new(&data);
    for (pos, token) in lexer.enumerate() {
        assert_eq!(token.token_type, expected[pos]);
    }
}

fn assert_token(src: &str, expected: TokenTypes) {
    assert_tokens(src, vec![expected]);
}

#[test]
fn test_numerical_tokems() {
    assert_token("1", TokenTypes::Int);
    assert_token("043", TokenTypes::Octal);
    assert_token("0x43", TokenTypes::Hex);
    assert_token("0X43", TokenTypes::Hex);
    assert_token("0b101", TokenTypes::Binary);
    assert_token("0B101", TokenTypes::Binary);

    let float_tests: Vec<&str> = vec![
        "1.0", "1.0e10", "1.0e+10", "1.0e-10", "1.0E10", "1.0E+10", "1.0E-10",
    ];

    for test in float_tests {
        assert_token(test, TokenTypes::Float);
    }

    let complex_tests: Vec<&str> = vec![
        "1.0i", "1.0e10i", "1.0e+10i", "1.0e-10i", "1.0E10i", "1.0E+10i", "1.0E-10i",
    ];

    for test in complex_tests {
        assert_token(test, TokenTypes::Imag);
    }
}

#[test]
fn test_text_literals() {
    assert_token("\"\"", TokenTypes::String);
    assert_token("\"hello\"", TokenTypes::String);
    assert_token("\"hello world\"", TokenTypes::String);
    // assert_token("\"hello \\\"world\\\"\"", TokenTypes::String);
    assert_token("`hello \\\\world`", TokenTypes::RawString);
}

#[test]
fn token_simple() {
    let pairs = vec![
        ("(", TokenTypes::LParen),
        (")", TokenTypes::RParen),
        ("{", TokenTypes::LBrace),
        ("}", TokenTypes::RBrace),
        ("[", TokenTypes::LBrack),
        ("]", TokenTypes::RBrack),
        (",", TokenTypes::Comma),
        (";", TokenTypes::Semicolon),
        (".", TokenTypes::Dot),
        ("...", TokenTypes::Ellipsis),
        ("|", TokenTypes::Or),
        ("||", TokenTypes::OrOr),
        ("|=", TokenTypes::OrAssign),
        ("!", TokenTypes::Not),
        ("!=", TokenTypes::Neq),
        ("^", TokenTypes::Caret),
        ("^=", TokenTypes::CaretAssign),
        ("%", TokenTypes::Percent),
        ("%=", TokenTypes::PercentAssign),
        ("&", TokenTypes::Amp),
        ("&&", TokenTypes::And),
        ("&=", TokenTypes::AndAssign),
        ("&^", TokenTypes::BitClear),
        ("&^=", TokenTypes::BitClearAssign),
        ("+", TokenTypes::Add),
        ("++", TokenTypes::Increment),
        ("+=", TokenTypes::AddAssign),
        ("-", TokenTypes::Sub),
        ("--", TokenTypes::Decrement),
        ("-=", TokenTypes::SubAssign),
        (":", TokenTypes::Colon),
        (":=", TokenTypes::Define),
        ("<", TokenTypes::Lss),
        ("<-", TokenTypes::Arrow),
        ("<=", TokenTypes::Leq),
        ("<<", TokenTypes::Lshift),
        ("<<=", TokenTypes::LshiftAssign),
        (">", TokenTypes::Gtr),
        (">=", TokenTypes::Geq),
        (">>", TokenTypes::Rshift),
        (">>=", TokenTypes::RshiftAssign),
        ("*", TokenTypes::Mul),
        ("*=", TokenTypes::MulAssign),
        ("=", TokenTypes::Assign),
        ("==", TokenTypes::Eql),
        ("/", TokenTypes::Quo),
        ("/=", TokenTypes::QuoAssign),
    ];

    for (src, kind) in pairs {
        assert_token(src, kind);
    }
}

#[test]
fn tokenize_keywords() {
    let pairs = [
        ("break", TokenTypes::Break),
        ("case", TokenTypes::Case),
        ("chan", TokenTypes::Chan),
        ("const", TokenTypes::Const),
        ("continue", TokenTypes::Continue),
        ("default", TokenTypes::Default),
        ("defer", TokenTypes::Defer),
        ("else", TokenTypes::Else),
        ("fallthrough", TokenTypes::Fallthrough),
        ("for", TokenTypes::For),
        ("func", TokenTypes::Func),
        ("go", TokenTypes::Go),
        ("goto", TokenTypes::Goto),
        ("if", TokenTypes::If),
        ("import", TokenTypes::Import),
        ("interface", TokenTypes::Interface),
        ("map", TokenTypes::Map),
        ("package", TokenTypes::Package),
        ("range", TokenTypes::Range),
        ("return", TokenTypes::Return),
        ("select", TokenTypes::Select),
        ("struct", TokenTypes::Struct),
        ("switch", TokenTypes::Switch),
        ("type", TokenTypes::Type),
        ("var", TokenTypes::Var),
        ("true", TokenTypes::True),
        ("false", TokenTypes::False),
        ("nil", TokenTypes::Nil),
    ];

    for (s, k) in pairs {
        assert_token(s, k);
    }
}

#[test]
fn tokenize_package_declaration() {
    assert_tokens(
        "package main",
        vec![TokenTypes::Package, TokenTypes::Identifier],
    );
}


#[test]
fn tokenize_simple_import() {
    assert_tokens(
        "import \"fmt\"",
        vec![TokenTypes::Import, TokenTypes::String],
    );
}

#[test]
fn tokenize_simple_assignment() {
    assert_tokens(
        "someVar := 23 + 45",
        vec![
            TokenTypes::Identifier,
            TokenTypes::Define,
            TokenTypes::Int,
            TokenTypes::Add,
            TokenTypes::Int,
        ],
    );
}

#[test]
fn tokenize_hello() {
    let src = r#"package main
import "fmt"
func main() {
	fmt.Println("Hello, world!")
}
"#;

    let expected = vec![
        TokenTypes::Package,
        TokenTypes::Identifier,
        //TokenTypes::Semicolon,
        TokenTypes::Import,
        TokenTypes::String,
        //TokenTypes::Semicolon,
        TokenTypes::Func,
        TokenTypes::Identifier,
        TokenTypes::LParen,
        TokenTypes::RParen,
        TokenTypes::LBrace,
        TokenTypes::Identifier,
        TokenTypes::Dot,
        TokenTypes::Identifier,
        TokenTypes::LParen,
        TokenTypes::String,
        TokenTypes::RParen,
        //TokenTypes::Semicolon,
        TokenTypes::RBrace,
        //TokenTypes::Semicolon,
        TokenTypes::Eof
    ];

    assert_tokens(src, expected);
}

// =====
// Comments
// =====

#[test]
fn tokenize_simple_assignment_with_inline_comment() {
    assert_tokens(
        "someVar /* someVar is a variable; and I'm a COMMENT! */ := 23 + 45",
        vec![
            TokenTypes::Identifier,
            TokenTypes::Define,
            TokenTypes::Int,
            TokenTypes::Add,
            TokenTypes::Int,
        ],
    );
}

#[test]
fn tokenize_hello_with_comments() {
    let src = r#"// This is a line comment.
// And another!
// All of these should be treated as a single contiguous whitespace block.
// Even this one!
package main
import "fmt"
func main() {
	fmt.Println("Hello, world!")
}
"#;

    let expected = vec![
        TokenTypes::Package,
        TokenTypes::Identifier,
        // TokenTypes::Semicolon,
        TokenTypes::Import,
        TokenTypes::String,
        // TokenTypes::Semicolon,
        TokenTypes::Func,
        TokenTypes::Identifier,
        TokenTypes::LParen,
        TokenTypes::RParen,
        TokenTypes::LBrace,
        TokenTypes::Identifier,
        TokenTypes::Dot,
        TokenTypes::Identifier,
        TokenTypes::LParen,
        TokenTypes::String,
        TokenTypes::RParen,
        // TokenTypes::Semicolon,
        TokenTypes::RBrace,
        // TokenTypes::Semicolon,
        TokenTypes::Eof
    ];

    assert_tokens(src, expected);
}
