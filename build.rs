#[cfg(feature = "generate")]
extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::env;

fn main() {
    // let mut cfg = pkg_config::Config::new();
    // if let Ok(lib) = cfg.atleast_version("5.9.0").probe("ctags") {
    //     for include in &lib.include_paths {
    //         println!("cargo:root={}", include.display());
    //     }
    //     return;
    // }

    let ref src_path = Path::new("ctags");

    let files = [
        "main/args.c",
        "main/colprint.c",
        "main/dependency.c",
        "main/entry.c",
        "main/entry_private.c",
        "main/error.c",
        "main/field.c",
        "main/flags.c",
        "main/fmt.c",
        "main/htable.c",
        "main/keyword.c",
        "main/kind.c",
        "main/lregex.c",
        "main/lxpath.c",
        "main/main.c",
        "main/mbcs.c",
        "main/nestlevel.c",
        "main/numarray.c",
        "main/objpool.c",
        "main/options.c",
        "main/param.c",
        "main/parse.c",
        "main/portable-scandir.c",
        "main/promise.c",
        "main/ptag.c",
        "main/ptrarray.c",
        "main/rbtree.c",
        "main/read.c",
        "main/routines.c",
        "main/seccomp.c",
        "main/selectors.c",
        "main/sort.c",
        "main/stats.c",
        "main/strlist.c",
        "main/trace.c",
        "main/trashbox.c",
        "main/tokeninfo.c",
        "main/unwindi.c",
        "main/vstring.c",
        "main/writer.c",
        "main/writer-etags.c",
        "main/writer-ctags.c",
        "main/writer-json.c",
        "main/writer-xref.c",
        "main/xtag.c",
        "main/repoinfo.c",
        "main/mio.c",
        "fnmatch/fnmatch.c",
        "gnu_regex/regex.c",
        "parsers/abaqus.c",
        "parsers/abc.c",
        "parsers/ada.c",
        "parsers/ant.c",
        "parsers/asciidoc.c",
        "parsers/asm.c",
        "parsers/asp.c",
        "parsers/autoconf.c",
        "parsers/autoit.c",
        "parsers/automake.c",
        "parsers/awk.c",
        "parsers/basic.c",
        "parsers/beta.c",
        "parsers/bibtex.c",
        "parsers/c.c",
        "parsers/clojure.c",
        "parsers/css.c",
        "parsers/cobol.c",
        "parsers/cpreprocessor.c",
        "parsers/cxx/cxx.c",
        "parsers/cxx/cxx_debug.c",
        "parsers/cxx/cxx_debug_type.c",
        "parsers/cxx/cxx_keyword.c",
        "parsers/cxx/cxx_parser.c",
        "parsers/cxx/cxx_parser_block.c",
        "parsers/cxx/cxx_parser_function.c",
        "parsers/cxx/cxx_parser_lambda.c",
        "parsers/cxx/cxx_parser_namespace.c",
        "parsers/cxx/cxx_parser_template.c",
        "parsers/cxx/cxx_parser_tokenizer.c",
        "parsers/cxx/cxx_parser_typedef.c",
        "parsers/cxx/cxx_parser_using.c",
        "parsers/cxx/cxx_parser_variable.c",
        "parsers/cxx/cxx_subparser.c",
        "parsers/cxx/cxx_qtmoc.c",
        "parsers/cxx/cxx_scope.c",
        "parsers/cxx/cxx_tag.c",
        "parsers/cxx/cxx_token.c",
        "parsers/cxx/cxx_token_chain.c",
        "parsers/diff.c",
        "parsers/dosbatch.c",
        "parsers/dtd.c",
        "parsers/dts.c",
        "parsers/eiffel.c",
        "parsers/erlang.c",
        "parsers/falcon.c",
        "parsers/flex.c",
        "parsers/fortran.c",
        "parsers/fypp.c",
        "parsers/go.c",
        "parsers/haskell.c",
        "parsers/haxe.c",
        "parsers/html.c",
        "parsers/iniconf.c",
        "parsers/itcl.c",
        "parsers/jprop.c",
        "parsers/jscript.c",
        "parsers/json.c",
        "parsers/julia.c",
        "parsers/ldscript.c",
        "parsers/lisp.c",
        "parsers/lua.c",
        "parsers/m4.c",
        "parsers/make.c",
        "parsers/matlab.c",
        "parsers/myrddin.c",
        "parsers/nsis.c",
        "parsers/objc.c",
        "parsers/ocaml.c",
        "parsers/pascal.c",
        "parsers/perl.c",
        "parsers/perl-function-parameters.c",
        "parsers/perl-moose.c",
        "parsers/perl6.c",
        "parsers/php.c",
        "parsers/powershell.c",
        "parsers/protobuf.c",
        "parsers/python.c",
        "parsers/pythonloggingconfig.c",
        "parsers/r-r6class.c",
        "parsers/r-s4class.c",
        "parsers/r.c",
        "parsers/rexx.c",
        "parsers/robot.c",
        "parsers/rpmspec.c",
        "parsers/rst.c",
        "parsers/ruby.c",
        "parsers/rust.c",
        "parsers/scheme.c",
        "parsers/sh.c",
        "parsers/slang.c",
        "parsers/sml.c",
        "parsers/sql.c",
        "parsers/systemdunit.c",
        "parsers/tcl.c",
        "parsers/tcloo.c",
        "parsers/tex.c",
        "parsers/tex-beamer.c",
        "parsers/ttcn.c",
        "parsers/txt2tags.c",
        "parsers/typescript.c",
        "parsers/verilog.c",
        "parsers/vhdl.c",
        "parsers/vim.c",
        "parsers/windres.c",
        "parsers/yacc.c",
        "parsers/yumrepo.c",
        "optlib/RSpec.c",
        "optlib/cmake.c",
        "optlib/ctags-optlib.c",
        "optlib/elixir.c",
        "optlib/elm.c",
        "optlib/gdbinit.c",
        "optlib/inko.c",
        "optlib/kconfig.c",
        "optlib/man.c",
        "optlib/markdown.c",
        "optlib/meson.c",
        "optlib/passwd.c",
        "optlib/pod.c",
        "optlib/qemuhx.c",
        "optlib/puppetManifest.c",
        "optlib/scss.c",
        "optlib/systemtap.c",
        "parsers/maven2.c",
        "parsers/dbusintrospect.c",
        "parsers/glade.c",
        "parsers/svg.c",
        "parsers/plist.c",
        "parsers/relaxng.c",
        "parsers/xml.c",
        "parsers/xslt.c",
        "parsers/yaml.c",
        "parsers/ansibleplaybook.c",
        "main/debug.c"
    ];

    let bindings = bindgen::Builder::default()
        .header("ctags/main/ctags.h")
        .derive_eq(true)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    builder.include("<stdbool.h>");
    builder.files(src_path.iter());

    builder.include(Path::new("ctags").join("main"))
        .include(Path::new("ctags").join("parsers"))
        .include(Path::new("ctags").join("optlib"));

    for file in files.iter() {
        builder.file(src_path.join(file));
    }

    builder.compile("ctags");
}