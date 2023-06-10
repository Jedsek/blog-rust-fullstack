use crate::component::{Card, ChatBubble, Link};
use crate::utils::set_title;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    set_title("Home");

    let bilibili_url = "https://www.bilibili.com/video/BV1pY4y1Z7jR";
    let github_url = "https://github.com/jedsek/blog-rust-fullstack";

    html! {
        <>
            <Card title={"首页"}>
                <div>
                    <span class="float-start"> {"这是首页"} </span>
                </div>
            </Card>

            <div class="indicator m-4">
                <span class="indicator-item badge badge-secondary text-xl font-bold">{"4+"}</span>
                <div class="mockup-window border border-base-300 bg-base-150">
                    <div class="px-5 py-7 pr-52 bg-neutral-focus">
                        <ChatBubble>
                            <span> {"这是一个用 Rust 写的  yew  +  actix-web  +  sqlx(sqlite)  +  tailwindcss(daisyui)  的博客练手项目 仅用于演示"} </span>
                            <span>
                                {"思路来源于B站"}
                                <br/>
                                {"B站视频地址: "}
                                <Link url={bilibili_url} display={"Rust: 用 ntex 和 yew 写一个 web 项目"}/>
                            </span>
                            <span> {"欢迎大家学习, 代码全部开源"}
                                <br/>
                                {"Gtihub 地址: "}
                                <Link url={github_url} display={"blog-rust-fullstack"}/>
                            </span>
                            {"谢谢你的到来!"}
                        </ChatBubble>
                    </div>
                </div>
            </div>

        </>
    }
}
