use std::io::Write;
use std::string::ToString;

pub trait ToSexp {
    fn to_sexp(&self) -> Sexp;
}

#[derive(Debug)]
pub enum Atom {
    Dbl(f64),
    Str(String),
}

#[derive(Debug)]
pub enum Sexp {
    Atom(Atom),
    Annotated(String, Box<Sexp>),
    List(Vec<Sexp>),
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

fn to_string_helper(stream: &mut String, sexp: &Sexp, col: usize) -> usize {
    match sexp {
        Sexp::Atom(Atom::Dbl(val)) => {
            let rep = val.to_string();
            let delta = rep.len();
            stream.push_str(&format!("{rep}"));
            col + delta
        }
        Sexp::Atom(Atom::Str(val)) => {
            let delta = 2 + val.len();
            stream.push_str(&format!("\"{val}\""));
            col + delta
        }
        Sexp::Annotated(content, bocks) => {
            stream.push_str(&format!("#{content}\n"));
            let prefix = " ".repeat(col);
            stream.push_str(&format!("{prefix}"));
            col + to_string_helper(stream, &bocks, col)
        }
        Sexp::List(sexps) => {
            let mut indentation = col;
            stream.push_str("(");
            indentation += 1;

            let size = sexps.len();

            if size > 0 {
                indentation += to_string_helper(stream, &sexps[0], col);
            }

            if size > 1 {
                stream.push_str(" ");
                indentation += 1;
                to_string_helper(stream, &sexps[1], col);
            }

            if size > 2 {
                for sexp in &sexps[2..] {
                    stream.push_str("\n");
                    let prefix = " ".repeat(indentation);
                    stream.push_str(&format!("{prefix}"));
                    to_string_helper(stream, sexp, indentation);
                }
            }
            col
        }
    }
}

impl ToString for Sexp {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        to_string_helper(&mut buffer, &self, 0);
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn it_works() {
        assert_eq!(
            Sexp::Annotated(String::from("comment"), Box::new(Sexp::Atom(Atom::Dbl(2.01)))).to_string(),
            "#comment
2.01"
        );
    }
}
