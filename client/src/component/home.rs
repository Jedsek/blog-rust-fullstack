use crate::component::{Card, ChatBubble};
use crate::utils::set_title;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    set_title("Home");

    html! {
        <>
            <Card title={"首页"}>
                <p>{ "这是首页" }</p>
            </Card>

            <div class="my-7 mx-2">
                <ChatBubble>
                    <span> {"这是一个  yew  +  actix-web  +  sqlx(sqlite)  +  tailwindcss(daisyui)  的博客练手项目, 仅用于演示"} </span>
                    <span> {"思路来源于B站"}
                        <br/>
                        {"B站视频地址: "}
                        <a class="link link-accent" target="_blank" href="https://www.bilibili.com/video/BV1pY4y1Z7jR"> {"Rust: 用 ntex 和 yew 写一个 web 项目"} </a>
                    </span>
                    <span> {"欢迎大家学习, 代码全部开源"}
                        <br/>
                        {"Gtihub 地址: "}
                        <a class="link link-accent" target="_blank" href="https://github.com/jedsek/blog-rust-fullstack"> {"https://github.com/jedsek/blog-rust-fullstack"} </a>
                    </span>
                    {"谢谢你的观看!"}
                </ChatBubble>
            </div>

        </>
    }
}
