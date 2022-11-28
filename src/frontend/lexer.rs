use super::error::{ErrorReporter, ErrorType};
use super::util::{FileIterator, FileLocation, TokenIterator};

/// A enumerate of all tokens possibly generated by [Lexer][crate::parser::lexer::Lexer].
#[derive(Debug)]
pub enum Token {
    Str(String),
    Integer(i64),
    Float(f64),
    Id(String),
    Key(Keyword),
    Op(Operator),
}

/// A enum of all keywords
#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    /// true
    True,
    /// false
    False,
    /// do
    Do,
    /// end
    End,
    /// if
    If,
    /// then
    Then,
    /// else
    Else,
    /// elsif
    Elsif,
    /// case
    Case,
    /// in
    In,
    /// for
    For,
    /// nil
    Nil,
    /// assert
    Assert,
    /// return
    Return,
    /// break
    Break,
    /// continue
    Continue,
    /// loop
    Loop,
    /// class
    Class,
    /// def
    Def,
}

/// A enum of all operators
#[derive(Debug, Clone, Copy)]
pub enum Operator {
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "**"
    Exp,
    /// "*"
    Mul,
    /// "//"
    DivFloor,
    /// "/"
    Div,
    /// "%"
    Mod,
    /// ".."
    Range,
    /// "and"
    And,
    /// "or"
    Or,
    /// "not"
    Not,
    /// ">"
    Gt,
    /// ">="
    Ge,
    /// "=="
    Eq,
    /// "<>"
    Ne,
    /// "<"
    Lt,
    /// "<"
    Le,
    /// "="
    Assign,
    /// ","
    Comma,
    /// "."
    Member,
    /// "|>"
    Pipeline,
    /// "("
    LPar,
    /// ")"
    RPar,
    /// "["
    LBrk,
    /// "]"
    RBrk,
    /// "{"
    LBrc,
    /// "}"
    RBrc,
    /// ":"
    Colon,
}

/// The lexical analyzer for Diatom.
///
/// Note: This lexer assumes that newline character only contains '\n'. Any '\r' is ignored and
/// does not count as a newline.
///
/// # Examples
///
/// ```rust
/// extern crate diatom;
/// use diatom::Lexer;
///
/// let file =
/// r#"#!/usr/bin/env diatom
///
/// a = 1..5
/// def plus_one(l) do
///     for x in l do x = x + 1 end
/// end
/// plus_one(a)
/// "#.to_string();
/// let lexer = Lexer::new(file, "test.dm".to_string());
///
/// assert!(!lexer.has_error());
/// ```
pub struct Lexer {
    file_content: String,
    file_name: String,
    error_reporter: ErrorReporter,
    tokens: Vec<(Token, FileLocation)>,
}

impl Lexer {
    pub fn new(file_content: String, file_name: String) -> Self {
        let mut lexer = Self {
            file_content,
            file_name,
            error_reporter: ErrorReporter::new(),
            tokens: vec![],
        };
        lexer.consume();
        lexer
    }

    pub fn has_error(&self) -> bool {
        !self.error_reporter.is_empty()
    }

    pub fn render_error(&self) -> String {
        self.error_reporter
            .render(&self.file_content, &self.file_name)
            .expect(
                "Render error message failed. If you see this message, please make a bug report.",
            )
    }

    /// Return an iterator tokens.
    pub(crate) fn iter(&self) -> TokenIterator {
        TokenIterator::new(&self.tokens)
    }

    /// Consume numeric types, aka int & float.
    fn consume_num(
        iter: &mut FileIterator,
    ) -> Result<(Token, FileLocation), (ErrorType, FileLocation)> {
        fn consume_int(s: &str) -> Result<i64, String> {
            let mut i: i64 = 0;
            if s.starts_with("0x") || s.starts_with("0X") {
                let mut iter = s.chars().skip(2);
                loop {
                    let c = iter.next();
                    match c {
                        Some(c) => {
                            let digit = match c {
                                '0'..='9' => c as i64 - '0' as i64,
                                'a'..='f' => c as i64 - 'a' as i64 + 10,
                                'A'..='F' => c as i64 - 'A' as i64 + 10,
                                _ => {
                                    return Err("Invalid Digit".to_string());
                                }
                            };
                            match i.checked_mul(16) {
                                Some(result) => {
                                    i = result;
                                }
                                None => {
                                    return Err("Integer overflow 64-bits.".to_string());
                                }
                            };
                            match i.checked_add(digit) {
                                Some(result) => {
                                    i = result;
                                }
                                None => {
                                    return Err("Integer overflow 64-bits.".to_string());
                                }
                            }
                        }
                        None => break,
                    }
                }
                return Ok(i);
            }

            if s.starts_with("0o") || s.starts_with("0O") {
                let mut iter = s.chars().skip(2);
                loop {
                    let c = iter.next();
                    match c {
                        Some(c) => match c {
                            '0'..='7' => {
                                let digit = c as i64 - '0' as i64;
                                match i.checked_mul(8) {
                                    Some(result) => {
                                        i = result;
                                    }
                                    None => {
                                        return Err("Integer overflow 64-bits.".to_string());
                                    }
                                };
                                match i.checked_add(digit) {
                                    Some(result) => {
                                        i = result;
                                    }
                                    None => {
                                        return Err("Integer overflow 64-bits.".to_string());
                                    }
                                }
                            }
                            _ => {
                                return Err("Invalid Digit".to_string());
                            }
                        },
                        None => break,
                    }
                }
                return Ok(i);
            }
            if s.starts_with("0b") || s.starts_with("0B") {
                let mut iter = s.chars().skip(2);
                loop {
                    let c = iter.next();
                    match c {
                        Some(c) => match c {
                            '0'..='1' => {
                                let digit = c as i64 - '0' as i64;
                                match i.checked_mul(2) {
                                    Some(result) => {
                                        i = result;
                                    }
                                    None => {
                                        return Err("Integer overflow 64-bits.".to_string());
                                    }
                                };
                                match i.checked_add(digit) {
                                    Some(result) => {
                                        i = result;
                                    }
                                    None => {
                                        return Err("Integer overflow 64-bits.".to_string());
                                    }
                                }
                            }
                            _ => {
                                return Err("Invalid Digit".to_string());
                            }
                        },
                        None => break,
                    }
                }
                return Ok(i);
            }

            let mut iter = s.chars();
            loop {
                let c = iter.next();
                match c {
                    Some(c) => match c {
                        '0'..='9' => {
                            let digit = c as i64 - '0' as i64;
                            match i.checked_mul(10) {
                                Some(result) => {
                                    i = result;
                                }
                                None => {
                                    return Err("Integer overflow 64-bits.".to_string());
                                }
                            };
                            match i.checked_add(digit) {
                                Some(result) => {
                                    i = result;
                                }
                                None => {
                                    return Err("Integer overflow 64-bits.".to_string());
                                }
                            }
                        }
                        _ => {
                            return Err("Invalid Digit".to_string());
                        }
                    },
                    None => break,
                }
            }
            Ok(i)
        }
        let mut num = String::new();
        let start = iter.get_location();

        let mut e_flag = false; // Flag for '123e+5' & '123e-4'
        let mut float_flag = false;
        loop {
            let c = iter.peek2();
            match c {
                (Some('.'), Some('.')) => {
                    break;
                }
                (Some(c), _) => {
                    match c {
                        '+' | '-' if e_flag => num.push(c),
                        '_' => (),
                        '.' => num.push(c),
                        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => break,
                        ' ' | '\t' | '\r' | '\n' => break,
                        c => num.push(c),
                    }

                    if c == 'e' || c == 'E' {
                        e_flag = true;
                        float_flag = true;
                    } else {
                        e_flag = false;
                    }

                    if c == '.' {
                        float_flag = true;
                    }

                    iter.next();
                }
                (None, _) => break,
            }
        }

        let end = iter.get_location();
        let location = FileLocation::new(start, end);

        if float_flag {
            let float = num.parse::<f64>();
            match float {
                Ok(f) => Ok((Token::Float(f), location)),
                Err(e) => Err((ErrorType::InvalidNum(format!("{e}")), location)),
            }
        } else {
            let int = consume_int(&num);
            match int {
                Ok(i) => Ok((Token::Integer(i), location)),
                Err(e) => Err((ErrorType::InvalidNum(e), location)),
            }
        }
    }

    /// Consume keyword or Identifier
    fn consume_id_or_key(
        iter: &mut FileIterator,
    ) -> Result<(Token, FileLocation), (ErrorType, FileLocation)> {
        let mut name = String::new();
        let start = iter.get_location();
        loop {
            match iter.peek() {
                Some('_') => name.push('_'),
                Some(' ' | '\r' | '\n' | '\t' | '!'..='/' | ':'..='@' | '['..='`' | '{'..='~') => {
                    break
                }
                Some(c) => name.push(c),
                None => break,
            }
            iter.next();
        }
        let location = FileLocation::new(start, iter.get_location());
        match name.as_str() {
            "and" => Ok((Token::Op(Operator::And), location)),
            "or" => Ok((Token::Op(Operator::Or), location)),
            "not" => Ok((Token::Op(Operator::Not), location)),
            "true" => Ok((Token::Key(Keyword::True), location)),
            "false" => Ok((Token::Key(Keyword::False), location)),
            "do" => Ok((Token::Key(Keyword::Do), location)),
            "end" => Ok((Token::Key(Keyword::End), location)),
            "if" => Ok((Token::Key(Keyword::If), location)),
            "then" => Ok((Token::Key(Keyword::Then), location)),
            "else" => Ok((Token::Key(Keyword::Else), location)),
            "elsif" => Ok((Token::Key(Keyword::Elsif), location)),
            "case" => Ok((Token::Key(Keyword::Case), location)),
            "in" => Ok((Token::Key(Keyword::In), location)),
            "for" => Ok((Token::Key(Keyword::For), location)),
            "nil" => Ok((Token::Key(Keyword::Nil), location)),
            "return" => Ok((Token::Key(Keyword::Return), location)),
            "assert" => Ok((Token::Key(Keyword::Assert), location)),
            "continue" => Ok((Token::Key(Keyword::Continue), location)),
            "break" => Ok((Token::Key(Keyword::Break), location)),
            "loop" => Ok((Token::Key(Keyword::Loop), location)),
            "class" => Ok((Token::Key(Keyword::Class), location)),
            "def" => Ok((Token::Key(Keyword::Def), location)),
            _ => Ok((Token::Id(name), location)),
        }
    }

    /// Consume string token
    fn consume_string(
        iter: &mut FileIterator,
    ) -> Result<(Token, FileLocation), (ErrorType, FileLocation)> {
        fn consume_escape(iter: &mut FileIterator) -> Result<char, ()> {
            /// Consume a hex escape sequence with n character exactly
            fn consume_hex_escape(iter: &mut FileIterator, count: u8) -> Result<char, ()> {
                let mut x: u32 = 0;
                for _ in 0..count {
                    x *= 16;
                    match iter.peek() {
                        Some(c @ '0'..='9') => x += c as u32 - '0' as u32,
                        Some(c @ 'a'..='f') => x += c as u32 - 'a' as u32 + 10,
                        Some(c @ 'A'..='F') => x += c as u32 - 'A' as u32 + 10,
                        _ => return Err(()),
                    }
                    iter.next();
                }
                match char::from_u32(x as u32) {
                    Some(c) => Ok(c),
                    None => Err(()),
                }
            }
            let c = iter.next();
            let c = match c {
                Some(c) => c,
                None => unreachable!(),
            };
            match c {
                '\\' => Ok('\\'),
                't' => Ok('\t'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                '\"' => Ok('\"'),
                '\'' => Ok('\''),
                'x' => consume_hex_escape(iter, 2),
                'u' => consume_hex_escape(iter, 4),
                'U' => consume_hex_escape(iter, 8),
                _ => Err(()),
            }
        }
        let start = iter.get_location();
        let start_char = iter.next();
        let mut result = String::new();
        let mut invalid = false;

        let is_single_quote = match start_char {
            Some('"') => false,
            Some('\'') => true,
            _ => unreachable!(),
        };
        loop {
            let c = iter.next();
            match c {
                Some(c) => match c {
                    '\\' => match consume_escape(iter) {
                        Ok(c) => result.push(c),
                        Err(()) => invalid = true,
                    },
                    '\'' if is_single_quote => {
                        if !invalid {
                            return Ok((
                                Token::Str(result),
                                FileLocation::new(start, iter.get_location()),
                            ));
                        } else {
                            return Err((
                                ErrorType::InvalidStr(
                                    "String contains invalid escape sequence.".to_string(),
                                ),
                                FileLocation::new(start, iter.get_location()),
                            ));
                        }
                    }
                    '"' if !is_single_quote => {
                        if !invalid {
                            return Ok((
                                Token::Str(result),
                                FileLocation::new(start, iter.get_location()),
                            ));
                        } else {
                            return Err((
                                ErrorType::InvalidStr(
                                    "String contains invalid escape sequence.".to_string(),
                                ),
                                FileLocation::new(start, iter.get_location()),
                            ));
                        }
                    }
                    c => result.push(c),
                },
                None => {
                    return Err((
                        ErrorType::InvalidStr("String is not terminated".to_string()),
                        FileLocation::new(start, iter.get_location()),
                    ))
                }
            }
        }
    }

    /// Consume operators
    fn consume_op(
        iter: &mut FileIterator,
    ) -> Result<(Token, FileLocation), (ErrorType, FileLocation)> {
        fn consume_next_2_char(iter: &mut FileIterator) -> FileLocation {
            let start = iter.get_location();
            iter.next();
            iter.next();
            let end = iter.get_location();
            FileLocation::new(start, end)
        }
        match iter.peek2() {
            (Some('/'), Some('/')) => {
                Ok((Token::Op(Operator::DivFloor), consume_next_2_char(iter)))
            }
            (Some('*'), Some('*')) => Ok((Token::Op(Operator::Exp), consume_next_2_char(iter))),
            (Some('.'), Some('.')) => Ok((Token::Op(Operator::Range), consume_next_2_char(iter))),
            (Some('>'), Some('=')) => Ok((Token::Op(Operator::Ge), consume_next_2_char(iter))),
            (Some('<'), Some('=')) => Ok((Token::Op(Operator::Le), consume_next_2_char(iter))),
            (Some('='), Some('=')) => Ok((Token::Op(Operator::Eq), consume_next_2_char(iter))),
            (Some('<'), Some('>')) => Ok((Token::Op(Operator::Ne), consume_next_2_char(iter))),
            (Some('|'), Some('>')) => {
                Ok((Token::Op(Operator::Pipeline), consume_next_2_char(iter)))
            }
            (Some(c), _) => {
                let start = iter.get_location();
                iter.next();
                let location = FileLocation::new(start, iter.get_location());
                match c {
                    '+' => Ok((Token::Op(Operator::Plus), location)),
                    '-' => Ok((Token::Op(Operator::Minus), location)),
                    '*' => Ok((Token::Op(Operator::Mul), location)),
                    '/' => Ok((Token::Op(Operator::Div), location)),
                    '%' => Ok((Token::Op(Operator::Mod), location)),
                    '=' => Ok((Token::Op(Operator::Assign), location)),
                    ',' => Ok((Token::Op(Operator::Comma), location)),
                    '.' => Ok((Token::Op(Operator::Member), location)),
                    '>' => Ok((Token::Op(Operator::Gt), location)),
                    '<' => Ok((Token::Op(Operator::Lt), location)),
                    '(' => Ok((Token::Op(Operator::LPar), location)),
                    ')' => Ok((Token::Op(Operator::RPar), location)),
                    '[' => Ok((Token::Op(Operator::LBrk), location)),
                    ']' => Ok((Token::Op(Operator::RBrk), location)),
                    '{' => Ok((Token::Op(Operator::LBrc), location)),
                    '}' => Ok((Token::Op(Operator::RBrc), location)),
                    ':' => Ok((Token::Op(Operator::Colon), location)),
                    _ => Err((ErrorType::InvalidOp(c.to_string()), location)),
                }
            }
            (None, _) => unreachable!(),
        }
    }

    /// Consume all tokens
    fn consume(&mut self) {
        let mut iter = FileIterator::new(&self.file_content);
        // Ignore shebang (#!...) at the beginning of the file
        if let (Some('#'), Some('!')) = iter.peek2() {
            loop {
                if let Some('\n') = iter.next() {
                    break;
                }
            }
        }
        // Start consuming characters
        loop {
            // Match some pattern requires 2 lookahead
            match iter.peek2() {
                (Some('-'), Some('-')) => {
                    // Ignore comment
                    loop {
                        let c = iter.peek();
                        match c {
                            Some(c) => {
                                if c == '\n' {
                                    break;
                                };
                            }
                            None => break,
                        }
                        iter.next();
                    }
                    continue;
                }
                (Some(c), _) => {
                    let result = match c {
                        '0'..='9' => Some(Self::consume_num(&mut iter)),
                        '"' | '\'' => Some(Self::consume_string(&mut iter)),
                        ' ' | '\t' | '\r' | '\n' => {
                            iter.next();
                            None
                        } // Ignore whitespace
                        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => {
                            Some(Self::consume_op(&mut iter))
                        }
                        _ => Some(Self::consume_id_or_key(&mut iter)),
                    };
                    match result {
                        Some(result) => match result {
                            Ok(x) => self.tokens.push(x),
                            Err((error, location)) => {
                                self.error_reporter.append("Lexer", location, error)
                            }
                        },
                        None => (),
                    }
                }
                (None, _) => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_int() {
        fn test_helper(s: &str, i: i64, should_fail: bool) {
            let mut iter = FileIterator::new(s);
            let result = Lexer::consume_num(&mut iter);
            if should_fail {
                assert!(
                    result.is_err(),
                    "Expected parse success! source = {s}, result = {:?}",
                    result
                );
            } else if let Ok((Token::Integer(j), _)) = result {
                assert_eq!(i, j);
            } else {
                assert!(
                    false,
                    "Expected parse failure! source = {s} , result = {:?}",
                    result
                );
            }
        }

        test_helper("1_23", 123, false);
        test_helper("0xf_f", 0xff, false);
        test_helper("0Xff", 0xff, false);
        test_helper("0b1__00011", 0b100011, false);
        test_helper("0o776_610_", 0o776610, false);
        test_helper("999+3", 999, false);
        test_helper("9_223_372_036_854_775_808", 0, true); // Overflow i64
        test_helper(
            "9_223_372_036_854_775_807",
            9_223_372_036_854_775_807,
            false,
        ); // Overflow i64
        test_helper("0x8000000000000000", 0, true); // Overflow i64
        test_helper("0x7fffffffffffffff", 9_223_372_036_854_775_807, false); // Overflow i64
        test_helper("0o1000000000000000000000", 0, true); // Overflow i64
        test_helper("0o777777777777777777777", 9_223_372_036_854_775_807, false); // Overflow i64
        test_helper(
            "0b1000000000000000000000000000000000000000000000000000000000000000",
            0,
            true,
        ); // Overflow i64
        test_helper(
            "0b111111111111111111111111111111111111111111111111111111111111111",
            9_223_372_036_854_775_807,
            false,
        ); // Overflow i64
        test_helper("0xfft", 0, true);
        test_helper("0O999", 0, true);
        test_helper("123y", 0, true);
    }

    #[test]
    fn test_consume_float() {
        fn test_helper(s: &str, i: f64, should_fail: bool) {
            let mut iter = FileIterator::new(s);
            let result = Lexer::consume_num(&mut iter);
            if should_fail {
                assert!(
                    result.is_err(),
                    "Expected parse success! source = {s}, result = {:?}",
                    result
                );
            } else if let Ok((Token::Float(j), _)) = result {
                assert_eq!(i, j);
            } else {
                assert!(
                    false,
                    "Expected parse failure! source = {s} , result = {:?}",
                    result
                );
            }
        }

        test_helper("123e14", 123e14, false);
        test_helper("1_23e_14", 123e14, false);
        test_helper("123E-14", 123e-14, false);
        test_helper("123.", 123., false);
        test_helper("0.01__2", 0.012, false);
        test_helper("123e1y", 0., true);
    }

    #[test]
    fn test_consume_string() {
        fn test_helper(s: &str, i: &str, should_fail: bool) {
            let mut iter = FileIterator::new(s);
            let result = Lexer::consume_string(&mut iter);
            if should_fail {
                assert!(
                    result.is_err(),
                    "Expected parse success! source = {s}, result = {:?}",
                    result
                );
            } else if let Ok((Token::Str(j), _)) = result {
                assert_eq!(i, j);
            } else {
                assert!(
                    false,
                    "Expected parse failure! source = {s} , result = {:?}",
                    result
                );
            }
        }

        test_helper(
            r#""abs__0a™£´∂`'`c\'\"k\n\rkk\x09'""#,
            "abs__0a™£´∂`'`c\'\"k\n\rkk\x09'",
            false,
        );
        test_helper(
            r#"'rrr__0a™£´∂`"`c\'\"k\n\rkk\u00E9"'"#,
            "rrr__0a™£´∂`\"`c\'\"k\n\rkk\u{00E9}\"",
            false,
        );
        test_helper("'\\xaz'", "", true);
        test_helper("'\\u@0'", "", true);
        test_helper("'\\U0000fFFf'", "\u{ffff}", false);
        test_helper("'\\ufFff'", "\u{ffff}", false);
        test_helper("'\\UdFFf'", "", true);
        test_helper("'\\uDfff'", "", true);
        test_helper("'", "", true);
    }

    #[test]
    fn test_dispatch() {
        let file = r#"#!/bin/diatom -c 
            fn fac(a) do 
                b = 1 c =2 -- @@@@
                if a> 0 then return a *fac(a - 1) elsif 
                a == 0 then return 1 else return 0
                end end
            ß = fac(5) å = ß**3//0x11f |> fac 
            set = {'s', 'ma\u00E9', 65e52, 0b00110} dict = {:}.insert(('key', 98)) 
            🐶🐱 <> "Doa\x09 and cat'?'" 
            "#;
        let lexer = Lexer::new(file.to_string(), "unit_test.dm".to_string());
        for token in lexer.tokens.iter() {
            println!("{:?}", token.0);
        }

        if lexer.has_error() {
            println!("{}", lexer.render_error());
        }
        assert!(!lexer.has_error());
    }
}