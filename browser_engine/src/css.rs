use std::default::Default;
use std::fmt;

pub struct Color {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
    pub(crate) alpha: f32,
}

pub enum Unit {
    Em,
    Ex,
    Chr,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

pub struct Declarations {
    pub(crate) property: String,
    pub(crate) value: Value,
}

pub struct SimpleSelector {
    pub(crate) tag_name: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) classes: Vec<String>,
}

pub struct Selector {
    pub(crate) simple: Vec<SimpleSelector>,
    pub(crate) combinators: Vec<char>,
}

pub struct Rule {
    pub(crate) selectors: Vec<Selector>,
    pub(crate) declarations: Vec<Declaration>,
}

pub struct StyleSheet {
    pub(crate) rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> StyleSheet {
        StyleSheet { rules }
    }
}

impl Default for StyleSheet {
    fn default() -> Self {
        StyleSheet { rules: Vec::new() }
    }
}

impl fmt::Debug for StyleSheet {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        let mut rules = String::new();

        for rule in &self.rules {
            if rules.len() > 0 {
                rules.push_str("\n\n");
            }
            rules.push_str(&format!("{0:?}", rule));
        }

        write!(format, "{0}", rules)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declarations>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        let mut selectors = String::new();
        let mut declarations = String::new();
        let tab = "     ";

        for selector in &self.selectors {
            if selectors.len() > 0 {
                selectors.push_str(", ");
            }

            selectors.push_str(&format!("{0:?}", selector));
        }

        for declaration in &self.declarations {
            declarations.push_str(tab);
            declarations.push_str(&format!("{0:?}", declaration));
            declarations.push('\n');
        }

        write!(format, "{} {{\n{}}}", selectors, declarations)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        let mut simple_selector = String::new();

        for selector in &self.simple {
            if simple_selector.len() > 0 {
                simple_selector.push_str(", ");
            }
            simple_selector.push_str(&format!("{0:?}", selector));
        }

        write!(format, "{}", simple_selector)
    }
}

impl SimpleSelector {
    pub fn new(
        tag_name: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
    ) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, format: fmt::Formatter) -> fmt::Result {
        let mut selector = String::new();

        match self.tag_name {
            Some(ref tag) => selector.push_str(tag),
            None => {}
        }

        match self.id {
            Some(ref id) => {
                selector.push('#');
                selector.push_str(id)
            }
            None => {}
        }

        for class in &self.classes {
            selector.push('.');
            selector.push_str(class);
        }

        write!(format, "{}", selector)
    }
}

impl Declarations {
    pub fn new(property: String, value: Value) -> Declarations {
        Declarations { property, value }
    }
}

impl Default for Declarations {
    fn default() -> Self {
        Declarations {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}

impl fmt::Debug for Declarations {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref color) => write!(format, "{0:?}", color),
            Value::Length(ref length, _) => write!(format, "{0:?}", length),
            Value::Other(ref string) => write!(format, "{0:?}", string),
        }
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(
            format,
            "r: {0} g: {1} b: {2} a: {3}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
