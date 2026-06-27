// Copyright 2026 Leedehai. All rights reserved.

use std::io::Write;
use std::sync::LazyLock;

use typst::diag::FileError;
use typst::foundations::{Bytes, Datetime, Duration, NoneValue, Value, func};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::{LazyHash, singleton};
use typst::{Feature, Library, LibraryExt, World};
use typst_kit::datetime::Time;
use typst_kit::diagnostics::DiagnosticWorld;

const INPUT_FILENAME_PLACEHOLDER: &'static str = "<ToMath input>";

pub struct MathWorld {
    base: &'static MathWorldBase,
    source: Source,
}

impl MathWorld {
    pub fn new(text: String) -> Self {
        Self {
            base: singleton!(MathWorldBase, MathWorldBase::default()),
            source: Source::new(*EXPRESSION_ID, text),
        }
    }
}

impl World for MathWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.base.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.base.book
    }

    fn main(&self) -> FileId {
        *EXPRESSION_ID
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == *EXPRESSION_ID {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().get_without_slash().into()))
        }
    }

    fn file(&self, id: FileId) -> typst::diag::FileResult<Bytes> {
        if id == *EXPRESSION_ID {
            Ok(Bytes::from_string(self.source.clone()))
        } else {
            Err(FileError::NotFound(id.vpath().get_without_slash().into()))
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.base.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        self.base.now.today(offset)
    }
}

impl DiagnosticWorld for MathWorld {
    fn name(&self, _id: FileId) -> String {
        INPUT_FILENAME_PLACEHOLDER.into()
    }
}

pub struct MathWorldBase {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    now: Time,
}

impl Default for MathWorldBase {
    fn default() -> Self {
        // We don't need the embedded fonts to write HTML. But to write SVG,
        // we use: typst_kit::fonts::embedded().map(|f| f.0).collect();
        let fonts: Vec<_> = Vec::new();
        Self {
            library: LazyHash::new(library()),
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts: fonts,
            now: Time::system(),
        }
    }
}

/// Static [`FileId`] allocated for stdin. This is to ensure that stdin can live
/// in the project root without colliding with any real on-disk file.
static EXPRESSION_ID: LazyLock<FileId> = LazyLock::new(|| {
    FileId::unique(RootedPath::new(
        VirtualRoot::Project,
        VirtualPath::new(INPUT_FILENAME_PLACEHOLDER).unwrap(),
    ))
});

fn library() -> Library {
    let features = [Feature::Html].into_iter().collect();
    let mut lib = Library::builder().with_features(features).build();

    lib.global.scope_mut().define_func::<print>();

    lib
}

#[func]
fn print(#[variadic] values: Vec<Value>) -> NoneValue {
    let mut out = std::io::stdout().lock();
    write!(out, "> ").unwrap();
    for (i, value) in values.into_iter().enumerate() {
        if i > 0 {
            write!(out, ", ").unwrap();
        }
        write!(out, "{value:?}").unwrap();
    }
    writeln!(out).unwrap();
    NoneValue
}
