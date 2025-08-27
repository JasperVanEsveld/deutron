use std::path::PathBuf;

use swc_core::common::{
    comments::SingleThreadedComments, sync::Lrc, Globals, Mark, SourceMap, GLOBALS,
};
use swc_core::ecma::ast::{Pass, Program};
use swc_core::ecma::codegen::{text_writer::JsWriter, Config, Emitter};
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
use swc_core::ecma::transforms::base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_core::ecma::transforms::typescript::strip;
use swc_core::ecma::visit::VisitMutWith;

use anyhow::{bail, Context, Result};

pub fn transpile(path: &PathBuf) -> Result<Vec<u8>> {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.load_file(path).context("File Not Found")?;

    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );
    let mut buf = Vec::new();
    let mut emitter = Emitter {
        cfg: Config::default().with_minify(false),
        cm: cm.clone(),
        comments: Some(&comments),
        wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };

    let mut parser = Parser::new_from(lexer);
    let parse = parser.parse_module();
    let module = {
        if parse.is_err() {
            bail!("Failed to transpile");
        }
        parse.unwrap()
    };
    let mut program = Program::Module(module);

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // Conduct identifier scope analysis
        program.visit_mut_with(&mut resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        strip(unresolved_mark, top_level_mark).process(&mut program);

        // Fix up any identifiers with the same name, but different contexts
        program.visit_mut_with(&mut hygiene());

        // Ensure that we have enough parenthesis.
        program.visit_mut_with(&mut fixer(emitter.comments));
    });
    emitter.emit_program(&program)?;
    Ok(buf)
}
