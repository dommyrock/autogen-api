use sqlx::{Row, SqlitePool};

//SQL AST
use sqlparser::ast::{ObjectType, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite://db.db").await?;
    let rows = sqlx::query(
        "SELECT * FROM sqlite_master WHERE type='table' AND tbl_name NOT LIKE 'sqlite_%'",
    )
    .fetch_all(&pool)
    .await?;

    for row in rows {
        println!("---------------------------------------");
        let tbl_name: String = row.get("tbl_name");
        let sql: String = row.get("sql");

        let dialect = GenericDialect {};
        let ast: Vec<Statement> = Parser::parse_sql(&dialect, &sql).unwrap();
        // println!("AST\n{:?}",ast);

        if let Statement::CreateTable { name, columns, .. } = &ast[0] {
            println!("Table Name: {}", name);
            for column in columns {
                println!(
                    "Column Name: {}, Column Type: {}",
                    column.name, column.data_type
                );
            }
        }

        println!("\nTable Name: {},\nSQL: {}", tbl_name, sql);
    }

    Ok(())
}


//TODO use proc_macro to generate api endpoints using something like bellow + Axum

// // In your proc_macro crate
// extern crate proc_macro;
// extern crate syn;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::parse::{Parse, ParseStream, Result};
// use syn::{parse_macro_input, Ident, LitStr, Token};

// struct Table {
//     name: LitStr,
// }

// impl Parse for Table {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let name: LitStr = input.parse()?;
//         Ok(Table { name })
//     }
// }

// #[proc_macro]
// pub fn create_struct(item: TokenStream) -> TokenStream {
//     let Table { name } = parse_macro_input!(item as Table);

//     let struct_name = Ident::new(&name.value(), name.span());
//     let output: TokenStream = quote! {
//         pub struct #struct_name {
//             id: i32,
//             name: String,
//             surname: String,
//             email: String,
//         }
//     }
//     .into();
//     output
// }

// // In your main crate
// create_struct!("Candidate");
