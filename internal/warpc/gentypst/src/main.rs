// Copyright 2026 Leedehai. All rights reserved.

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::io::{self, Write};

pub mod run;
pub mod world;

use run::{OutputFormat, SuccessCompileResult, compile};
use world::MathWorld;

// Following the shape of Header in warpc.go.
#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(default = "default_version")]
    version: u16,
    #[serde(default)]
    id: u32,
    #[serde(default)]
    command: String,
    #[serde(default)]
    err: String,
    #[serde(default)]
    warnings: Vec<String>,
}

// Following Message[T] in warpc.go, where T is TypstInput.
#[derive(Debug, Deserialize)]
struct Request {
    header: Header,
    #[serde(default)]
    data: TypstInput,
}

// Following Message[T] in warpc.go, where T is TypstOutput.
#[derive(Debug, Serialize)]
struct Response {
    header: Header,
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
        format!("$ {} $", request.data.expression.trim())
    } else {
        format!("${}$", request.data.expression.trim())
    };

    let world = MathWorld::new(text);
    let base_header = Header {
        version: request.header.version,
        id: request.header.id,
        command: request.header.command,
        err: String::new(),
        warnings: vec![],
    };
    match compile(world, OutputFormat::Html) {
        Ok(SuccessCompileResult { html, warnings }) => Response {
            header: Header {
                warnings,
                ..base_header
            },
            data: TypstOutput { output: html },
        },
        Err(err) => Response {
            header: Header { err, ..base_header },
            data: TypstOutput {
                output: String::new(),
            },
        },
    }
}

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
