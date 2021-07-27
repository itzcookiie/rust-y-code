use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};


#[proc_macro_derive(MysqlInsert)]
pub fn MysqlInsert(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let id = input.ident.clone();

    let id_string = input.ident.to_string();

    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let ident_strs = fields
        .iter()
        .map(|x|x.ident.clone().unwrap().to_string())
        .collect::<Vec<_>>();

    let idents = fields
        .iter()
        .map(|x|x.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let stmnt = ident_strs.iter().fold(String::new(),|a,b|{
        match a.is_empty() {
            true => format!(":{}",b),
            false => {
                format!("{}, :{}",a,b)
            }
        }
    });

    // Build the output, possibly using quasi-quotation
    let ts = quote! {
        #( #ident_strs => &p.#idents ),*
    };

    let expanded = quote! {
        impl #id {
            pub fn mysql_insert(data:Vec<#id>,url:&str){
                use crate::mysql::prelude::Queryable;
                use crate::mysql::params;
                let query = format!("INSERT INTO {} VALUES ({})", #id_string, #stmnt);
                println!("{}",query);
                let mysql_pool = crate::mysql::Pool::new(
                    crate::mysql::Opts::from_url(url).unwrap()
                ).unwrap();
                let mut conn = mysql_pool.get_conn().unwrap();
                let mut tx = conn.start_transaction(crate::mysql::TxOpts::default()).unwrap();
                tx.exec_batch(
                    query,
                    data.iter().map(|p| crate::mysql::params! {
                        #ts
                    })
                );
                tx.commit().unwrap();
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}