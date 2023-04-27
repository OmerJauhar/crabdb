var sourcesIndex = JSON.parse('{\
"cfg_if":["",[],["lib.rs"]],\
"log":["",[],["lib.rs","macros.rs"]],\
"sqlparser":["",[["ast",[["helpers",[],["mod.rs","stmt_create_table.rs","stmt_data_loading.rs"]]],["data_type.rs","ddl.rs","mod.rs","operator.rs","query.rs","value.rs"]],["dialect",[],["ansi.rs","bigquery.rs","clickhouse.rs","generic.rs","hive.rs","mod.rs","mssql.rs","mysql.rs","postgresql.rs","redshift.rs","snowflake.rs","sqlite.rs"]]],["keywords.rs","lib.rs","parser.rs","test_utils.rs","tokenizer.rs"]]\
}');
createSourceSidebar();
