

pub struct Stylesheet {
    pub rules: Vec<Rule>
}

impl std::fmt::Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rules.iter().map(|rule| format!("{}\n=------=", rule)).collect::<Vec<String>>().join("\n"))
    }
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // {
        let sels = (&self.selectors).iter().map(|selector| {format!("{}",selector)}).collect::<Vec<String>>().join(", ");

        let decs = (self.declarations).iter().map(|declaration| {
            let value = &declaration.value;

            format!("===={}: {}", &declaration.name, match value {
                Value::Keyword(keyword) => keyword.to_string(),
                Value::Length(len, unit) => format!("{}{}",&len, match unit {
                    Unit::Px => "px".to_string(),
                    Unit::None => "".to_string()
                }).to_string(),
                Value::ColorValue(color) => format!("({},{},{},{})",&color.r, &color.g, &color.b, &color.a)
            })
        }).collect::<Vec<String>>().join("\n");
        // }
        write!(f, "{} {{\n{}\n}}",sels, decs)
    }
}

pub enum Selector {
    Simple(SimpleSelector)
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match &self {
            Selector::Simple(sel) => {
                let mut tag = "";
                let mut id = "";
                let classes = sel.class.join(".");
                if let Some(tag_name) = &sel.tag_name {
                    tag = &tag_name;
                }
                if let Some(sid) = &sel.id {
                    id = &sid;
                }
                write!(f,"{}({})[{}]",tag, id, classes)
            }
        }
    }
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    pub name: String,
    pub value: Value,
}

pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

pub enum Unit {
    Px,
    None,
}
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub a: u8,
    pub g: u8,
}

pub type Specificity = (usize, usize, usize);