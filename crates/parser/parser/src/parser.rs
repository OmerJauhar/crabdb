use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
pub mod parserfunction{
    fn parserftn() -> () 
    {
        let a = 32 ; 
        let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";
    }

}