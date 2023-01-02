use yew::prelude::*;

use crate::components::icons::pages_icon::PagesIcon;
use crate::types::book::Book;
use crate::components::icons::dimensions_icon::DimensionsIcon;

#[derive(PartialEq, Properties, Clone)]
pub struct CardProps {
    pub book: Book,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let dimensions = props.book.dimensions.clone();
    let page_count = props.book.page_count.clone();
    html! {
        <div class="main-container">

            <div class="title-container">
                <h1 class="title-text">{&props.book.title}</h1>
            </div>

            <div class="isbn-container">
                <p iclassd="isbn-text">{&props.book.isbn}</p>
            </div>

            <div class="cover-img-container">
                if props.book.cover_url.is_some() {
                    <img src={props.book.cover_url.clone().unwrap()} alt={"cover"}/>
                }
            </div>

            <div class="meta-container">

                if dimensions.is_some() {
                <div class="meta-left-container">
                    <DimensionsIcon 
                        dimensions={dimensions.unwrap()}/>
                </div>
                }

                if page_count.is_some() {
                    <div class="meta-right-container">
                        <PagesIcon pages={page_count.unwrap()} />
                    </div>
                }

            </div>

            if props.book.subtitle.is_some() {
                <div class="subtitle-container">
                    <h3 id="subtitle-text">{&props.book.subtitle.clone().unwrap()}</h3>
                </div>
            }

            if props.book.authors.len() > 0 {
                <div class="authors-container">
                    <h7 class="authors-entry">{props.book.authors.clone().iter().collect::<Html>()}</h7>
                </div>
            }

            if props.book.subjects.len() > 0 {
                <div class="subjects-container">

                    {
                        props.book.subjects.clone().into_iter().map(|subject| {
                            html! {
                                <div class="subjects-entry">
                                    <h7>{subject}</h7>
                                </div>
                            }
                        }).collect::<Html>()
                    }

                </div>
            }
        </div>
    }
}