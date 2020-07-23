use std::fmt;

pub enum Expr {
    Lit(Literal),
    Un(Unary),
    Bin(Binary),
    Grp(Grouping),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(lit) => write!(f, "{}", lit),
            Self::Un(un) => write!(f, "{}", un),
            Self::Bin(bin) => write!(f, "{}", bin),
            Self::Grp(grp) => write!(f, "{}", grp),
        }
    }
}

pub enum Literal {
    Num(f64),
    Str(String),
    True,
    False,
    Nil,
}

pub struct Grouping(Box<Expr>);

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.0)
    }
}

pub struct Unary {
    op: UnaryOp,
    expr: Box<Expr>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, *self.expr)
    }
}

pub enum UnaryOp {
    Minus,
    Bang,
}

pub struct Binary {
    l_expr: Box<Expr>,
    op: Op,
    r_expr: Box<Expr>,
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, *self.l_expr, *self.r_expr)
    }
}

pub enum Op {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Self::Num(n) => n.to_string(),
            Self::Str(s) => format!("{}", s),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
            Self::Nil => "nil".to_string(),
        };
        write!(f, "{}", display)
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Minus => "-",
                Self::Bang => "!",
            }
        )
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::Less => "<",
                Self::LessEqual => "<=",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Star => "*",
                Self::Slash => "/",
            }
        )
    }
}
