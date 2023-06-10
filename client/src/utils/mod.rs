use yew::{html, AttrValue, Html};

pub fn set_title<T>(title: T)
where
    T: AsRef<str>,
{
    gloo::utils::document().set_title(title.as_ref());
}

pub fn to_html<T>(content: T) -> Html
where
    T: Into<AttrValue>,
{
    let parsed = Html::from_html_unchecked(content.into());
    html! {
        <div>
            {parsed}
        </div>
    }
}
