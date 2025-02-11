extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    // libyaml has a error config, so we need to maunaly do this
    let libyaml = pkg_config::Config::new();
    let mut libyaml_include = "".to_string();
    if let Ok(lib) = libyaml.probe("yaml-0.1") {
        libyaml_include = format!("{}", lib.include_paths[0].display());
    }

    println!("cargo:rustc-link-lib=xml2");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=jansson");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=ApplicationServices");

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
        // "gnu_regex/regcomp.c",
        // "gnu_regex/regexec.c",
        // "gnu_regex/regex_internal.c",
        // "gnu_regex/regex.c",
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
         // todo： add packcc convert
         // todo: modify kotlin.c include path
        "peg/kotlin.c",
        "peg/varlink.c",
    ];

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // wrapper.c from main.c without exit(0)
    // let s = env!("CARGO_MANIFEST_DIR");
    // let from = PathBuf::from(s).join("wrapper").join("wrapper.c");
    // let ctags_dir = out_dir.join("ctags.c");
    // fs::copy(&from, &ctags_dir).unwrap();

    let ctags_dir = out_dir.join("ctags.c");
    fs::write(ctags_dir.clone(), "").expect("");

    let config_h = out_dir.join("config.h");
    fs::write(
        config_h,
        format!(
            "
#define CASE_INSENSITIVE_FILENAMES 1
#define DEFAULT_FILE_FORMAT 2
#define ETAGS \"etags\"
#define HAVE_ASPRINTF 1
#define HAVE_DECL__NSGETENVIRON 1
#define HAVE_DECL___ENVIRON 0
#define HAVE_DIRENT_H 1
#define HAVE_FCNTL_H 1
#define HAVE_ICONV 1
#define HAVE_JANSSON 1
#define HAVE_LIBXML 1
#define HAVE_LIBYAML 1
#define HAVE_MBLEN 1
#define HAVE_MKSTEMP 1
#define HAVE_OPENDIR 1
#define HAVE_PACKCC 0
#define HAVE_SCANDIR 1
#define HAVE_SETENV 1
#define HAVE_STAT_ST_INO 1
#define HAVE_STDBOOL_H 1
#define HAVE_STRCASECMP 1
#define HAVE_STRERROR 1
#define HAVE_STRNCASECMP 1
#define HAVE_STRSTR 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_TRUNCATE 1
#define HAVE_TYPEOF 1
#define HAVE_UNISTD_H 1
#define PACKAGE \"universal-ctags\"
#define PACKAGE_VERSION \"5.9.0\"
#define VERSION \"5.9.0\"
#define TMPDIR {:?}
                ", out_dir.display()),
    )
        .expect("Can't write config.h to OUT_DIR");

    let peg_source = Path::new("ctags").join("peg");
    let builder = bindgen::Builder::default()
        .header("ctags/main/main_p.h")
        .clang_arg("-std=gnu99")
        .clang_arg(format!("-I{}", peg_source.display()))
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/libxml/")
        ;

    let bindings = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    builder.files(src_path.iter());
    builder
        .flag("-DHAVE_CONFIG_H")
        .flag("-std=gnu99")
        .flag("-DUSE_SYSTEM_STRNLEN")
    ;

    builder
        .include(&out_dir)
        .include(Path::new(&libyaml_include))

        .include(Path::new("ctags").join("peg"))
        .include(Path::new("ctags").join("parsers"))
        .include(Path::new("ctags").join("optlib"))
        .include(Path::new("ctags").join("fnmatch"))
        .include(Path::new("ctags").join("gnu_regex"))
        .include(Path::new("ctags").join("extra-cmds"))

        .include(Path::new("ctags").join("main"))
    ;

    for file in files.iter() {
        builder.file(src_path.join(file));
    }
    builder.file(ctags_dir); // fake file;

    builder.warnings(false);
    builder.compile("ctags");
}