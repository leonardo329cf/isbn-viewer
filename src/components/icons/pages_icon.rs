use yew::{function_component, Html, Properties, html};

#[derive(PartialEq, Properties, Clone)]
pub struct PagesIconProps {
    pub pages: u32,
}

#[function_component(PagesIcon)]
pub fn pages_icon(props: &PagesIconProps) -> Html {
  html! {
    <svg width="190" height="240">
        <rect x="30" y="20" width="150" height="200" style="fill:white;stroke-width:2;stroke:black" />
        <rect x="20" y="15" width="150" height="200" style="fill:white;stroke-width:2;stroke:black" />
        <polygon points="10 10,10 210,160 210,160 50,120 50,120 10" style="fill:white;stroke-width:2;stroke:black" />
        <polygon points="160 50,120 50,120 10,160 50" style="fill:white;stroke-width:2;stroke:black" />
        <text text-anchor="middle" x="85" y="120" fill="black" font-size="2em">{props.pages.to_string()}</text>
    </svg>
  }
}