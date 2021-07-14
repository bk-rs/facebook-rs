/*
cargo run -p facebook-permission-html_parser
*/

use std::error;

use convert_case::{Case, Casing as _};
use scraper::{Html, Selector};
use selectors::Element as _;

fn main() -> Result<(), Box<dyn error::Error>> {
    let html = include_str!("../tests/developers_docs_permissions_reference.html");

    let document = Html::parse_document(html);

    let login_permissions_selector = Selector::parse("#login_permissions").unwrap();
    let login_permissions_div = document
        .select(&login_permissions_selector)
        .into_iter()
        .next()
        .unwrap()
        .parent_element()
        .unwrap();

    let permission_index_selector = Selector::parse("h3").unwrap();
    let permission_index_iter = login_permissions_div
        .select(&permission_index_selector)
        .into_iter();

    let permission_table_selector = Selector::parse("div:nth-child(n+2) table").unwrap();
    let permission_table_vec: Vec<_> = login_permissions_div
        .select(&permission_table_selector)
        .into_iter()
        .collect();

    let permission_table_th_selector = Selector::parse("th:first-child").unwrap();
    let permission_table_td_selector = Selector::parse("td:first-child").unwrap();
    let permission_table_td_a_selector = Selector::parse("a:first-child").unwrap();

    let mut data = vec![];
    for (i, permission_index) in permission_index_iter.enumerate() {
        let is_deprecated = permission_index.inner_html() == "Deprecated Permissions";

        let permission_table = permission_table_vec[i];

        let th_html = permission_table
            .select(&permission_table_th_selector)
            .next()
            .unwrap()
            .inner_html();

        if th_html.trim() != "Permission" {
            return Err("th html not eq 'Permission'".into());
        }

        for td in permission_table.select(&permission_table_td_selector) {
            let td_a = td.select(&permission_table_td_a_selector).next().unwrap();
            let href = td_a.value().attr("href").unwrap();
            let name = td_a.inner_html();

            data.push((is_deprecated, name, href));
        }
    }

    println!("==============================");
    for (_, name, _) in data.iter().filter(|(is_deprecated, _, _)| !is_deprecated) {
        println!("{},", name.to_case(Case::Pascal));
    }

    println!("==============================");
    for (_, name, _) in data.iter().filter(|(is_deprecated, _, _)| !is_deprecated) {
        println!(r#""{}","#, name);
    }

    Ok(())
}
