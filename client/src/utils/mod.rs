pub fn set_title<T>(title: T)
where
    T: AsRef<str>,
{
    gloo::utils::document().set_title(title.as_ref());
}
