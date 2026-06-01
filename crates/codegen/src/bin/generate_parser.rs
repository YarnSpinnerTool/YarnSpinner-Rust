//! Regenerates the ANTLR lexer/parser/visitor/listener for the Yarn grammar and
//! applies the small set of deterministic fix-ups that the `antlr4rust` code
//! generator requires, so that the generated files never have to be hand-edited.
//!
//! Run with:
//!
//! ```bash
//! # Java 11+ on PATH, and the antlr4rust tool jar available (see
//! # crates/compiler/src/parser/generated/readme.md). Point ANTLR4RUST_JAR at it,
//! # or drop it in .antlr/ at the repo root.
//! cargo run -p yarnspinner_codegen --bin generate_parser
//! ```
//!
//! Every fix-up asserts how many times its anchor appears in the freshly
//! generated output. If a future `antlr4rust` release changes the generated code,
//! the relevant assertion fails loudly here instead of silently producing broken
//! or unpatched output — which is the whole point of automating this rather than
//! maintaining a manual checklist.

use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use yarnspinner_codegen::*;

/// Inserted into the generated parser so that `HashtagContextExt` can be
/// constructed with an explicit text token, mirroring a convenience the C# ANTLR
/// runtime provides. It must live in the generated module because it constructs
/// the module-private `ph` field. Inserted immediately before [`NEW_WITH_TEXT_ANCHOR`].
const NEW_WITH_TEXT_IMPL: &str = "\
// Added by the `generate_parser` codegen step (see generated/readme.md): not
// emitted by ANTLR. Mirrors the C# runtime's ability to construct a context with
// specific text, used when synthesising hashtag nodes for line tagging. Lives
// here (not a sibling module) because it constructs the private `ph` field.
impl<'input> HashtagContextExt<'input> {
\tpub fn new_with_text(
\t\tparent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
\t\tinvoking_state: i32,
\t\ttext: impl Into<Option<TokenType<'input>>>,
\t) -> Rc<HashtagContextAll<'input>> {
\t\tRc::new(BaseParserRuleContext::new_parser_ctx(
\t\t\tparent,
\t\t\tinvoking_state,
\t\t\tHashtagContextExt {
\t\t\t\ttext: text.into(),
\t\t\t\tph: PhantomData,
\t\t\t},
\t\t))
\t}
}

";

/// The generated line that immediately follows the `HashtagContextExt` struct;
/// [`NEW_WITH_TEXT_IMPL`] is inserted just before it. Stable, unique anchor.
const NEW_WITH_TEXT_ANCHOR: &str =
    "impl<'input> YarnSpinnerParserContext<'input> for HashtagContext<'input>{}";

/// The two base files the generator emits but the crate never uses.
const UNUSED_FILES: &[&str] = &[
    "yarnspinnerparserbaselistener.rs",
    "yarnspinnerparserbasevisitor.rs",
];

fn main() {
    let third_party = path(ProjectPath::ThirdPersonYarnSpinner);

    // The grammars moved into a `Grammars/` subdirectory in newer YarnSpinner
    // releases; support both layouts so this works against whichever version the
    // submodule is pinned to.
    let grammar_dir = {
        let with_subdir = third_party.join("YarnSpinner.Compiler/Grammars");
        if with_subdir.join("YarnSpinnerLexer.g4").exists() {
            with_subdir
        } else {
            third_party.join("YarnSpinner.Compiler")
        }
    };
    let lexer_g4 = grammar_dir.join("YarnSpinnerLexer.g4");
    let parser_g4 = grammar_dir.join("YarnSpinnerParser.g4");
    assert!(
        lexer_g4.exists() && parser_g4.exists(),
        "grammar files not found under {} — did you run `git submodule update --init`?",
        grammar_dir.display()
    );

    let out_dir = path(ProjectPath::Compiler).join("src/parser/generated");

    let jar = env::var("ANTLR4RUST_JAR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| third_party.join("../../.antlr/antlr4-4.13.3-SNAPSHOT-complete.jar"));
    assert!(
        jar.exists(),
        "antlr4rust tool jar not found at {}.\n\
         Download antlr4-4.13.3-SNAPSHOT-complete.jar from \
         https://github.com/antlr4rust/antlr4/releases (matching the antlr4rust \
         runtime version in crates/compiler/Cargo.toml) and either place it at \
         .antlr/ in the repo root or set the ANTLR4RUST_JAR environment variable.",
        jar.display()
    );

    // 1. Run the ANTLR tool.
    let status = Command::new("java")
        .arg("-jar")
        .arg(&jar)
        .arg("-Dlanguage=Rust")
        .arg(&lexer_g4)
        .arg(&parser_g4)
        .arg("-visitor")
        .arg("-listener")
        .arg("-Xexact-output-dir")
        .arg("-o")
        .arg(&out_dir)
        .status()
        .expect("failed to launch `java` — is a Java 11+ runtime on PATH?");
    assert!(status.success(), "ANTLR generation failed");

    // 2. Apply the deterministic fix-ups to the generated parser.
    let parser_rs = out_dir.join("yarnspinnerparser.rs");
    let mut src = fs::read_to_string(&parser_rs).expect("generated parser not written");

    // antlr4rust emits the context type with a doubled "Parser" segment.
    src = replace_exactly(
        src,
        "YarnSpinnerParserParserContext",
        "YarnSpinnerParserContext",
        3,
        "doubled-`Parser` context type name",
    );

    // The generated `NodeContext::accept` calls `visitor.visit_node(self)`, which
    // resolves to the wrong `visit_node` and recurses forever. Disambiguate it.
    src = replace_exactly(
        src,
        "visitor.visit_node(self)",
        "YarnSpinnerParserVisitor::visit_node(visitor, self)",
        1,
        "visit_node infinite-recursion disambiguation",
    );

    // Add the `new_with_text` constructor inside the generated module.
    src = inject_before(
        src,
        NEW_WITH_TEXT_ANCHOR,
        NEW_WITH_TEXT_IMPL,
        "HashtagContext anchor for new_with_text",
    );

    fs::write(&parser_rs, src).expect("failed to write patched parser");

    // 3. Remove the generated-but-unused base listener/visitor.
    for name in UNUSED_FILES {
        let p = out_dir.join(name);
        if p.exists() {
            fs::remove_file(&p).expect("failed to remove unused generated file");
        }
    }

    println!(
        "Regenerated and patched the Yarn parser at {}",
        out_dir.display()
    );
}

/// Replaces every occurrence of `from` with `to`, asserting that `from` appears
/// exactly `expected` times first. Panics with a descriptive message otherwise so
/// generator drift is caught immediately.
fn replace_exactly(src: String, from: &str, to: &str, expected: usize, what: &str) -> String {
    let found = src.matches(from).count();
    assert_eq!(
        found, expected,
        "codegen fix-up '{what}': expected {expected} occurrence(s) of {from:?}, found {found}. \
         The antlr4rust output likely changed — update generate_parser.rs."
    );
    src.replace(from, to)
}

/// Inserts `insertion` immediately before the single occurrence of `anchor`.
fn inject_before(src: String, anchor: &str, insertion: &str, what: &str) -> String {
    let found = src.matches(anchor).count();
    assert_eq!(
        found, 1,
        "codegen fix-up '{what}': expected exactly 1 occurrence of the anchor, found {found}. \
         The antlr4rust output likely changed — update generate_parser.rs."
    );
    src.replacen(anchor, &format!("{insertion}{anchor}"), 1)
}
