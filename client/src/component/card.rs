use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component]
pub fn Card(props: &Props) -> Html {
    html! {
        <div class="card bg-neutral text-white w-30 h-30">
          <div class="card-body">
            <h2 class="card-title">{&props.title}</h2>
                {for props.children.iter()}
          </div>
        </div>
    }
}
