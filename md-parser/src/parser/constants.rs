pub struct LexConst;

impl LexConst {
    pub const H1: &'static str = "#";
    pub const H2: &'static str = "##";
    pub const H3: &'static str = "###";
    pub const H4: &'static str = "####";
    pub const H5: &'static str = "#####";
    pub const H6: &'static str = "######";
    pub const ORDERED_LIST: &'static str = r"^\d\.\s";
    pub const UNORDERED_LIST: &'static str = r"^\*\s";
    pub const QUOTE: &'static str = r"^>\s";
    pub const UNDERLINE: &'static str = r"_";
    pub const STRONG: &'static str = r"\*\*";
    pub const ITALIC: &'static str = r"\*";
    pub const STRONG_ITALIC: &'static str = r"\*\*\*";
    pub const INLINE_CODE: &'static str = r"`";
    pub const CODE: &'static str = r"^```";
}