# crabdb

## Installation
Clone using 
```bash
 git clone https://github.com/OmerJauhar/crabdb.git
```
cd into dir 
```bash 
cd crabdb
```
run using 
```bash 
cargo run root mode sql 
```
Alternatevly for help use 
```bash 
cargo run help 
```
```bash 
************ CRABDB ***************
Use the following format
crabdb [USERNAME] mode [MODE]
USERNAME :
    -root (default)
MODE
    -sql
    -nosql
************ CRABDB ***************
```


## TODO
* Implement Delete and update queries
* Implement Disk Management for “sql mode”
* Make the parser more flexible by accepting nosql/sql interchangeable terms
```bash

Table <==> Collection  
Tuple/row <==> Document   
Column <==> Field   
```

* Remove warning from current code
* Implement error handling with miette
## Working Commands 
``` bash 
create database [database name] 
use [database name]
show databases 
show tables 
create table commands  
insert table commands 
drop table commands 
Select commands 
```
## Resources 
* [sqlparser](https://crates.io/crates/sql-parser)
* [rustyline](https://github.com/kkawakam/rustyline)
## Contributing

Contributions are always welcome!

## Contribution Rules

```bash 
1. Fork the repository and create a new branch for your contribution.
2. Follow any code formatting guidelines provided.
3. Test your code before submitting a pull request.
4. Submit your pull request to the appropriate branch.
5. Provide a clear description of your contribution.
6. Follow licensing requirements specified.
7. Enjoy contributing to the project!

```


## License

[MIT](https://choosealicense.com/licenses/mit/)
