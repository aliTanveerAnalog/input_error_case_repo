use dioxus::prelude::*;
use dioxus_desktop;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let title = use_state(&cx, || "Title");

    cx.render(rsx! {
        div {
            h1{
                    "{title}"
            }
            //error case component i.e comment input will pass panic
            input{
                placeholder:"input",
                value:"",
            }
            // button to generate rust panic issue
            button{
                onclick: move |_evt|{
                    title.set("New title");
                },
                "click me"
            }
        }
    })
}
