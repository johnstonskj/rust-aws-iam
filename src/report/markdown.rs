use crate::model::*;
use crate::report::visitor::*;
use std::io::{stdout, Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This types implements `PolicyVisitor`, `StatementVisitor`, and `ConditionVisitor` to
/// produce [Markdown](https://commonmark.org/) formatted documentation for a Policy.
///
#[allow(missing_debug_implementations)]
pub struct MarkdownGenerator {
    writer: Box<dyn Write>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const IO_ERROR_MSG: &str = "Unexpected write error";

impl MarkdownGenerator {
    ///
    /// Create a new generator that will write formatted content to `writer`. If you wish
    /// to write to `stdout` use `Default::default()`.
    ///
    pub fn new<T>(writer: T) -> Self
    where
        T: Write + Sized + 'static,
    {
        MarkdownGenerator {
            writer: Box::new(writer),
        }
    }

    fn newln(&mut self) {
        writeln!(self.writer.as_mut()).expect(IO_ERROR_MSG);
    }
}

impl Default for MarkdownGenerator {
    fn default() -> Self {
        MarkdownGenerator {
            writer: Box::new(stdout()),
        }
    }
}

impl PolicyVisitor for MarkdownGenerator {
    fn start(&mut self) {
        writeln!(self.writer.as_mut(), "# Policy").expect(IO_ERROR_MSG);
    }

    fn id(&mut self, i: &String) {
        self.newln();
        writeln!(self.writer.as_mut(), "> Policy ID: {}", i).expect(IO_ERROR_MSG);
    }

    fn version(&mut self, v: &Version) {
        self.newln();
        writeln!(
            self.writer.as_mut(),
            "> IAM Policy Version: {}",
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
}

impl StatementVisitor for MarkdownGenerator {
    fn start(&mut self) {
        self.newln();
        writeln!(self.writer.as_mut(), "## Statement").expect(IO_ERROR_MSG);
    }

    fn sid(&mut self, s: &String) {
        self.newln();
        writeln!(self.writer.as_mut(), "> Statement ID: {}", s).expect(IO_ERROR_MSG);
    }

    fn effect(&mut self, e: &Effect) {
        self.newln();
        writeln!(
            self.writer.as_mut(),
            "**{}** IF",
            match e {
                Effect::Allow => "ALLOW",
                Effect::Deny => "DENY",
            }
        )
        .expect(IO_ERROR_MSG);
        self.newln();
    }

    fn principal(&mut self, p: &Principal) {
        let (negated, values) = match p {
            Principal::Principal(v) => (false, v),
            Principal::NotPrincipal(v) => (true, v),
        };
        writeln!(
            self.writer.as_mut(),
            "* `Principal {}`**`IN`**",
            if negated { "`**`NOT`**` " } else { "" }
        )
        .expect(IO_ERROR_MSG);
        for (kind, value) in values {
            writeln!(
                self.writer.as_mut(),
                "   * _`type`_` = {:?} `**`AND`**` `_`id`_` {}`",
                kind,
                match value {
                    OneOrAny::Any => {
                        format!("{}`**`ANY`**`", if negated { "" } else { "`**`IS`**` " })
                    }
                    OneOrAny::One(v) => format!("= \"{}\"", v),
                    OneOrAny::AnyOf(vs) => format!(
                        "`**`IN`**` {:?}",
                        vs.iter().map(|s| s.to_string()).collect::<Vec<String>>()
                    ),
                }
            )
            .expect(IO_ERROR_MSG);
        }
    }

    fn action(&mut self, a: &Action) {
        let (negated, value) = match a {
            Action::Action(v) => (false, v),
            Action::NotAction(v) => (true, v),
        };
        writeln!(
            self.writer.as_mut(),
            "* `Action {}{}`",
            if negated { "`**`NOT`**` " } else { "" },
            match value {
                OneOrAny::Any => format!("{}`**`ANY`**`", if negated { "" } else { "`**`IS`**` " }),
                OneOrAny::One(v) => format!("= \"{}\"", v),
                OneOrAny::AnyOf(vs) => format!(
                    "`**`IN`**` {:?}",
                    vs.iter().map(|s| s.to_string()).collect::<Vec<String>>()
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
            "* `Resource {} {}`",
            if negated { "`**`NOT`**`" } else { "" },
            match value {
                OneOrAny::Any => format!("{}`**`ANY`**`", if negated { "" } else { "`**`IS`**` " }),
                OneOrAny::One(v) => format!("= \"{}\"", v),
                OneOrAny::AnyOf(vs) => format!(
                    "`**`IN`**` {:?}",
                    vs.iter().map(|s| s.to_string()).collect::<Vec<String>>()
                ),
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn condition_visitor(&mut self) -> Option<Box<&mut dyn ConditionVisitor>> {
        Some(Box::new(self))
    }
}

impl ConditionVisitor for MarkdownGenerator {
    fn start(&mut self) {
        write!(self.writer.as_mut(), "* `Condition ").expect(IO_ERROR_MSG);
    }

    fn left(&mut self, f: &QString, op: &ConditionOperator) {
        write!(
            self.writer.as_mut(),
            "{}`_`{}`_`{}",
            if op.if_exists {
                "`**`IF EXISTS`**` "
            } else {
                ""
            },
            f.to_string(),
            if op.if_exists {
                format!(" `**`THEN`**\n   * _`{}`_`", f.to_string())
            } else {
                "".to_string()
            },
        )
        .expect(IO_ERROR_MSG);
    }

    fn operator(&mut self, op: &ConditionOperator) {
        write!(
            self.writer.as_mut(),
            " `**`{:?}`**`{} ",
            op.operator,
            match op.quantifier {
                None => "",
                Some(ConditionOperatorQuantifier::ForAllValues) => " `**`∀`**`",
                Some(ConditionOperatorQuantifier::ForAnyValue) => " `**`∃`**`",
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn right(&mut self, v: &OneOrAll<ConditionValue>, _op: &ConditionOperator) {
        write!(
            self.writer.as_mut(),
            "{}",
            match v {
                OneOrAll::One(v) => {
                    if let ConditionValue::String(s) = v {
                        format!("{:?}", s)
                    } else {
                        condition_value(v)
                    }
                }
                OneOrAll::All(vs) => format!(
                    "{:?}",
                    vs.iter()
                        .map(|v| condition_value(v))
                        .collect::<Vec<String>>()
                ),
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn finish(&mut self) {
        writeln!(self.writer.as_mut(), "`").expect(IO_ERROR_MSG);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn condition_value(v: &ConditionValue) -> String {
    match v {
        ConditionValue::String(v) => v.to_string(),
        ConditionValue::Integer(v) => v.to_string(),
        ConditionValue::Float(v) => v.to_string(),
        ConditionValue::Bool(v) => v.to_string(),
    }
}
