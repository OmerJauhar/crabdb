var sourcesIndex = JSON.parse('{\
"memchr":["",[["memchr",[["x86",[],["avx.rs","mod.rs","sse2.rs"]]],["fallback.rs","iter.rs","mod.rs","naive.rs"]],["memmem",[["prefilter",[["x86",[],["avx.rs","mod.rs","sse.rs"]]],["fallback.rs","genericsimd.rs","mod.rs"]],["x86",[],["avx.rs","mod.rs","sse.rs"]]],["byte_frequencies.rs","genericsimd.rs","mod.rs","rabinkarp.rs","rarebytes.rs","twoway.rs","util.rs","vector.rs"]]],["cow.rs","lib.rs"]],\
"miette":["",[["eyreish",[],["context.rs","error.rs","fmt.rs","into_diagnostic.rs","kind.rs","macros.rs","mod.rs","ptr.rs","wrapper.rs"]],["handlers",[],["debug.rs","json.rs","mod.rs","narratable.rs"]]],["chain.rs","diagnostic_chain.rs","error.rs","lib.rs","macro_helpers.rs","named_source.rs","protocol.rs","source_impls.rs"]],\
"miette_derive":["",[],["code.rs","diagnostic.rs","diagnostic_arg.rs","diagnostic_source.rs","fmt.rs","forward.rs","help.rs","label.rs","lib.rs","related.rs","severity.rs","source_code.rs","url.rs","utils.rs"]],\
"minimal_lexical":["",[],["bigint.rs","extended_float.rs","lemire.rs","lib.rs","mask.rs","num.rs","number.rs","parse.rs","rounding.rs","slow.rs","stackvec.rs","table.rs","table_lemire.rs","table_small.rs"]],\
"nom":["",[["bits",[],["complete.rs","mod.rs","streaming.rs"]],["branch",[],["mod.rs"]],["bytes",[],["complete.rs","mod.rs","streaming.rs"]],["character",[],["complete.rs","mod.rs","streaming.rs"]],["combinator",[],["mod.rs"]],["multi",[],["mod.rs"]],["number",[],["complete.rs","mod.rs","streaming.rs"]],["sequence",[],["mod.rs"]]],["error.rs","internal.rs","lib.rs","macros.rs","str.rs","traits.rs"]],\
"once_cell":["",[],["imp_std.rs","lib.rs","race.rs"]],\
"proc_macro2":["",[],["detection.rs","extra.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"syn":["",[["gen",[],["clone.rs","gen_helper.rs"]]],["attr.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","drops.rs","error.rs","export.rs","expr.rs","ext.rs","generics.rs","group.rs","ident.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","meta.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","path.rs","print.rs","punctuated.rs","restriction.rs","sealed.rs","span.rs","spanned.rs","thread.rs","token.rs","ty.rs","verbatim.rs"]],\
"thiserror":["",[],["aserror.rs","display.rs","lib.rs"]],\
"thiserror_impl":["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","valid.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]],\
"unicode_width":["",[],["lib.rs","tables.rs"]]\
}');
createSourceSidebar();
