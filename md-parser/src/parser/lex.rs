// lexical analysis

use log::warn;

#[derive(Debug)]
enum Token {
    // Header
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    // list
    OrderedList,
    UnOrderedList,
    // quote
    Quote,
    // inline style
    Underline,
    Strong,
    Italic,
    StrongItalic,
    // code
    InlineCode,
    Code,
    // Text
    Text(String),
}

impl Token {
    /// 获取 header token
    pub fn header(level: usize) -> Option<Token> {
        match level {
            1 => Some(Token::H1),
            2 => Some(Token::H2),
            3 => Some(Token::H3),
            4 => Some(Token::H4),
            5 => Some(Token::H5),
            6 => Some(Token::H6),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    // 当前处理坐标
    pub ind: usize,
    // 字符数组
    pub chars: Vec<char>,
    // token list
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Tokenizer {
            ind: 0,
            chars: text.chars().collect(),
            tokens: vec![],
        }
    }
    pub fn text_to_token_list(&mut self) {
        while self.ind < self.chars.len() {
            match self.chars[self.ind] {
                '#' => self.parse_header(),
                '-' | '*' => self.parse_unordered_list(),
                '>' => self.parse_quote_list(),
                // fallback to parse text
                '\n' => self.parse_new_line(),
                _ => self.parse_text(),
            }
        }
    }

    pub fn token_spy(&mut self) {}

    /// 查询 Ind 坐标下的字符类型
    pub fn text_peek<F>(&self, ind: usize, predicate: F) -> Option<char>
    where
        F: FnOnce(&char) -> bool,
    {
        self.chars.get(ind).copied().filter(predicate)
    }

    /// 判断 ind 是否为换行（包括 \n 或 \r\n）
    fn is_newline(&self, ind: usize) -> bool {
        if let Some('\n') = self.chars.get(ind) {
            return true;
        }

        if ind + 1 < self.chars.len() {
            if let (Some('\r'), Some('\n')) = (self.chars.get(ind), self.chars.get(ind + 1)) {
                return true;
            }
        }

        false
    }

    fn skip_whitespace(&self, mut ind: usize) -> usize {
        while ind < self.chars.len() && self.chars[ind].is_whitespace() {
            ind += 1;
        }
        ind
    }

    // 获取子序列
    fn substring(&self, left: usize, right: usize) -> Option<String> {
        if left < 0 || right < 0 || left > self.chars.len()
            || right > self.chars.len() || left > right {
            warn!("invalid substring param {} {}", left, right);
            return None;
        }
        Some(self.chars[left..right].iter().collect())
    }

    pub fn parse_header(&mut self) {
        // 1. header level
        let mut ind_bak = self.ind;
        let mut header_level = 0;
        while let Some(_) = self.text_peek(ind_bak, |t| *t == '#') {
            header_level += 1;
            ind_bak += 1;
        }
        if let Some(header_token) = Token::header(header_level) {
            self.tokens.push(header_token);
        } else {
            warn!("invalid header token, reset to default ind, callback");
            return;
        }
        // 2. parse text content
        let mut content_ind = self.skip_whitespace(ind_bak);
        ind_bak = content_ind;
        while let Some(_) = self.text_peek(content_ind, |_| !self.is_newline(content_ind)) {
            content_ind += 1;
        }
        self.tokens.push(Token::Text(self.substring(ind_bak, content_ind).unwrap()));
        self.ind = self.skip_whitespace(content_ind);
    }

    pub fn parse_unordered_list(&mut self) {
        // 1. parse symbol
        if let Some(_) = self.text_peek(self.ind, |t| *t == '-' || *t == '*') {
            self.tokens.push(Token::UnOrderedList);
            self.ind += 1;
        } else {
            return;
        }
        // 2. parse content
        let mut content_ind = self.skip_whitespace(self.ind);
        let content_start = content_ind;
        while let Some(_) = self.text_peek(content_ind, |_| !self.is_newline(content_ind)) {
            content_ind += 1;
        }
        self.tokens.push(Token::Text(self.substring(content_start, content_ind).unwrap()));
        self.ind = self.skip_whitespace(content_ind);
    }

    pub fn parse_quote_list(&mut self) {
        // 1. parse symbol
        if let Some(_) = self.text_peek(self.ind, |t| *t == '>') {
            self.tokens.push(Token::Quote);
            self.ind += 1;
        } else {
            return;
        }
        // 2. parse content
        let mut content_ind = self.skip_whitespace(self.ind);
        let content_start = content_ind;
        while let Some(_) = self.text_peek(content_ind, |_| !self.is_newline(content_ind)) {
            content_ind += 1;
        }
        self.tokens.push(Token::Text(self.substring(content_start, content_ind).unwrap()));
        self.ind = self.skip_whitespace(content_ind);
    }

    // 解析普通文本
    pub fn parse_text(&mut self) {
        self.current_char();
        let mut ind_bak = self.skip_whitespace(self.ind);
        let mut content_ind = ind_bak;
        while let Some(_) = self.text_peek(content_ind, |_| !self.is_newline(content_ind)) {
            content_ind += 1;
        }
        self.tokens.push(Token::Text(self.substring(ind_bak, content_ind).unwrap()));
        self.ind = content_ind;
    }

    pub fn parse_new_line(&mut self) {
        let mut ind_bak = self.ind;
        if self.chars[ind_bak] == '\n' {
            self.tokens.push(Token::Text("\n".into()));
            self.ind += 1;
        } else if self.chars[ind_bak] == '\r' && self.chars[ind_bak + 1] == '\n' {
            self.tokens.push(Token::Text("\n".into()));
            self.ind += 2;
        }
    }

    fn current_char(&self) {
        println!("{} {}", self.ind, self.chars[self.ind]);
    }
}

#[cfg(test)]
mod lex_test {
    use crate::parser::lex::Tokenizer;

    const RAW_MARKDOWN_TEXT: &'static str = r#"# 标题 H1
## 标题 H2
### 标题 H3
#### 标题 H4


这是一个**加粗文本**，这是一个*斜体文本*，这是~~删除线文本~~。

##### 标题 H5

无序列表：
- 项目一
- 项目二
- 项目三

有序列表：
1. 第一步
2. 第二步
3. 第三步

> 这是一个引用块，常用于引用他人话语或备注内容。

###### 标题 H6

`inline code` 是行内代码片段。
"#;

    #[test]
    fn test_char_indices() {
        let mut tokenizer = Tokenizer::new(RAW_MARKDOWN_TEXT.into());
        tokenizer.text_to_token_list();
        println!("tokens is {:#?}", tokenizer.tokens);
    }
}