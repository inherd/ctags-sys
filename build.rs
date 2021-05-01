#[cfg(feature = "generate")]
extern crate bindgen;
extern crate cc;
extern crate pkg_config;

fn main() {
    let src = [
        "ctags/main/args.c",
        "ctags/main/colprint.c",
        "ctags/main/dependency.c",
        "ctags/main/entry.c",
        "ctags/main/entry_private.c",
        "ctags/main/error.c",
        "ctags/main/field.c",
        "ctags/main/flags.c",
        "ctags/main/fmt.c",
        "ctags/main/htable.c",
        "ctags/main/keyword.c",
        "ctags/main/kind.c",
        "ctags/main/lregex.c",
        "ctags/main/lxpath.c",
        "ctags/main/main.c",
        "ctags/main/mbcs.c",
        "ctags/main/nestlevel.c",
        "ctags/main/numarray.c",
        "ctags/main/objpool.c",
        "ctags/main/options.c",
        "ctags/main/param.c",
        "ctags/main/parse.c",
        "ctags/main/portable-scandir.c",
        "ctags/main/promise.c",
        "ctags/main/ptag.c",
        "ctags/main/ptrarray.c",
        "ctags/main/rbtree.c",
        "ctags/main/read.c",
        "ctags/main/routines.c",
        "ctags/main/seccomp.c",
        "ctags/main/selectors.c",
        "ctags/main/sort.c",
        "ctags/main/stats.c",
        "ctags/main/strlist.c",
        "ctags/main/trace.c",
        "ctags/main/trashbox.c",
        "ctags/main/tokeninfo.c",
        "ctags/main/unwindi.c",
        "ctags/main/vstring.c",
        "ctags/main/writer.c",
        "ctags/main/writer-etags.c",
        "ctags/main/writer-ctags.c",
        "ctags/main/writer-json.c",
        "ctags/main/writer-xref.c",
        "ctags/main/xtag.c"
    ];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include")
        .flag("-Wno-unused-parameter")
        .define("USE_ZLIB", None);
    build.compile("ctags");
}