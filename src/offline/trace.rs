/*!
Provides a simple struct, `Tracer` that holds a trace of operations performed.
*/
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;
use std::fmt::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The tracer type, use `open`, `close`, and `message` methods to add to the trace and
/// `TraceVisitor` to output the trace at a later time.
///
#[derive(Debug)]
pub struct Tracer {
    lines: Vec<TraceLine>,
    depth: u8,
}

///
/// A visitor pattern, will allow for processing a trace after evaluation.
///
pub trait TraceVisitor<T> {
    /// Called when the depth increases.
    fn open(&mut self, depth: u8);

    /// Called for each message string.
    fn message(&mut self, msg: &String) -> Result<(), T>;

    /// Called when the depth decreases.
    fn close(&mut self, depth: u8);
}

///
/// Implements `TraceVisitor` and writes to a formatter.
///
pub struct TraceFormatter<'a> {
    w: &'a mut dyn Write,
    prefix: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Visit the trace `tracer` with the implementation `visitor`.
///
pub fn visit<T>(tracer: &Tracer, visitor: &mut dyn TraceVisitor<T>) -> Result<(), T> {
    let mut c_depth = 0u8;
    for line in &tracer.lines {
        if line.depth > c_depth {
            c_depth = line.depth;
            visitor.open(c_depth);
        } else if line.depth < c_depth {
            c_depth = line.depth;
            visitor.close(c_depth);
        }
        visitor.message(&line.message)?;
    }
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct TraceLine {
    depth: u8,
    message: String,
}

impl Tracer {
    ///
    /// Add a message string at the current trace depth
    ///
    pub fn message(&mut self, msg: &str) -> &mut Self {
        self.lines.push(TraceLine {
            depth: self.depth,
            message: msg.to_string(),
        });
        self
    }

    ///
    /// Increase the trace depth.
    ///
    pub fn open(&mut self) -> &mut Self {
        self.depth += 1;
        self
    }

    ///
    /// Decrease the trace depth.
    ///
    pub fn close(&mut self) -> &mut Self {
        self.depth -= 1;
        self
    }
}

impl Default for Tracer {
    fn default() -> Self {
        Tracer {
            lines: Default::default(),
            depth: 0,
        }
    }
}

impl Display for Tracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut visitor = TraceFormatter::new(f);
        visit(self, &mut visitor)
    }
}

impl<'a> TraceVisitor<std::fmt::Error> for TraceFormatter<'a> {
    fn open(&mut self, depth: u8) {
        self.prefix = format!("{}+-- ", self.prefix)
    }

    fn message(&mut self, msg: &String) -> Result<(), std::fmt::Error> {
        writeln!(self.w, "{}{}", self.prefix.replace('+', "|"), msg)
    }

    fn close(&mut self, depth: u8) {
        self.prefix.truncate(self.prefix.len() - 4);
    }
}

impl<'a> TraceFormatter<'a> {
    pub fn new(w: &'a mut dyn Write) -> Self {
        TraceFormatter {
            w,
            prefix: String::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_trace_output() {
        let mut tracer = Tracer::default();

        tracer.message("hello world");
        tracer.open();
        tracer.message("hello person");
        tracer.open();
        tracer.message("(echo)");
        tracer.close();
        tracer.message(":-)");
        tracer.close();
        tracer.message(":-)");

        let output = tracer.to_string();

        assert_eq!(
            output,
            r#"hello world
|-- hello person
|-- |-- (echo)
|-- :-)
:-)
"#
        )
    }
}
