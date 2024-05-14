use crate::models::styling::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StyleProps {
    pub style: Style,
}

#[function_component(StyleForm)]
pub fn style_form(props: &StyleProps) -> Html {
    html! {
        <div class="posts">
            <form>
                <textarea id="style_update_text" value={props.style.css.clone()} />
                <div class="form-buttons">
                    <input type="submit" value="update style" class="submit-button" />
                    <input type="reset" value="reset" class="reset-button" />
                </div>
            </form>
        </div>
    }
}
