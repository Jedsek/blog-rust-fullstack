use yew::prelude::*;

#[function_component]
pub fn Loading() -> Html {
    html! {
        <div class="flex-row justify-center space-x-10 text-center m-2" >
        <span class="text-4xl font-bold">{"Loading..."}</span>
        <br/>
        <br/>
        <span class="loading loading-spinner loading-lg scale-150 text-primary"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-secondary"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-accent"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-neutral"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-info"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-success"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-warning"></span>
        <span class="loading loading-spinner loading-lg scale-150 text-error"></span>
        </div>
    }
}
