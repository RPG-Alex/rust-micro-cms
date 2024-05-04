use crate::models::styling::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StyleProps {
    pub style: Style,
}

#[function_component(StyleForm)]
pub fn style_form(props: &StyleProps) -> Html {
    html! {
        <form>
            <textarea id="style_update_text" value={props.style.css.clone()} />
            <input type="submit" value="update style" />
            <input type="reset" value="reset" />
        </form>
    }
}
