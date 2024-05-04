use crate::models::styling::Style;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct StyleProps {
    pub style: Style,
}

#[function_component(StyleInjector)]
pub fn style_injector(props: &StyleProps) -> Html {
    html! {
        <style>
            {&props.style.css}
        </style>
    }
}
