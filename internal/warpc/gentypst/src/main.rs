// Copyright 2026 Leedehai. All rights reserved.

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::io::{self, Write};

pub mod run;
pub mod world;

use run::{OutputFormat, compile};
use world::MathWorld;

// Following the shape of Header in warpc.go.
#[derive(Debug, Deserialize)]
struct RequestHeader {
    #[serde(default = "default_version")]
    version: u16,
    id: u32,
    #[serde(default)]
    command: String,
}

// Following Message[T] in warpc.go, where T is TypstInput.
#[derive(Debug, Deserialize)]
struct Request {
    header: RequestHeader,
    #[serde(default)]
    data: TypstInput,
}

// Following the shape of Header in warpc.go.
#[derive(Debug, Serialize)]
struct ResponseHeader {
    version: u16,
    id: u32,
    command: String,
}

// Following Message[T] in warpc.go, where T is TypstOutput.
#[derive(Debug, Serialize)]
struct Response {
    header: ResponseHeader,
    data: TypstOutput,
}

// From typst.go.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TypstInput {
    expression: String,
    #[serde(default)]
    options: TypstOptions,
}

// From typst.go.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TypstOptions {
    output: String,
    block: bool,
}

// From typst.go.
#[derive(Debug, Serialize)]
struct TypstOutput {
    output: String,
}

fn default_version() -> u16 {
    1
}

fn compile_to_mathml(request: Request) -> Response {
    let text = if request.data.options.block {
        format!("$ {} $", request.data.expression)
    } else {
        format!("${}$", request.data.expression.trim())
    };

    let world = MathWorld::new(text);
    let output = compile(world, OutputFormat::Html);
    // let output = "<math display=\"block\"><mrow><mi>E</mi></mrow></math>".to_string();
    Response {
        header: ResponseHeader {
            version: request.header.version,
            id: request.header.id,
            command: request.header.command,
        },
        data: TypstOutput { output },
    }
}

// pub fn typst_to_mathml(math_expr: &str) -> Result<String, String> {
//     // 1. Wrap the expression in Typst math mode delimiters
//     let source_text = format!("$ {} $", math_expr);

//     // 2. Initialize the mocked World
//     let world = MathWorld::new(&source_text);

//     // 3. Compile the source into a Typst Document
//     let mut tracer = typst::eval::Tracer::new();
//     let document = typst::compile(&world, &mut tracer)
//         .map_err(|e| format!("Typst compilation failed: {:?}", e))?;

//     // 4. Export the compiled document to HTML.
//     // Typst's HTML engine natively converts equation nodes into structured MathML.
//     let html_output = typst_html::html(&document);

//     // 5. Extract the `<math>` element from the resulting HTML string
//     let start = html_output
//         .find("<math")
//         .ok_or("No MathML block generated.")?;
//     let end = html_output
//         .find("</math>")
//         .ok_or("Malformed HTML output.")?
//         + 7;

//     Ok(html_output[start..end].to_string())
// }

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let reader = stdin.lock();
    let mut writer = io::BufWriter::new(stdout.lock());
    let stream = Deserializer::from_reader(reader).into_iter::<Request>();

    for request in stream {
        match request {
            Ok(request) => {
                let response = compile_to_mathml(request);
                if serde_json::to_writer(&mut writer, &response).is_ok() {
                    let _ = writer.write_all(b"\n");
                    let _ = writer.flush();
                }
            }
            Err(_) => break,
        }
    }
}
