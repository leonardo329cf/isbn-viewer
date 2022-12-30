use std::{error::Error};
use wasm_bindgen::JsValue;
use yew::prelude::*;

mod components;
mod types;
use components::card::Card;
use types::book::Book;

fn read_book_from_json(str: String) -> Result<Book, Box<dyn Error>> {
    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_str(&str)?;

    // Return the `User`.
    Ok(u)
}

fn console_log(str: String) {
    let object = JsValue::from(str);
    log::error!("Error: {}", object.as_string().unwrap());
}

fn render_card(book: &Book) -> Html {
    html!{
        <Card book = {book.clone()} />
    }
}

#[function_component]
fn App() -> Html {

    let mut book_list: Vec<Book>=Vec::new();
    let input = r#"{
        "isbn": "9788576573821",
        "title": "Messias de Duna",
        "authors": [
            "Frank Herbert"
        ],
        "publisher": "Editora Aleph",
        "dimensions": null,
        "year": 2017,
        "format": "PHYSICAL",
        "subjects": [],
        "retail_price": null,
        "cover_url": "https://covers.openlibrary.org/b/id/10376632-L.jpg",
        "provider": "open-library"
    }"#.to_string();
    let data = read_book_from_json(input);
    match data {
        Ok(book) => book_list.push(book),
        Err(error) => console_log(error.to_string()),
    };

    
    html! {
        <div>
            <div>
                { for book_list.iter().map(|book| render_card(book)) }
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}