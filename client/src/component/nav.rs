use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq, Properties)]
struct NavItemProps {
    route: Route,
}

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <>
            <div class="bg-gradient-to-r from-blue-500 to-cyan-500">
                <nav class="p-3 text-4xl">
                    <NavItem route={Route::Home} />
                    <NavItem route={Route::NotFound} />
                </nav>
            </div>
            <hr class="border-2 border-blue-500" />
        </>
    }
}

#[function_component]
fn NavItem(props: &NavItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let goto = |route: Route| {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    let style_button = concat!(
        "transition ease-in-out hover:skew-y-2 transform-gpu delay-75 hover:translate-x-3 hover:translate-y-1 hover:scale-110 duration-200",
        " p-2 rounded-lg font-bold border-2 border-orange-200",
        // " bg-gradient-to-r from-sky-blue-400 to-sky-blue-500",
        " bg-slate-200 hover:bg-sky-500"
    );

    html! {
        <>
            <a onclick={goto(props.route)} class="px-3">
                <button class={style_button}>{ props.route.to_string() }</button>
            </a>
        </>
    }
}
