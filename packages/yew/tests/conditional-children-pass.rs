// #[test]
// fn conditional_children_printing() {
//     let b = false;
//     let h = ::yew::html! {
//         <div>
//             <div>{ "DAVIS" }</div>
//             if b {
//                 <div>{ "LOL" }</div>
//             }
//         </div>
//     };

//     let h = ::yew::html! {
//         <div>
//             if b {
//                 <div>{ "LOL" }</div>
//             }
//         </div>
//     };

//     let z = vec![
//         ::yew::html_nested! {<div>{ "DAVIS" }</div>},
//         ::yew::html_nested! {
//             if b {
//                 <div>{ "LOL" }</div>
//             }
//         },
//     ];

//     let h = ::yew::html! {
//         <div>
//         {
//             for z.into_iter()
//         }
//         </div>
//     };
// }

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Problem(p: &Props) -> Html {
    html! {
        <div class="container">
            {
                for p.children.iter().enumerate().map(|(i, x)| {
                    html! {
                        <div class="OK">
                            {x}
                            {i}
                        </div>
                    }
                })
            }
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let b = false;
    html! {
        <Problem>
            <span>{ "A" }</span>
            <span>{ "B" }</span>
            if b {
                <span>{ "C" }</span>
            }
            <span>{ "D" }</span>
        </Problem>
    }
}

#[tokio::test]
async fn conditional_children_printing() {
    let mut s = String::new();
    yew::ServerRenderer::<App>::new()
        .render_to_string(&mut s)
        .await;

    assert!(false, "S WORKS: {s:?}");
    // yew::Renderer::<App>::new().render();
}
