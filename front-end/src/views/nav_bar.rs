use yew::prelude::*;
use crate::models::nav::{Nav, NavItemType, NavItem};

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    pub nav: Nav,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProps) -> Html {
    let NavBarProps { nav } = props;

    let title = nav.items.iter()
        .find(|item| item.item_type == NavItemType::ThumbnailUrl)
        .map(|item| item.content.clone())
        .unwrap_or_else(|| "Place holder text".to_string());

    let description = nav.items.iter()
        .find(|item| item.item_type == NavItemType::BlogSummary)
        .map(|item| item.content.clone())
        .unwrap_or_else(|| "Placeholder of Blog Description".to_string());

    let social_links: Vec<Html> = nav.items.iter()
        .filter(|item| item.item_type == NavItemType::SocialLink)
        .map(|item| {
            html! {
                <li><a href={item.url.clone().unwrap_or_else(|| "#".to_string())}>{ &item.content }</a></li>
            }
        })
        .collect();

    html! {
        <nav>
            <div class="content">
                <div class="img-container">
                     <h1 class="blog-title">
                        <a href="/"> { title }</a>
                     </h1>
                </div>
                <p class="description">
                { description }
                </p>
                <ul class="links">
                    { for social_links }
                </ul>
            </div>
        </nav>
    }
}
