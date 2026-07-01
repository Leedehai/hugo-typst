// Copyright 2026 Leedehai. All rights reserved.
use ecow::EcoVec;
use regex::Regex;
use std::sync::LazyLock;

use typst::diag::{SourceDiagnostic, Warned};
use typst_html::{HtmlDocument, HtmlOptions, html};
use typst_kit::diagnostics::{DiagnosticFormat, DiagnosticWorld, emit, termcolor};

use crate::world::MathWorld;

pub enum OutputFormat {
    Html,
    Svg,
}

pub struct SuccessCompileResult {
    pub html: String,
    pub warnings: Vec<String>,
}

type CompileResult = Result<SuccessCompileResult, String>;

pub fn compile(world: MathWorld, output_format: OutputFormat) -> CompileResult {
    match output_format {
        OutputFormat::Html => compile_to_html(world),
        OutputFormat::Svg => todo!("not implemented: OutputFormat::Svg"),
    }
}

static MATH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<math.*>.*<\/math>").unwrap());

macro_rules! typst_diag {
    ($fmt:literal $(, $($arg:tt)*)?) => {{
        let inner = format!($fmt $(, $($arg)*)?);
        format!("typst: {inner}")
    }};
}

fn compile_to_html(world: MathWorld) -> CompileResult {
    let Warned { output, warnings } = typst::compile::<HtmlDocument>(&world);

    match output {
        Ok(doc) => match html(&doc, &HtmlOptions { pretty: false }) {
            Ok(full_html) => {
                if let Some(m) = MATH_REGEX.find(&full_html) {
                    Ok(SuccessCompileResult {
                        html: format!("{}", m.as_str()),
                        warnings: warnings
                            .iter()
                            .map(|diag| typst_diag!("{}", diag.message.to_string()))
                            .collect(),
                    })
                } else {
                    Err(error_no_math_node())
                }
            }
            Err(_) => Err(error_html_encoding()),
        },
        Err(errors) => Err(compile_diags(&errors, &world)),
    }
}

#[cold]
fn error_html_encoding() -> String {
    typst_diag!("html encoding error")
}

#[cold]
fn compile_diags(diags: &EcoVec<SourceDiagnostic>, world: &dyn DiagnosticWorld) -> String {
    let mut buffer = termcolor::Buffer::no_color();
    // We need to use DiagnosticFormat::Short instead of richer variants, since
    // HTML rendering does not respect whitespaces well.
    match emit(&mut buffer, world, diags, DiagnosticFormat::Short) {
        Ok(_) => {
            let e = String::from_utf8(buffer.into_inner()).unwrap_or_default();
            typst_diag!("{e}")
        }
        Err(e) => {
            typst_diag!("error when emitting compiler diagnostics: {e}")
        }
    }
}

#[cold]
fn error_no_math_node() -> String {
    typst_diag!("no math node in generated HTML")
}
