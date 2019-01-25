// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate codespan;
extern crate codespan_reporting;
extern crate regex;

mod location;
mod char_locations;
mod parser_source;
mod source;
mod comment;

use self::regex::{RegexSetBuilder, RegexSet};

use self::char_locations::CharLocations;
use self::comment::*;
use self::source::Source;

use self::LexicalError::*;
use self::location::{spanned, ByteOffset, Line, Column, ColumnOffset, Span};
pub use self::location::{Position, Spanned, Location};
pub use self::parser_source::ParserSource;


#[derive(Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),

    StringLiteral(String),
    IntLiteral(i64),
    ByteLiteral(u8),
    FloatLiteral(f64),
    DocComment(Comment),

    // region [builtin datatype]
    Text,
    Int,
    Float,
    Numeric,
    Timestamp,
    Datetime,
    Date,
    Time,
    // endregion

    // region [Symbol]
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `|`
    Pipe,
    /// `||`
    DoublePipe,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// `.`
    Period,
    /// `=`
    Equal,
    /// `!=`,`<>`, `^=`, `~=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
    /// `+`
    PlusSign,
    /// `-`
    MinusSign,
    /// `*`
    Asterisk,
    /// `/`
    Solidus,

    // endregion

    // region [keywords]
    All,
    And,
    As,
    Asc,
    Both,
    By,
    Case,
    Cross,
    Desc,
    Distinct,
    Dual,
    Else,
    End,
    Except,
    From,
    Full,
    Group,
    Having,
    In,
    Is,
    Inner,
    Intersect,
    Join,
    Leading,
    Left,
    Limit,
    Minus,
    Not,
    Null,
    Offset,
    On,
    Or,
    Order,
    Outer,
    Right,
    Select,
    Skip,
    Then,
    Trailing,
    Union,
    Unique,
    When,
    Where,
    // endregion

    // region [function]
    Abs,
    Avg,
    AvgIf,
    BTrim,
    Cast,
    Ceil,
    Ceiling,
    Coalesce,
    Cos,
    Concat,
    Count,
    CountIf,
    Day,
    DayAdd,
    DaySub,
    Decode,
    DenseRank,
    Extract,
    Floor,
    Hour,
    HourAdd,
    HourSub,
    Length,
    Log,
    Log10,
    Lower,
    LPad,
    LTrim,
    Max,
    MaxIf,
    Median,
    MedianIf,
    Min,
    MinIf,
    Minute,
    MinuteAdd,
    MinuteSub,
    Month,
    MonthAdd,
    MonthSub,
    Now,
    Nvl,
    PadLeft,
    PadRight,
    Pow,
    Power,
    Replace,
    Reverse,
    Rank,
    Round,
    Sign,
    Sin,
    Sqrt,
    Stddev,
    StddevIf,
    RPad,
    RTrim,
    Second,
    SecondAdd,
    SecondSub,
    Substr,
    Substring,
    Sum,
    SumIf,
    Tan,
    Trim,
    TrimStart,
    TrimEnd,
    Upper,
    Year,
    YearAdd,
    YearSub,
    // endregion
    EOF, // Required for the layout algorithm
}

const REGEX_SOURCE:[(&str, Token); 123] = [
    // region [keyword]
    ("^([Aa][Ll][Ll])$", Token::All),
    ("^([Aa][Nn][Dd])$", Token::And),
    ("^([Aa][Ss])$", Token::As),
    ("^([Aa][Ss][Cc])$", Token::Asc),
    ("^([Bb][Oo][Tt][Hh])$", Token::Both),
    ("^([Bb][Yy])$", Token::By),
    ("^([Cc][Aa][Ss][Ee])$", Token::Case),
    ("^([Cc][Rr][Oo][Ss][Ss])$", Token::Cross),
    ("^([Dd][Ee][Ss][Cc])$", Token::Desc),
    ("^([Dd][Ii][Ss][Tt][Ii][Nn][Cc][Tt])$", Token::Distinct),
    ("^([Dd][Uu][Aa][Ll])$", Token::Dual),
    ("^([Ee][Ll][Ss][Ee])$", Token::Else),
    ("^([Ee][Nn][Dd])$", Token::End),
    ("^([Ee][Xx][Cc][Ee][Pp][Tt])$", Token::Except),
    ("^([Ff][Rr][Oo][Mm])$", Token::From),
    ("^([Ff][Uu][Ll][Ll])$", Token::Full),
    ("^([Gg][Rr][Oo][Uu][Pp])$", Token::Group),
    ("^([Hh][Aa][Vv][Ii][Nn][Gg])$", Token::Having),
    ("^([Ii][Nn])$", Token::In),
    ("^([Ii][Ss])$", Token::Is),
    ("^([Ii][Nn][Nn][Ee][Rr])$", Token::Inner),
    ("^([Ii][Nn][Tt][Ee][Rr][Ss][Ee][Cc][Tt])$", Token::Intersect),
    ("^([Jj][Oo][Ii][Nn])$", Token::Join),
    ("^([Ll][Ee][Aa][Dd][Ii][Nn][Gg])$", Token::Leading),
    ("^([Ll][Ee][Ff][Tt])$", Token::Left),
    ("^([Ll][Ii][Mm][Ii][Tt])$", Token::Limit),
    ("^([Mm][Ii][Nn][Uu][Ss])$", Token::Minus),
    ("^([Nn][Oo][Tt])$", Token::Not),
    ("^([Nn][Uu][Ll][Ll])$", Token::Null),
    ("^([Oo][Ff][Ff][Ss][Ee][Tt])$", Token::Offset),
    ("^([Oo][Nn])$", Token::On),
    ("^([Oo][Rr])$", Token::Or),
    ("^([Oo][Rr][Dd][Ee][Rr])$", Token::Order),
    ("^([Oo][Uu][Tt][Ee][Rr])$", Token::Outer),
    ("^([Rr][Ii][Gg][Hh][Tt])$", Token::Right),
    ("^([Ss][Ee][Ll][Ee][Cc][Tt])$", Token::Select),
    ("^([Ss][Kk][Ii][Pp])$", Token::Skip),
    ("^([Tt][Hh][Ee][Nn])$", Token::Then),
    ("^([Tt][Rr][Aa][Ii][Ll][Ii][Nn][Gg])$", Token::Trailing),
    ("^([Uu][Nn][Ii][Oo][Nn])$", Token::Union),
    ("^([Uu][Nn][Ii][Qq][Uu][Ee])$", Token::Unique),
    ("^([Ww][Hh][Ee][Nn])$", Token::When),
    ("^([Ww][Hh][Ee][Rr][Ee])$", Token::Where),
    // endregion,

    // region [function]
    ("^([Aa][Bb][Ss])$", Token::Abs),
    ("^([Aa][Vv][Gg])$", Token::Avg),
    ("^([Aa][Vv][Gg][Ii][Ff])$", Token::AvgIf),
    ("^([Bb][Tt][Rr][Ii][Mm])$", Token::BTrim),
    ("^([Cc][Aa][Ss][Tt])$", Token::Cast),
    ("^([Cc][Ee][Ii][Ll])$", Token::Ceil),
    ("^([Cc][Ee][Ii][Ll][Ii][Nn][Gg])$", Token::Ceiling),
    ("^([Cc][Oo][Aa][Ll][Ee][Ss][Cc][Ee])$", Token::Coalesce),
    ("^([Cc][Oo][Ss])$", Token::Cos),
    ("^([Cc][Oo][Nn][Cc][Aa][Tt])$", Token::Concat),
    ("^([Cc][Oo][Uu][Nn][Tt])$", Token::Count),
    ("^([Cc][Oo][Uu][Nn][Tt][Ii][Ff])$", Token::CountIf),
    ("^([Dd][Aa][Yy])$", Token::Day),
    ("^([Dd][Aa][Yy]_[Aa][Dd][Dd])$", Token::DayAdd),
    ("^([Dd][Aa][Yy]_[Ss][Uu][Bb])$", Token::DaySub),
    ("^([Dd][Ee][Cc][Oo][Dd][Ee])$", Token::Decode),
    ("^([Dd][Ee][Nn][Ss][Ee]_[Rr][Aa][Nn][Kk])$", Token::DenseRank),
    ("^([Ee][Xx][Tt][Rr][Aa][Cc][Tt])$", Token::Extract),
    ("^([Ff][Ll][Oo][Oo][Rr])$", Token::Floor),
    ("^([Hh][Oo][Uu][Rr])$", Token::Hour),
    ("^([HH][Oo][Uu][Rr]_[Aa][Dd][Dd])$", Token::HourAdd),
    ("^([HH][Oo][Uu][Rr]_[Ss][Uu][Bb])$", Token::HourSub),
    ("^([Ll][Ee][Nn][Gg][Tt][Hh])$", Token::Length),
    ("^([Ll][Oo][Gg])$", Token::Log),
    ("^([Ll][Oo][Gg]10)$", Token::Log10),
    ("^([Ll][Oo][Ww][Ee][Rr])$", Token::Lower),
    ("^([Ll][Pp][Aa][Dd])$", Token::LPad),
    ("^([Ll][Tt][Rr][Ii][Mm])$", Token::LTrim),
    ("^([Mm][Aa][Xx])$", Token::Max),
    ("^([Mm][Aa][Xx][Ii][Ff])$", Token::MaxIf),
    ("^([Mm][Ee][Dd][Ii][Aa][Nn])$", Token::Median),
    ("^([Mm][Ee][Dd][Ii][Aa][Nn][Ii][Ff])$", Token::MedianIf),
    ("^([Mm][Ii][Nn])$", Token::Min),
    ("^([Mm][Ii][Nn][Ii][Ff])$", Token::MinIf),
    ("^([Mm][Ii][Nn][Uu][Tt][Ee])$", Token::Minute),
    ("^([Mm][Ii][Nn][Uu][Tt][Ee]_[Aa][Dd][Dd])$", Token::MinuteAdd),
    ("^([Mm][Ii][Nn][Uu][Tt][Ee]_[Ss][Uu][Bb])$", Token::MinuteSub),
    ("^([Mm][Oo][Nn][Tt][Hh])$", Token::Month),
    ("^([Mm][Oo][Nn][Tt][Hh]_[Aa][Dd][Dd])$", Token::MonthAdd),
    ("^([Mm][Oo][Nn][Tt][Hh]_[Ss][Uu][Bb])$", Token::MonthSub),
    ("^([Nn][Oo][Ww])$", Token::Now),
    ("^([Nn][Vv][Ll])$", Token::Nvl),
    ("^([Pp][Aa][Dd]_[Ll][Ee][Ff][Tt])$", Token::PadLeft),
    ("^([Pp][Aa][Dd]_[Rr][Ii][Gg][Hh][Tt])$", Token::PadRight),
    ("^([Pp][Oo][Ww])$", Token::Pow),
    ("^([Pp][Oo][Ww][Ee][Rr])$", Token::Power),
    ("^([Rr][Ee][Pp][Ll][Aa][Cc][Ee])$", Token::Replace),
    ("^([Rr][Ee][Vv][Ee][Rr][Ss][Ee])$", Token::Reverse),
    ("^([Rr][Aa][Nn][Kk])$", Token::Rank),
    ("^([Rr][Oo][Uu][Nn][Dd])$", Token::Round),
    ("^([Ss][Ii][Gg][Nn])$", Token::Sign),
    ("^([Ss][Ii][Nn])$", Token::Sin),
    ("^([Ss][Qq][Rr][Tt])$", Token::Sqrt),
    ("^([Ss][Tt][Dd][Dd][Ee][Vv])$", Token::Stddev),
    ("^([Ss][Tt][Dd][Dd][Ee][Vv][Ii][Ff])$", Token::StddevIf),
    ("^([Rr][Pp][Aa][Dd])$", Token::RPad),
    ("^([Rr][Tt][Rr][Ii][Mm])$", Token::RTrim),
    ("^([Ss][Ee][Cc][Oo][Nn][Dd])$", Token::Second),
    ("^([Ss][Ee][Cc][Oo][Nn][Dd]_[Aa][Dd][Dd])$", Token::SecondAdd),
    ("^([Ss][Ee][Cc][Oo][Nn][Dd]_[Ss][Uu][Bb])$", Token::SecondSub),
    ("^([Ss][Uu][Bb][Ss][Tt][Rr])$", Token::Substr),
    ("^([Ss][Uu][Bb][Ss][Tt][Rr][Ii][Nn][Gg])$", Token::Substring),
    ("^([Ss][Uu][Mm])$", Token::Sum),
    ("^([Ss][Uu][Mm][Ii][Ff])$", Token::SumIf),
    ("^([Tt][Aa][Nn])$", Token::Tan),
    ("^([Tt][Rr][Ii][Mm])$", Token::Trim),
    ("^([Tt][Rr][Ii][Mm]_[Ss][Tt][Aa][Rr][Tt])$", Token::TrimStart),
    ("^([Tt][Rr][Ii][Mm]_[Ee][Nn][Dd])$", Token::TrimEnd),
    ("^([Uu][Pp][Pp][Ee][Rr])$", Token::Upper),
    ("^([Yy][Ee][Aa][Rr])$", Token::Year),
    ("^([Yy][Ee][Aa][Rr]_[Aa][Dd][Dd])$", Token::YearAdd),
    ("^([Yy][Ee][Aa][Rr]_[Ss][Uu][Bb])$", Token::YearSub),
    // endregion

    // region
    ("^([Tt][Ee][Xx][Tt])$", Token::Text),
    ("^([Ii][Nn][Tt])$", Token::Int),
    ("^([Ff][Ll][Oo][Aa][Tt])$", Token::Float),
    ("^([Nn][Uu][Mm][Ee][Rr][Ii][Cc])$", Token::Numeric),
    ("^([Tt][Ii][Mm][Ee][Ss][Tt][Aa][Mm][Pp])$", Token::Timestamp),
    ("^([Dd][Aa][Tt][Ee][Tt][Ii][Mm][Ee])$", Token::Datetime),
    ("^([Dd][Aa][Tt][Ee])$", Token::Date),
    ("^([Tt][Ii][Mm][Ee])$", Token::Time),
    // endregion
];

impl<'input> Token<'input> {
    fn token_type(&self) -> &'static str {
        use self::Token::*;
        match self {
            Select | From | Where | Group | Order | Skip | Limit | By | In | Not => "keyword",
            Nvl | Day => "function",
            StringLiteral(_) => "string",
            IntLiteral(_) | FloatLiteral(_) => "number",
            DocComment(_) => "comment",
            _ => "unknown"
        }
    }
}

struct MatcherBuilder {
    regex_set: RegexSet,
}

impl MatcherBuilder {
    fn new() -> Self {
        let regex_set = RegexSetBuilder::new(REGEX_SOURCE.iter().map(|(s, _)| s)).build().unwrap();
        MatcherBuilder{regex_set}
    }

    fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> Matcher<'input, 'builder> {
        Matcher {
            text: s,
            consumed: 0,
            regex_set: &self.regex_set,
        }
    }
}

struct Matcher<'input, 'builder> {
    text: &'input str,
    consumed: usize,
    regex_set: &'builder regex::RegexSet
}

impl<'input, 'builder> Iterator for Matcher<'input, 'builder> {
    type Item = Token<'input>;

    //noinspection RsTypeCheck
    fn next(&mut self) -> Option<Self::Item> {
        let matches: Vec<usize> = self.regex_set.matches(self.text).into_iter().collect();
        if matches.len() > 1 {
            panic!("Non-unique matching error!");
        }
        if let Some(i) = matches.first() {
            let (_, t) = &REGEX_SOURCE[*i];
            Some(t.clone())
        } else { None }
    }
}

//noinspection ALL
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexicalError {

    /// empty char literal
    EmptyCharLiteral,

    /// unexpected character
    UnexpectedChar(char),

    /// unexpected end of file
    UnexpectedEof,

    /// unexpected escape code
    UnexpectedEscapeCode(char),

    /// unterminated string literal
    UnterminatedStringLiteral,

    /// cannot parse integer, probable overflow
    NonParseableInt,

    /// cannot parse hex literal, overflow
    HexLiteralOverflow,

    /// cannot parse hex literal, underflow
    HexLiteralUnderflow,

    /// wrong hex literal prefix, should start as '0x' or '-0x'
    HexLiteralWrongPrefix,

    /// cannot parse hex literal, incomplete
    HexLiteralIncomplete
}

pub type SpannedToken<'input> = Spanned<Token<'input>, Location>;

pub type SpannedError = Spanned<LexicalError, Location>;

pub fn is_identifier_start(ch: char) -> bool {
    match ch {
        '_' | 'a'...'z' | 'A'...'Z' | '\u{4E00}'...'\u{9FA5}' | '\u{F900}'...'\u{FA2D}' => true,
        _ => false,
    }
}

pub fn is_identifier_continue(ch: char) -> bool {
    match ch {
        '0'...'9' | '\'' => true,
        ch => is_identifier_start(ch),
    }
}

pub fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

pub fn is_hex(ch: char) -> bool {
    ch.is_digit(16)
}


pub struct Lexer<'input> {
    input: &'input str,
    chars: CharLocations<'input>,
    lookahead: Option<(Location, char)>,
    start_index: Position,
    builder: MatcherBuilder,
}

impl<'input> Lexer<'input> {
    pub fn new<S>(input: &'input S) -> Self where S: ?Sized + ParserSource {
        let mut chars = CharLocations::new(input);
        Lexer {
            input: input.src(),
            start_index: input.start_index(),
            lookahead: chars.next(),
            chars,
            builder: MatcherBuilder::new()
        }
    }

    pub fn tokenizer(self) -> Tokenizer<'input> {
        Tokenizer(self)
    }

    fn bump(&mut self) -> Option<(Location, char)> {
        match self.lookahead {
            Some((location, ch)) => {
                self.lookahead = self.chars.next();
                Some((location, ch))
            }
            None => None,
        }
    }


    fn skip_to_end(&mut self) {
        while let Some(_) = self.bump() {}
    }

    fn next_location(&self) -> Location {
        self.lookahead.as_ref().map_or(self.chars.location, |l| l.0)
    }

    fn identifier(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let (mut end, mut identifier) = self.take_while(start, is_identifier_continue);
        match self.lookahead {
            Some((_, c)) if c == '!' => {
                self.bump();
                end.column += 1.into();
                end.absolute += 1.into();
                identifier = self.slice(start, end);
            }
            _ => (),
        }

        let token =match self.builder.matcher(identifier).next() {
            Some(t) => t,
            None => Token::Identifier(identifier)
        };

//        let token = Token::Identifier(identifier);

        Ok(spanned(start, end, token))
    }

    fn numeric_literal(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let (end, int) = self.take_while(start, is_digit);
        let (start, end, token) = if int.chars().next().unwrap() == '.' {
            match self.lookahead {
                Some((_, ch)) if ch.is_whitespace() => (start, end, Token::FloatLiteral(int.parse().unwrap())),
                None => (start, end, Token::FloatLiteral(int.parse().unwrap())),
                _ => panic!("错误")
            }
        } else {
            match self.lookahead {
                Some((_, '.')) => {
                    self.bump(); // Skip '.'
                    let (end, float) = self.take_while(start, is_digit);
                    match self.lookahead {
                        Some((_, ch)) if is_identifier_start(ch) => {
                            return self.error(end, UnexpectedChar(ch));
                        }
                        _ => (start, end, Token::FloatLiteral(float.parse().unwrap())),
                    }
                }
                Some((_, 'x')) => {
                    self.bump(); // Skip 'x'
                    let int_start = self.next_location();
                    let (end, hex) = self.take_while(int_start, is_hex);
                    match int {
                        "0" | "-0" => match self.lookahead {
                            Some((_, ch)) if is_identifier_start(ch) => {
                                return self.error(end, UnexpectedChar(ch));
                            }
                            _ => {
                                if hex.is_empty() {
                                    return self.error(start, HexLiteralIncomplete);
                                }
                                let is_positive = int == "0";
                                match i64_from_hex(hex, is_positive) {
                                    Ok(val) => (start, end, Token::IntLiteral(val)),
                                    Err(err) => return self.error(start, err),
                                }
                            }
                        },
                        _ => return self.error(start, HexLiteralWrongPrefix),
                    }
                }
                Some((_, 'b')) => {
                    self.bump(); // Skip 'b'
                    let end = self.next_location();
                    match self.lookahead {
                        Some((pos, ch)) if is_identifier_start(ch) => {
                            return self.error(pos, UnexpectedChar(ch));
                        }
                        _ => {
                            if let Ok(val) = int.parse() {
                                (start, end, Token::ByteLiteral(val))
                            } else {
                                return self.error(start, NonParseableInt);
                            }
                        }
                    }
                }
                Some((start, ch)) if is_identifier_start(ch) => return self.error(start, UnexpectedChar(ch)),
                None | Some(_) => {
                    if let Ok(val) = int.parse() {
                        (start, end, Token::IntLiteral(val))
                    } else {
                        return self.error(start, NonParseableInt);
                    }
                }
            }
        };
        Ok(spanned(start, end, token))
    }

    fn test_lookahead<F: FnMut(char) -> bool>(&self, mut test: F) -> bool
    {
        self.lookahead.map_or(false, |(_, ch)| test(ch))
    }

    fn line_comment(&mut self, start: Location) -> Option<SpannedToken<'input>> {
        let (end, comment) = self.take_until(start, |ch| ch == '\n');

        if comment.starts_with("--") {
            let skip = if comment.starts_with("-- ") { 3 } else { 2 };
            let comment = Token::DocComment(Comment {
                r#type: CommentType::Line,
                content: comment[skip..].to_string(),
            });
            Some(spanned(start, end, comment))
        } else {
            None
        }
    }

    fn eof_error<T>(&mut self) -> Result<T, SpannedError> {
        let location = self.next_location();
        self.error(location, UnexpectedEof)
    }

    fn error<T>(&mut self, location: Location, code: LexicalError) -> Result<T, SpannedError> {
        self.skip_to_end();
        Err(spanned(location, location, code))
    }

    fn escape_code(&mut self) -> Result<char, SpannedError> {
        match self.bump() {
            Some((_, '\'')) => Ok('\''),
            Some((_, '\\')) => Ok('\\'),
            Some((_, '/')) => Ok('/'),
            Some((_, 'n')) => Ok('\n'),
            Some((_, 'r')) => Ok('\r'),
            Some((_, 't')) => Ok('\t'),
            // TODO: Unicode escape codes
            Some((start, ch)) => self.error(start, UnexpectedEscapeCode(ch)),
            None => self.eof_error(),
        }
    }

    fn slice(&self, start: Location, end: Location) -> &'input str {
        let start = start.absolute - ByteOffset(self.start_index.to_usize() as i64);
        let end = end.absolute - ByteOffset(self.start_index.to_usize() as i64);
        &self.input[start.to_usize()..end.to_usize()]
    }

    fn take_while<F: FnMut(char) -> bool>(&mut self, start: Location, mut keep_going: F) -> (Location, &'input str)
    {
        self.take_until(start, |c| !keep_going(c))
    }

    fn take_until<F: FnMut(char) -> bool>(&mut self, start: Location, mut terminate: F) -> (Location, &'input str)
    {
        while let Some((end, ch)) = self.lookahead {
            if terminate(ch) {
                return (end, self.slice(start, end));
            } else {
                self.bump();
            }
        }
        (self.next_location(), self.slice(start, self.next_location()))
    }

    fn string_literal(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let mut string = String::new();
        while let Some((_, ch)) = self.bump() {
            match ch {
                '\'' => {
                    if self.test_lookahead(|c| c == '\'') {
                        self.bump(); // skip '\''
                        string.push(ch);
                    } else {
                        let end = self.next_location();
                        let token = Token::StringLiteral(string);
                        return Ok(spanned(start, end, token));
                    }
                },
                ch => string.push(ch),
            }
        }

        self.error(start, UnterminatedStringLiteral)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<SpannedToken<'input>, SpannedError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.bump(){
            return match ch {
                ch if ch.is_whitespace() => continue,
                '-' if self.test_lookahead(|ch| ch == '-') => match self.line_comment(start) {
                    Some(token) => Some(Ok(token)),
                    None => continue,
                },
                '!' | '^' | '~' if self.test_lookahead(|ch| ch == '=') => {
                    self.bump(); // skip '='
                    Some(Ok(spanned(start, self.next_location(), Token::NotEqual)))
                },
                ':' => {
                    if self.test_lookahead(|ch| ch == ':') {
                        self.bump(); // skip ':'
                        Some(Ok(spanned(start, self.next_location(), Token::DoubleColon)))
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Colon)))
                    }
                },
                '|' => {
                    if self.test_lookahead(|ch| ch == '|') {
                        self.bump(); // skip ':'
                        Some(Ok(spanned(start, self.next_location(), Token::DoublePipe)))
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Pipe)))
                    }
                },
                '<' => {
                    if let Some((_, ch)) = self.lookahead {
                        match ch {
                            '=' => {
                                self.bump(); // skip '='
                                Some(Ok(spanned(start, self.next_location(), Token::LessOrEqual)))
                            },
                            '>' => {
                                self.bump(); // skip '>'
                                Some(Ok(spanned(start, self.next_location(), Token::NotEqual)))
                            }
                            _ => Some(Ok(spanned(start, self.next_location(), Token::Less)))
                        }
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Less)))
                    }
                },
                '>' => {
                    if self.test_lookahead(|ch| ch == '=') {
                        self.bump(); // skip '='
                        Some(Ok(spanned(start, self.next_location(), Token::GreaterOrEqual)))
                    } else { Some(Ok(spanned(start, self.next_location(), Token::Greater))) }
                },
                ch if is_digit(ch) || ((ch == '-' || ch == '.') && self.test_lookahead(is_digit)) => {
                    Some(self.numeric_literal(start))
                },
                '\'' => Some(self.string_literal(start)),
                '(' => Some(Ok(spanned(start, self.next_location(), Token::LeftParen))),
                ')' => Some(Ok(spanned(start, self.next_location(), Token::RightParen))),
                ',' => Some(Ok(spanned(start, self.next_location(), Token::Comma))),
                '.' => Some(Ok(spanned(start, self.next_location(), Token::Period))),
                '=' => Some(Ok(spanned(start, self.next_location(), Token::Equal))),
                '+' => Some(Ok(spanned(start, self.next_location(), Token::PlusSign))),
                '-' => Some(Ok(spanned(start, self.next_location(), Token::MinusSign))),
                '*' => Some(Ok(spanned(start, self.next_location(), Token::Asterisk))),
                '/' => Some(Ok(spanned(start, self.next_location(), Token::Solidus))),
                ch if is_identifier_start(ch) => Some(self.identifier(start)),
                ch => unimplemented!("{:?}", ch)
            }
        }
        // Return EOF instead of None so that the layout algorithm receives the eof location
//        Some(Ok(spanned(
//            self.next_location(),
//            self.next_location(),
//            Token::EOF,
//        )))
        None
    }
}

pub struct Tokenizer<'input>(Lexer<'input>);

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<(Position, Token<'input>, Position), SpannedError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(t) => Some(match t {
                Ok(t) => Ok((t.span.start().absolute, t.value, t.span.end().absolute)),
                Err(t) => Err(t)
            }),
            None => None,
        }
    }
}


/// Converts partial hex literal (i.e. part after `0x` or `-0x`) to 64 bit signed integer.
///
/// This is basically a copy and adaptation of `std::num::from_str_radix`.
fn i64_from_hex(hex: &str, is_positive: bool) -> Result<i64, LexicalError> {
    const RADIX: u32 = 16;
    let digits = hex.as_bytes();
    let sign: i64 = if is_positive { 1 } else { -1 };
    let mut result = 0i64;
    for &c in digits {
        let x = (c as char).to_digit(RADIX).expect("valid hex literal");
        result = result
            .checked_mul(RADIX as i64)
            .and_then(|result| result.checked_add((x as i64) * sign))
            .ok_or_else(|| {
                if is_positive {
                    HexLiteralOverflow
                } else {
                    HexLiteralUnderflow
                }
            })?;
    }
    Ok(result)
}

#[cfg(test)]
mod test{
    use super::*;
    use super::Token::*;

    fn location(byte: u32) -> Location {
        Location {
            line: Line(0),
            column: Column(byte + 1),
            absolute: Position(byte + 1),
        }
    }

    fn tokenizer<'input>(
        input: &'input str,
    ) -> impl Iterator<Item = Result<SpannedToken<'input>, SpannedError>> + 'input {
        Box::new(Iterator::take_while(Lexer::new(input), |token| match *token {
            Ok(Spanned {
                   value: Token::EOF, ..
               }) => false,
            _ => true,
        }))
    }

    fn test(input: &str, expected: Vec<(&str, Token)>) {
        use super::Source;

        let mut tokenizer = tokenizer(input);
        let mut count = 0;
        let length = expected.len();
        let source = self::codespan::FileMap::new("test".into(), input.to_string());

        for (token, (expected_span, expected_tok)) in tokenizer.by_ref().zip(expected.into_iter()) {
            count += 1;
            let start_byte =
                source.span().start() + ByteOffset(expected_span.find("~").unwrap() as i64);
            let mut start = Source::location(&source, start_byte).unwrap();
            start.column += ColumnOffset(1);

            let end_byte = source.span().start()
                + ByteOffset(expected_span.rfind("~").unwrap() as i64 + 1);
            let mut end = Source::location(&source, end_byte.into()).unwrap();
            end.column += ColumnOffset(1);

            assert_eq!(Ok(spanned(start, end, expected_tok)), token);
        }

        assert_eq!(count, length);
        assert_eq!(true, count > 0);

        // Make sure that there is nothing else to consume
        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn builtin_operators() {
        test(
            r#". : = | ( ) + - * / > < , <= >= != <> ^= ~= ::"#,
            vec![
                (r#"~                                             "#, Period),
                (r#"  ~                                           "#, Colon),
                (r#"    ~                                         "#, Equal),
                (r#"      ~                                       "#, Pipe),
                (r#"        ~                                     "#, LeftParen),
                (r#"          ~                                   "#, RightParen),
                (r#"            ~                                 "#, PlusSign),
                (r#"              ~                               "#, MinusSign),
                (r#"                ~                             "#, Asterisk),
                (r#"                  ~                           "#, Solidus),
                (r#"                    ~                         "#, Greater),
                (r#"                      ~                       "#, Less),
                (r#"                        ~                     "#, Comma),
                (r#"                          ~~                  "#, LessOrEqual),
                (r#"                             ~~               "#, GreaterOrEqual),
                (r#"                                ~~            "#, NotEqual),
                (r#"                                   ~~         "#, NotEqual),
                (r#"                                      ~~      "#, NotEqual),
                (r#"                                         ~~   "#, NotEqual),
                (r#"                                            ~~"#, DoubleColon),
            ],
        );
    }

    #[test]
    fn line_comments() {
        test(
            r#"h-i -- hellooo"#,
            vec![
                (r#"~             "#, Identifier("h")),
                (r#" ~            "#, MinusSign),
                (r#"  ~           "#, Identifier("i")),
                (r#"    ~~ ~~~~~~~"#, DocComment(Comment{r#type: CommentType::Line, content: "hellooo".to_string()}))
            ],
        );
    }

    #[test]
    fn string_literals() {
        test(
            r#"'abc' mn 'hjk''xyz'"#,
            vec![
                (r#"~~~~~              "#, StringLiteral("abc".to_string())),
                (r#"      ~~           "#, Identifier("mn")),
                (r#"         ~~~~~~~~~~"#, StringLiteral(r#"hjk'xyz"#.to_string()), ),
            ],
        );
    }

    #[test]
    fn float_literals() {
        test(
            r#"3.2 4.236 -9.365"#,
            vec![
                (r#"~~~             "#, FloatLiteral(3.2)),
                (r#"    ~~~~~       "#, FloatLiteral(4.236)),
                (r#"          ~~~~~~"#, FloatLiteral(-9.365)),
            ],
        );
    }

    #[test]
    fn keywords() {
        test(
            r#"select * from order left"#,
            vec![
                (r#"~~~~~~                  "#, Select),
                (r#"       ~                "#, Asterisk),
                (r#"         ~~~~           "#, From),
                (r#"              ~~~~~     "#, Order),
                (r#"                    ~~~~"#, Left),
            ],
        );
    }

    #[test]
    fn variable(){
        test(
            r#"a = :  m"#,
            vec![
                (r#"~       "#, Identifier("a")),
                (r#"  ~     "#, Equal),
                (r#"    ~   "#, Colon),
                (r#"       ~"#, Identifier("m")),
            ],
        );
    }

    #[test]
    fn test1() {
        test("a >-3.2",
             vec![
                 ("~      ", Identifier("a")),
                 ("  ~    ", Greater),
                 ("   ~~~~", FloatLiteral(-3.2)),
             ],
        )
    }
    #[test]
    fn test2() {
        test("a > '3.2'",
             vec![
                 ("~        ", Identifier("a")),
                 ("  ~      ", Greater),
                 ("    ~~~~~", StringLiteral("3.2".to_string())),
             ],
        )
    }
    #[test]
    fn test3(){
        test("trim(trailing  'a' from 'abc')",
             vec![
                 ("~~~~                          ", Trim),
                 ("    ~                         ", LeftParen),
                 ("     ~~~~~~~~                 ", Trailing),
                 ("               ~~~            ", StringLiteral("a".to_string())),
                 ("                   ~~~~       ", From),
                 ("                        ~~~~~ ", StringLiteral("abc".to_string())),
                 ("                             ~", RightParen),
             ],
        )
    }
}