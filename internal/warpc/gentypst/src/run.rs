// Copyright 2026 Leedehai. All rights reserved.
use ecow::EcoVec;
use regex::Regex;

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

fn compile_to_html(world: MathWorld) -> String {
    let Warned { output, warnings } = typst::compile::<HtmlDocument>(&world);
    let html_text = match output {
        Ok(doc) => match html(&doc, &HtmlOptions { pretty: false }) {
            Ok(text) => text,
            Err(_) => html_encoding_error(),
        },
        Err(diags) => compiler_error(&diags, &world),
    };

    let re = Regex::new(r"<math.*>.*<\/math>").unwrap();

    let output_text = if let Some(m) = re.find(&html_text) {
        format!("{}", m.as_str())
    } else {
        no_math_node(&html_text)
    };

    // let warnings = eval::deduplicate_with(warnings, &evaluated.warnings);
    // for warning in warnings.iter() {
    //     self.check_diagnostic(NoteKind::Warning, warning, target);
    // }

    // match output {
    //     Ok(output) => Some(output),
    //     Err(errors) => {
    //         let eval_errors = (evaluated.output.as_ref().err())
    //             .map(|errors| errors.as_slice())
    //             .unwrap_or(&[]);
    //         let errors = eval::deduplicate_with(errors, eval_errors);

    //         for error in errors.iter() {
    //             self.check_diagnostic(NoteKind::Error, error, target);
    //         }

    //         None
    //     }

    output_text
}

#[cold]
fn html_encoding_error() -> String {
    // TODO: print debugging info
    // TODO: a common util for <span ...>...</span>.
    format!("<span class='typst-error'>typst html encoding error</span>")
}

#[cold]
fn compiler_error(diags: &EcoVec<SourceDiagnostic>, world: &dyn DiagnosticWorld) -> String {
    let mut buffer = termcolor::Buffer::no_color();
    // We need to use DiagnosticFormat::Short instead of richer variants, since
    // HTML rendering does not respect whitespaces well.
    match emit(&mut buffer, world, diags, DiagnosticFormat::Short) {
        Ok(_) => String::from_utf8(buffer.into_inner()).unwrap_or_default(),
        Err(e) => {
            format!("<span class='typst-error'>error when emitting compiler diagnotics: {e}</span>")
        }
    }
}

#[cold]
fn no_math_node(html_text: &str) -> String {
    format!("<span class='typst-error'>no math node: {html_text}</span>")
}
