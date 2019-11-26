use crate::model::*;
use crate::report::visitor::*;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This types implements `PolicyVisitor`, `StatementVisitor`, and `ConditionVisitor` to
/// produce Markdown formatted documentation for a Policy.
///
#[derive(Debug)]
pub struct MarkdownGenerator {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const IO_ERROR_MSG: &str = "Unexpected write error";

impl Default for MarkdownGenerator {
    fn default() -> Self {
        MarkdownGenerator {}
    }
}

impl PolicyVisitor for MarkdownGenerator {
    fn start(&self, writer: &mut dyn Write) {
        writeln!(writer, "# Policy").expect(IO_ERROR_MSG);
    }

    fn id(&self, writer: &mut dyn Write, i: &String) {
        writeln!(writer, "\n> Policy ID: {}", i).expect(IO_ERROR_MSG);
    }

    fn version(&self, writer: &mut dyn Write, v: &Version) {
        writeln!(
            writer,
            "\n> IAM Policy Version: {}",
            match v {
                Version::V2008 => "2008-10-17",
                Version::V2012 => "2012-10-17",
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn statement<'a>(&'a self) -> Option<Box<&'a dyn StatementVisitor>> {
        Some(Box::new(self))
    }
}

impl StatementVisitor for MarkdownGenerator {
    fn start(&self, writer: &mut dyn Write) {
        writeln!(writer, "\n## Statement").expect(IO_ERROR_MSG);
    }

    fn sid(&self, writer: &mut dyn Write, s: &String) {
        writeln!(writer, "\n> Statement ID: {}", s).expect(IO_ERROR_MSG);
    }

    fn effect(&self, writer: &mut dyn Write, e: &Effect) {
        writeln!(
            writer,
            "\n**{}** IF\n",
            match e {
                Effect::Allow => "ALLOW",
                Effect::Deny => "DENY",
            }
        )
        .expect(IO_ERROR_MSG);
    }

    fn principal(&self, writer: &mut dyn Write, p: &Principal) {
        let (negated, values) = match p {
            Principal::Principal(v) => (false, v),
            Principal::NotPrincipal(v) => (true, v),
        };
        writeln!(
            writer,
            "* `Principal {}`**`IN`**",
            if negated { "`**`NOT`**` " } else { "" }
        )
        .expect(IO_ERROR_MSG);
        for (kind, value) in values {
            writeln!(
                writer,
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

    fn action(&self, writer: &mut dyn Write, a: &Action) {
        let (negated, value) = match a {
            Action::Action(v) => (false, v),
            Action::NotAction(v) => (true, v),
        };
        writeln!(
            writer,
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

    fn resource(&self, writer: &mut dyn Write, r: &Resource) {
        let (negated, value) = match r {
            Resource::Resource(v) => (false, v),
            Resource::NotResource(v) => (true, v),
        };
        writeln!(
            writer,
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

    fn condition<'a>(&'a self) -> Option<Box<&'a dyn ConditionVisitor>> {
        Some(Box::new(self))
    }
}

impl ConditionVisitor for MarkdownGenerator {
    fn start(&self, writer: &mut dyn Write) {
        write!(writer, "* `Condition ").expect(IO_ERROR_MSG);
    }

    fn left(&self, writer: &mut dyn Write, f: &QString, op: &ConditionOperator) {
        write!(
            writer,
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

    fn operator(&self, writer: &mut dyn Write, op: &ConditionOperator) {
        write!(
            writer,
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

    fn right(&self, writer: &mut dyn Write, v: &OneOrAll<ConditionValue>, _op: &ConditionOperator) {
        write!(
            writer,
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

    fn finish(&self, writer: &mut dyn Write) {
        writeln!(writer, "`").expect(IO_ERROR_MSG);
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
