use sqlx::{Row, SqlitePool};

//SQL AST
use sqlparser::ast::{DataType, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut file = File::create("./src/models.rs").expect("Unable to create file");

    let pool = SqlitePool::connect("sqlite://db.db").await?;
    let rows = sqlx::query(
        "SELECT * FROM sqlite_master WHERE type='table' AND tbl_name NOT LIKE 'sqlite_%'",
    )
    .fetch_all(&pool)
    .await?;

    //Header
    let header_msg = "//Generated file [Do not change]\n\n";
    let header_imports = "use autogen_macros::generate_controller;\n";
    let header_serde = r"use serde::{Serialize,Deserialize};";
    file.write_all(format!("{header_msg}{header_imports}{header_serde}\n\n").as_bytes())
        .expect("Unable to write header");

    //Struct definitions
    for row in rows {
        let sql: String = row.get("sql");
        let dialect = GenericDialect {};
        let ast: Vec<Statement> = Parser::parse_sql(&dialect, &sql).unwrap();

        match &ast[0] {
            Statement::CreateTable { name, columns, .. } => {
                let mut struct_def = format!("#[allow(non_snake_case)]\n#[derive(Debug,Clone,Serialize,Deserialize)]\npub struct {} {{\n", name);
                for column in columns {
                    let rust_type: &str = match &column.data_type {
                        DataType::Integer(_) => "i32",
                        DataType::BigInt(_) => "i64",
                        DataType::Real => "f64",
                        DataType::Boolean => "i32",
                        DataType::Text => "String",
                        DataType::Date => "chrono::NaiveDate",
                        DataType::Time(_, _) => "chrono::NaiveTime",
                        DataType::Datetime(_) => "chrono::NaiveDateTime",
                        DataType::Blob(_) => "Vec<u8>",

                        _ => "String",
                    };

                    struct_def.push_str(&format!("    pub {}: {},\n", column.name, rust_type));
                }
                struct_def.push_str("}\n\n");

                file.write_all(struct_def.as_bytes())
                    .expect("Unable to write data");
            }
            _ => (),
        }
    }

    Ok(())
}
