use yew::{function_component, Html, Properties, html};

use crate::types::book::Dimensions;

#[derive(PartialEq, Properties, Clone)]
pub struct DimensionsIconProps {
    pub dimensions: Dimensions
}

#[function_component(DimensionsIcon)]
pub fn dimensions_icon(props: &DimensionsIconProps) -> Html {
  let mut width_text: String = props.dimensions.width.to_string();
    width_text.push_str(" ");
    width_text.push_str(&props.dimensions.unit.to_string());

  let mut height_text: String = props.dimensions.height.to_string();
  height_text.push_str(" ");
  height_text.push_str(&props.dimensions.unit.to_string());

  html! {
    <svg viewBox="0 0 250 250">
      <line x1="20%" y1="80%" x2="96%" y2="80%" style="stroke:black;stroke-width:2" />
      <line x1="96%" y1="80%" x2="92%" y2="76%" style="stroke:black;stroke-width:2" />
      <line x1="96%" y1="80%" x2="92%" y2="84%" style="stroke:black;stroke-width:2" />
      
      <line x1="20%" y1="4%" x2="20%" y2="80%" style="stroke:black;stroke-width:2" />
      <line x1="20%" y1="4%" x2="24%" y2="8%" style="stroke:black;stroke-width:2" />
      <line x1="20%" y1="4%" x2="16%" y2="8%" style="stroke:black;stroke-width:2" />
      
      <text text-anchor="middle" x="58%" y="95%" font-size="2em" fill="black">{width_text}</text>
      <text text-anchor="middle" x="5%" y="50%" font-size="2em" fill="black" style="writing-mode: tb;">{height_text}</text>
    </svg>
  }
}