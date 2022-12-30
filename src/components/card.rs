use yew::prelude::*;

use crate::types::book::Book;

#[derive(PartialEq, Properties, Clone)]
pub struct CardProps {
    pub book: Book,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div>
            <p>{&props.book.isbn}</p>
            <p>{&props.book.title}</p>
            if props.book.subtitle.is_some() {
                <p>{props.book.subtitle.clone().unwrap()}</p>
            }
            if props.book.cover_url.is_some() {
                <img src={props.book.cover_url.clone().unwrap()} alt={"cover"}/>
            }
            if props.book.year.is_some() {
                <p>{props.book.year.clone().unwrap()}</p>
            }
            if props.book.publisher.is_some() {
                <p>{props.book.publisher.clone().unwrap()}</p>
            }
        </div>
    }

}