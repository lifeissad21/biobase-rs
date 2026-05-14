#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    Integer,
    Real,
}

impl TemplateType {
    pub fn method_suffix(self) -> &'static str {
        match self {
            TemplateType::Integer => "Integer",
            TemplateType::Real => "Real",
        }
    }
}
