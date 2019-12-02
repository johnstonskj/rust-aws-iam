use crate::document::visitor::*;
use crate::model::*;
use std::io::{stdout, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This types implements `PolicyVisitor`, `StatementVisitor`, and `ConditionVisitor` to
/// produce [LaTeX](https://www.latex-project.org/) formatted documentation for a Policy.
///
#[allow(missing_debug_implementations)]
pub struct LatexGenerator {
    writer: Box<dyn Write>,
    stand_alone: bool,
    has_conditions: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const IO_ERROR_MSG: &str = "Unexpected write error";

impl LatexGenerator {
    ///
    /// Create a new generator that will write formatted content to `writer`. If you wish
    /// to write to `stdout` use `Default::default()`. If `stand_alone` is true a document
    /// class, header and operator appendix are included, otherwise a single section is output.
    ///
    pub fn new<T>(writer: T, stand_alone: bool) -> Self
    where
        T: Write + Sized + 'static,
    {
        LatexGenerator {
            writer: Box::new(writer),
            stand_alone,
            has_conditions: false,
        }
    }

    fn newln(&mut self) {
        writeln!(self.writer.as_mut()).expect(IO_ERROR_MSG);
    }
}

impl Default for LatexGenerator {
    fn default() -> Self {
        LatexGenerator {
            writer: Box::new(stdout()),
            stand_alone: true,
            has_conditions: false,
        }
    }
}

impl PolicyVisitor for LatexGenerator {
    fn start(&mut self) {
        if self.stand_alone {
            writeln!(
                self.writer.as_mut(),
                "{}",
                r#"\documentclass[10pt,letterpaper]{article}

\usepackage[T1]{fontenc}
\usepackage{libertine}
\usepackage{amsfonts}

\usepackage{sectsty}
\sectionfont{\LARGE\normalfont\sffamily}
\subsectionfont{\Large\normalfont\sffamily}
\subsubsectionfont{\large\normalfont\sffamily}

\begin{document}
"#
            )
            .expect(IO_ERROR_MSG);
        }
        writeln!(self.writer.as_mut(), "\\section{{Policy}}").expect(IO_ERROR_MSG);
    }

    fn id(&mut self, i: &String) {
        self.newln();
        writeln!(
            self.writer.as_mut(),
            "The policy identifier is \\texttt{{\\small{{{}}}}}. ",
            i
        )
        .expect(IO_ERROR_MSG);
    }

    fn version(&mut self, v: &Version) {
        self.newln();
        writeln!(
            self.writer.as_mut(),
            "The \\textsc{{iam}} policy language version is {}.",
            match v {
                Version::V2008 => "2008-10-17",
                Version::V2012 => "2012-10-17",
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn statement_visitor(&mut self) -> Option<Box<&mut dyn StatementVisitor>> {
        Some(Box::new(self))
    }

    fn finish(&mut self) {
        self.newln();
        if self.stand_alone {
            writeln!(
                self.writer.as_mut(),
                "{}",
                r#"\appendix

\section{Operators}

\begin{tabular}{|c|l|p{3.25in}|}
  \hline
  \textbf{Symbol} & \textbf{Operation} & \textbf{Applies To} \\
  \hline
  $=$ & equality & strings, numbers, dates, \textsc{ip} addresses, \textsc{arn}\small{s}, principal identifiers, actions, resources \\
  \hline
  $\neq$ & not equality & strings, numbers, dates, \textsc{ip} addresses, \textsc{arn}\small{s}, principal identifiers, actions, resources \\
  \hline
  $\equiv$ & case insensitive equality & strings \\
  \hline
  $\not\equiv$ & case insensitive not equality & strings \\
  \hline
  $\approx$ & \textit{likeness} & strings, \textsc{arn}\small{s}, principal identifiers, actions, resources \\
  \hline
  $\not\approx$ & not \textit{likeness} & strings, \textsc{arn}\small{s}, principal identifiers, actions, resources \\
  \hline
  $<$ & less than & numbers, dates \\
  \hline
  $\leq$ & less than or equal to & numbers, dates \\
  \hline
  $>$ & greater than & numbers, dates \\
  \hline
  $\geq$ & greater than or equal to & numbers, dates \\
  \hline
  $?$ & is null & strings, numbers, dates, \textsc{ip} addresses, \textsc{arn}\small{s} \\
  \hline
  $\in$ & set inclusion & strings, numbers, principal identifiers, actions, resources \\
  \hline
  $\notin$ & not set inclusion & strings, numbers, principal identifiers, actions, resources \\
  \hline
\end{tabular}

\hfill

\noindent Also not our use of the symbol $\mathbb{U}$, defined as \textit{the set of all elements being considered},
in the expression $\in \mathbb{U}$ representing the universal wildcard ``*''.

\end{document}
"#
            )
            .expect(IO_ERROR_MSG);
        }
    }
}

impl StatementVisitor for LatexGenerator {
    fn start(&mut self) {
        self.newln();
        writeln!(self.writer.as_mut(), "\\subsection{{Statement}}").expect(IO_ERROR_MSG);
        self.newln();
    }

    fn sid(&mut self, s: &String) {
        write!(
            self.writer.as_mut(),
            "The statement \\textit{{identifier}} is \\texttt{{\\small{{{}}}}}. ",
            s
        )
        .expect(IO_ERROR_MSG);
    }

    fn effect(&mut self, e: &Effect) {
        writeln!(
            self.writer.as_mut(),
            "The effect of this statement is to \\textbf{{{}}} the requesting principal to perform the requested action if all of the following conditions are met:",
            match e {
                Effect::Allow => "allow",
                Effect::Deny => "deny",
            }
        )
        .expect(IO_ERROR_MSG);
        self.newln();
        writeln!(self.writer.as_mut(), "\\begin{{itemize}}").expect(IO_ERROR_MSG);
    }

    fn principal(&mut self, p: &Principal) {
        let (negated, values) = match p {
            Principal::Principal(v) => (false, v),
            Principal::NotPrincipal(v) => (true, v),
        };
        writeln!(
            self.writer.as_mut(),
            "    \\item The request \\textit{{principal}} matches any of: "
        )
        .expect(IO_ERROR_MSG);
        writeln!(self.writer.as_mut(), "    \\begin{{itemize}}").expect(IO_ERROR_MSG);
        for (kind, value) in values {
            writeln!(
                self.writer.as_mut(),
                "        \\item \\textit{{type}} $=$ {:?} $\\wedge$ \\textit{{id}} {}.",
                kind,
                match value {
                    OneOrAny::Any => any(negated),
                    OneOrAny::One(v) => string_or_any(v, negated),
                    OneOrAny::AnyOf(vs) => format!(
                        "{} \\{{{}\\}}",
                        if negated { "$\\notin$" } else { "$\\in$" },
                        vs.iter()
                            .map(|s| string_value(s))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                }
            )
            .expect(IO_ERROR_MSG);
        }
        writeln!(self.writer.as_mut(), "    \\end{{itemize}}").expect(IO_ERROR_MSG);
    }

    fn action(&mut self, a: &Action) {
        let (negated, value) = match a {
            Action::Action(v) => (false, v),
            Action::NotAction(v) => (true, v),
        };
        writeln!(
            self.writer.as_mut(),
            "    \\item The request's \\textit{{action}} {}.",
            match value {
                OneOrAny::Any => any(negated),
                OneOrAny::One(v) => string_or_any(&v.to_string(), negated),
                OneOrAny::AnyOf(vs) => format!(
                    "{} \\{{{}\\}}",
                    if negated { "$\\notin$" } else { "$\\in$" },
                    vs.iter()
                        .map(|s| string_value(&s.to_string()))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn resource(&mut self, r: &Resource) {
        let (negated, value) = match r {
            Resource::Resource(v) => (false, v),
            Resource::NotResource(v) => (true, v),
        };
        writeln!(
            self.writer.as_mut(),
            "    \\item The request's \\textit{{resource}} {}.",
            match value {
                OneOrAny::Any => any(negated),
                OneOrAny::One(v) => string_or_any(v, negated),
                OneOrAny::AnyOf(vs) => format!(
                    "{} \\{{{}\\}}",
                    if negated { "$\\notin$" } else { "$\\in$" },
                    vs.iter()
                        .map(|s| string_value(s))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn condition_visitor(&mut self) -> Option<Box<&mut dyn ConditionVisitor>> {
        self.has_conditions = true;
        writeln!(
            self.writer.as_mut(),
            "    \\item The request matches all of the following conditions:"
        )
        .expect(IO_ERROR_MSG);
        writeln!(self.writer.as_mut(), "    \\begin{{itemize}}").expect(IO_ERROR_MSG);
        Some(Box::new(self))
    }

    fn finish(&mut self) {
        if self.has_conditions {
            self.has_conditions = false;
            writeln!(self.writer.as_mut(), "    \\end{{itemize}}").expect(IO_ERROR_MSG);
        }
        writeln!(self.writer.as_mut(), "\\end{{itemize}}").expect(IO_ERROR_MSG);
    }
}

impl ConditionVisitor for LatexGenerator {
    fn left(&mut self, f: &QString, op: &ConditionOperator) {
        write!(
            self.writer.as_mut(),
            "        \\item {}{}{}",
            if op.if_exists {
                format!(
                    "\\textbf{{if exists}} \\textit{{{}}} \\textbf{{then}} \\\\ ",
                    f
                )
            } else {
                "".to_string()
            },
            match op.quantifier {
                None => "",
                Some(ConditionOperatorQuantifier::ForAllValues) => "$\\forall(v)$",
                Some(ConditionOperatorQuantifier::ForAnyValue) => "$\\exists(v)$",
            },
            format!("\\textit{{{}}}", f)
        )
        .expect(IO_ERROR_MSG);
    }

    fn operator(&mut self, op: &ConditionOperator) {
        write!(self.writer.as_mut(), " {} ", operator_string(op),).expect(IO_ERROR_MSG);
    }

    fn right(&mut self, v: &OneOrAll<ConditionValue>, _op: &ConditionOperator) {
        writeln!(
            self.writer.as_mut(),
            "{}",
            match v {
                OneOrAll::One(v) => {
                    condition_value(v)
                }
                OneOrAll::All(vs) => format!(
                    "\\{{{}\\}}",
                    vs.iter()
                        .map(|v| condition_value(v))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
        .expect(IO_ERROR_MSG);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn string_value(v: &str) -> String {
    format!("``{}''", v.to_string())
        .replace('$', r"\$")
        .replace('{', r"\{")
        .replace('}', r"\}")
}

fn condition_value(v: &ConditionValue) -> String {
    match v {
        ConditionValue::String(v) => string_value(v),
        ConditionValue::Integer(v) => v.to_string(),
        ConditionValue::Float(v) => v.to_string(),
        ConditionValue::Bool(v) => v.to_string(),
    }
}

fn any(negated: bool) -> String {
    format!(
        "${} \\mathbb{{U}}$",
        if negated { "\\notin" } else { "\\in " }
    )
}

fn string_or_any(v: &String, negated: bool) -> String {
    if v == "*" {
        any(negated)
    } else if v.contains('*') {
        format!(
            "{} {}",
            if negated {
                "$\\not\\approx$"
            } else {
                "$\\approx$"
            },
            string_value(v)
        )
    } else {
        format!(
            "{} {}",
            if negated { "$\\neq$" } else { "$=$" },
            string_value(v)
        )
    }
}

#[inline]
fn op_str(op: &str) -> String {
    format!("${}$", op)
}

fn operator_string(op: &ConditionOperator) -> String {
    match &op.operator {
        GlobalConditionOperator::StringEquals => op_str("="),
        GlobalConditionOperator::StringNotEquals => op_str("\\neq"),
        GlobalConditionOperator::StringEqualsIgnoreCase => op_str("\\equiv"),
        GlobalConditionOperator::StringNotEqualsIgnoreCase => op_str("\\not\\equiv"),
        GlobalConditionOperator::StringLike => op_str("\\approx"),
        GlobalConditionOperator::StringNotLike => op_str("\\not\\approx"),

        GlobalConditionOperator::NumericEquals => op_str("="),
        GlobalConditionOperator::NumericNotEquals => op_str("\\neq"),
        GlobalConditionOperator::NumericLessThan => op_str("<"),
        GlobalConditionOperator::NumericLessThanEquals => op_str("\\leq"),
        GlobalConditionOperator::NumericGreaterThan => op_str(">"),
        GlobalConditionOperator::NumericGreaterThanEquals => op_str("\\geq"),

        GlobalConditionOperator::DateEquals => op_str("="),
        GlobalConditionOperator::DateNotEquals => op_str("\\neq"),
        GlobalConditionOperator::DateLessThan => op_str("<"),
        GlobalConditionOperator::DateLessThanEquals => op_str("\\leq"),
        GlobalConditionOperator::DateGreaterThan => op_str(">"),
        GlobalConditionOperator::DateGreaterThanEquals => op_str("\\geq"),

        GlobalConditionOperator::Bool => op_str("="),

        GlobalConditionOperator::BinaryEquals => op_str("="),

        GlobalConditionOperator::IpAddress => op_str("="),
        GlobalConditionOperator::NotIpAddress => op_str("\\neq"),

        GlobalConditionOperator::ArnEquals => op_str("="),
        GlobalConditionOperator::ArnLike => op_str("\\approx"),
        GlobalConditionOperator::ArnNotEquals => op_str("\\neq"),
        GlobalConditionOperator::ArnNotLike => op_str("\\not\\approx"),

        GlobalConditionOperator::Null => op_str("?"),

        GlobalConditionOperator::Other(id) => op_str(&id.to_string()),
    }
}
