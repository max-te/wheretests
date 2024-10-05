# wheretests (WIP/Proof-of-concept)

This tool uses debug symbols of a Rust libtest (or compatible) binary to create a mapping of test names to test source locations.
The purpose of this is to augment test reports with links to tests, for example for [mutation test reports](https://github.com/max-te/mutant-report).

## How it works

We run the test binary with `--list --format=terse` to get a list of test names.
We also extract all the function names and source locations from the debug symbols of the binary using the `symbolic_debuginfo` crate.
Then we heuristically match the test names to the function names.

## Usage
Run `wheretests <TEST BINARY LOCATION>`. Example of use:

```shell
$ wheretests ./target/debug/deps/typst_syntax-e82ce8371a857995
typst_syntax::set::tests::test_set @ /home/max/dev/typst/crates/typst-syntax/src/set.rs:180
{"name":"typst_syntax::set::tests::test_set","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/set.rs","line":180}
typst_syntax::source::tests::test_source_file_new @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:327
{"name":"typst_syntax::source::tests::test_source_file_new","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":327}
typst_syntax::source::tests::test_source_file_pos_to_line @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:341
{"name":"typst_syntax::source::tests::test_source_file_pos_to_line","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":341}
typst_syntax::source::tests::test_source_file_pos_to_column @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:354
{"name":"typst_syntax::source::tests::test_source_file_pos_to_column","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":354}
typst_syntax::source::tests::test_source_file_utf16 @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:365
{"name":"typst_syntax::source::tests::test_source_file_utf16","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":365}
typst_syntax::source::tests::test_source_file_roundtrip @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:386
{"name":"typst_syntax::source::tests::test_source_file_roundtrip","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":386}
typst_syntax::source::tests::test_source_file_edit @ /home/max/dev/typst/crates/typst-syntax/src/source.rs:403
{"name":"typst_syntax::source::tests::test_source_file_edit","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/source.rs","line":403}
typst_syntax::ast::tests::test_expr_default @ /home/max/dev/typst/crates/typst-syntax/src/ast.rs:2192
{"name":"typst_syntax::ast::tests::test_expr_default","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/ast.rs","line":2192}
typst_syntax::node::tests::test_linked_node @ /home/max/dev/typst/crates/typst-syntax/src/node.rs:1013
{"name":"typst_syntax::node::tests::test_linked_node","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/node.rs","line":1013}
typst_syntax::node::tests::test_linked_node_non_trivia_leaf @ /home/max/dev/typst/crates/typst-syntax/src/node.rs:1033
{"name":"typst_syntax::node::tests::test_linked_node_non_trivia_leaf","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/node.rs","line":1033}
typst_syntax::reparser::tests::test_reparse_markup @ /home/max/dev/typst/crates/typst-syntax/src/reparser.rs:279
{"name":"typst_syntax::reparser::tests::test_reparse_markup","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/reparser.rs","line":279}
typst_syntax::reparser::tests::test_reparse_block @ /home/max/dev/typst/crates/typst-syntax/src/reparser.rs:309
{"name":"typst_syntax::reparser::tests::test_reparse_block","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/reparser.rs","line":309}
typst_syntax::span::tests::test_span_encoding @ /home/max/dev/typst/crates/typst-syntax/src/span.rs:144
{"name":"typst_syntax::span::tests::test_span_encoding","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/span.rs","line":144}
typst_syntax::highlight::tests::test_highlighting @ /home/max/dev/typst/crates/typst-syntax/src/highlight.rs:418
{"name":"typst_syntax::highlight::tests::test_highlighting","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/highlight.rs","line":418}
typst_syntax::package::tests::version_version_match @ /home/max/dev/typst/crates/typst-syntax/src/package.rs:509
{"name":"typst_syntax::package::tests::version_version_match","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/package.rs","line":509}
typst_syntax::package::tests::minimal_manifest @ /home/max/dev/typst/crates/typst-syntax/src/package.rs:526
{"name":"typst_syntax::package::tests::minimal_manifest","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/package.rs","line":526}
typst_syntax::package::tests::tool_section @ /home/max/dev/typst/crates/typst-syntax/src/package.rs:561
{"name":"typst_syntax::package::tests::tool_section","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/package.rs","line":561}
typst_syntax::package::tests::unknown_keys @ /home/max/dev/typst/crates/typst-syntax/src/package.rs:602
{"name":"typst_syntax::package::tests::unknown_keys","compile_dir":"/home/max/dev/typst","file":"crates/typst-syntax/src/package.rs","line":602}

$ ./target/debug/deps/typst_syntax-e82ce8371a857995

running 18 tests
test ast::tests::test_expr_default ... ok
test node::tests::test_linked_node ... ok
test highlight::tests::test_highlighting ... ok
test node::tests::test_linked_node_non_trivia_leaf ... ok
test package::tests::minimal_manifest ... ok
test package::tests::tool_section ... ok
test package::tests::unknown_keys ... ok
test package::tests::version_version_match ... ok
test set::tests::test_set ... ok
test source::tests::test_source_file_new ... ok
test source::tests::test_source_file_pos_to_column ... ok
test source::tests::test_source_file_pos_to_line ... ok
test span::tests::test_span_encoding ... ok
test source::tests::test_source_file_roundtrip ... ok
test source::tests::test_source_file_edit ... ok
test source::tests::test_source_file_utf16 ... ok
test reparser::tests::test_reparse_block ... ok
test reparser::tests::test_reparse_markup ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
