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

pub fn compile(world: MathWorld, output_format: OutputFormat) -> String {
    match output_format {
        OutputFormat::Html => compile_to_html(world),
        OutputFormat::Svg => todo!("not implemented: OutputFormat::Svg"),
    }
}

static MATH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<math.*>.*<\/math>").unwrap());

fn compile_to_html(world: MathWorld) -> String {
    let Warned { output, .. } = typst::compile::<HtmlDocument>(&world);

    let html = match output {
        Ok(doc) => match html(&doc, &HtmlOptions { pretty: false }) {
            Ok(full_html) => {
                if let Some(m) = MATH_REGEX.find(&full_html) {
                    format!("{}", m.as_str())
                } else {
                    error_no_math_node()
                }
            }
            Err(_) => error_html_encoding(),
        },
        Err(errors) => compile_diags(&errors, &world),
    };

    html
}

macro_rules! typst_error {
    ($fmt:literal $(, $($arg:tt)*)?) => {{
        let inner = format!($fmt $(, $($arg)*)?);
        format!("<span class='typst-error'>typst: {inner}</span>")
    }};
}

#[cold]
fn error_html_encoding() -> String {
    typst_error!("html encoding error<")
}

#[cold]
fn compile_diags(diags: &EcoVec<SourceDiagnostic>, world: &dyn DiagnosticWorld) -> String {
    let mut buffer = termcolor::Buffer::no_color();
    // We need to use DiagnosticFormat::Short instead of richer variants, since
    // HTML rendering does not respect whitespaces well.
    match emit(&mut buffer, world, diags, DiagnosticFormat::Short) {
        Ok(_) => String::from_utf8(buffer.into_inner()).unwrap_or_default(),
        Err(e) => {
            typst_error!("error when emitting compiler diagnostics: {e}")
        }
    }
}

#[cold]
fn error_no_math_node() -> String {
    typst_error!("no math node in generated HTML")
}
