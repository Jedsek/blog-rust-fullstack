use crate::route::Route;
use gloo::console::log;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct NavBarProps {
    pub title: AttrValue,
    pub routes: Vec<Route>,
}

#[function_component]
pub fn NavBar(props: &NavBarProps) -> Html {
    html! {
        <div class="navbar bg-base-100 flex">
            <div class="navbar-start">
                <NavMenu routes={props.routes.clone()} />
            </div>

            <div class="navbar-center">
                <span class="text-xl font-bold">{props.title.clone()}</span>
            </div>

            <div class="navbar-end">
                // <button class="btn btn-ghost btn-circle">
                //   <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                // </button>
                // <button class="btn btn-ghost btn-circle">
                //   <div class="indicator">
                //     <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" /></svg>
                //     <span class="badge badge-xs badge-primary indicator-item"></span>
                //   </div>
                // </button>
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
struct NavMenuProps {
    routes: Vec<Route>,
}

#[function_component]
fn NavMenu(props: &NavMenuProps) -> Html {
    let navigator = use_navigator().unwrap();
    let toggle = use_bool_toggle(false);
    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| {
            log!("1");
            toggle.toggle();
        })
    };

    let goto = |route: Route| {
        let navigator = navigator.clone();
        let toggle = toggle.clone();
        Callback::from(move |_| {
            toggle.set(true);
            navigator.push(&route);
        })
    };

    let display_route = {
        move |route: Route| {
            html! {<li onclick={goto(route)} class="text-lg font-bold"><a>{format!("{route}")}</a></li>}
        }
    };

    html! {
        <details class="dropdown">
          <summary tabindex="0" class="btn m-1 btn-ghost" {onclick}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg>
                {*toggle}
          </summary>
          <ul tabindex="0" class="z-50 dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
            {props.routes.iter().map(|&route| display_route(route)).collect::<Html>()}
          </ul>
        </details>
    }
}
