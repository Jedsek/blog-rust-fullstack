use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    // html! {
    //     <article class="p-7 hover:bg-slate-200">
    //         <header class="text-3xl font-bold">
    //             <h3>{ &props.title }</h3>
    //         </header>
    //         <footer>
    //             { for props.children.iter() }
    //         </footer>
    //     </article>
    // }
    html! {
        <div class="card bg-neutral text-white w-30 h-30">
          <div class="card-body">
            <h2 class="card-title">{&props.title}</h2>
                {for props.children.iter()}
          </div>
        </div>
    }
}
