use crate::handlers::{posts::*, site_info::SiteInfo, use_state::PostComponent};
use web_sys::window;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let site_info = SiteInfo {
        site_name: "Site Name Not Set".to_string(),
    };

    use_effect_with_deps(
        move |site_info| {
            let document = window().unwrap().document().unwrap();
            document.set_title(&site_info.site_name);

            || ()
        },
        site_info.clone(),
    );

    let sample_post = Post {
        id: 0,
        title: "This is a sample post".to_string(),
        date: format!("{}", chrono::Local::now().format("%Y-%m-%d")),
        body: "You are seeing this because there are no posts yet! Let's get writing! 

        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi sit amet erat sit amet magna ultrices suscipit. Cras eleifend id urna sit amet malesuada. Nulla facilisi. Nulla luctus maximus metus sed aliquet. Fusce at porta ipsum. Nulla eu ex pharetra, pellentesque erat nec, consequat mi. Nunc sit amet velit eu nunc posuere fermentum. Nam eleifend eros ut nisl viverra imperdiet. Ut facilisis erat leo, eu varius eros faucibus quis. Vestibulum id ligula ullamcorper, pharetra massa sit amet, condimentum nisl. Pellentesque imperdiet felis sed velit faucibus, vitae varius turpis tempus.
        
        Duis sed massa eget urna congue pharetra. Etiam metus turpis, finibus in metus eget, varius ultrices enim. Sed mollis quam ac tincidunt lacinia. Duis cursus quis elit ut sagittis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Mauris nec mi massa. Suspendisse ipsum magna, elementum sit amet tincidunt vel, ultrices sed turpis. Suspendisse suscipit dui vitae rutrum pulvinar. Sed fringilla ex magna. Integer a purus blandit, eleifend quam non, vestibulum erat. Nulla ac nibh imperdiet, lobortis turpis id, aliquam eros. Praesent egestas arcu ac purus viverra scelerisque. In tellus est, bibendum at faucibus et, tristique vitae sem.
        
        Nam venenatis, est in varius accumsan, lacus felis imperdiet metus, nec scelerisque ligula metus sed metus. Fusce ornare lacinia dictum. Ut venenatis accumsan facilisis. Proin convallis urna et turpis tristique, vitae elementum urna vestibulum. Nulla rutrum at enim sed porttitor. Quisque mollis enim sed quam porta semper. Suspendisse potenti. Suspendisse sit amet ante a arcu cursus ullamcorper. Integer volutpat, augue porttitor varius bibendum, magna quam imperdiet ipsum, vitae convallis mi turpis in dui. Etiam ac mauris ac velit finibus eleifend. Donec nec odio congue, molestie ex et, convallis tortor. Proin faucibus, purus sed vehicula vehicula, sem arcu fringilla sem, quis hendrerit elit est eget massa. Ut nec tempus purus. Etiam eleifend eget sem tempus maximus. Donec vehicula a mauris id molestie.
        
        Proin velit nisi, pharetra non nulla vel, consequat accumsan nunc. Nam id condimentum risus. Donec eu ultricies sem. Donec sit amet faucibus velit. Vivamus mauris neque, consectetur a molestie sed, euismod sed ipsum. Proin vel arcu dictum justo faucibus dictum. Aliquam orci risus, molestie sit amet justo sit amet, dignissim mattis sapien. Vivamus mi elit, dictum in nibh quis, dictum cursus justo.
        
        Quisque a leo ac odio facilisis auctor. Pellentesque sed nibh sed orci ultrices tempus. Sed nec rutrum lacus, sed tincidunt sapien. Curabitur efficitur felis sit amet posuere hendrerit. Quisque ac blandit nunc, nec placerat nulla. Duis ut sollicitudin libero, a venenatis lorem. Suspendisse posuere odio neque, ut ultrices nulla aliquam quis. Vestibulum at pharetra enim. Vestibulum in interdum lacus, at interdum justo. Nunc nec quam placerat, pretium sapien et, facilisis ipsum. Etiam purus urna, interdum nec maximus nec, ornare vel risus. Quisque at bibendum ipsum. Vivamus id ornare dolor. Nulla tincidunt orci in lectus dignissim, et facilisis nisl eleifend. Cras nec arcu mi. Cras pellentesque, purus et finibus mattis, lacus quam viverra ex, vel congue nisi sapien sit amet magna. ".to_string(),
        archived: false,
        draft: false,
        author_id: 1,
        author: "Sample Author".to_string(),
    };

    let posts = vec![
        sample_post,
        Post {
            id: 2,
            title: "Another example post".to_string(),
            date: "2021-05-01".to_string(),
            body: "Another short example post".to_string(),
            archived: false,
            draft: false,
            author_id: 1,
            author: "Another Sample Author".to_string(),
        },
    ];

    html! {
        <>
            <header>{ &site_info.site_name }</header>
            <main>
                { for posts.iter().map(|post| html! { <PostComponent post={post.clone()} /> }) }
            </main>
        </>
    }
}
