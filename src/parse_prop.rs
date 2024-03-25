

#[derive(Debug)]
pub enum PropExpr {
    And(Box<PropExpr>, Box<PropExpr>),
    Or(Box<PropExpr>, Box<PropExpr>),
    Not(Box<PropExpr>),
    Lit(Box<str>),
}

// pub fn parse(contents: &String) -> Box<PropExpr> {
    
// }