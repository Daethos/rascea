use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="The Ascea"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Okay this is the equivalent of Solid.js here, use those principles and you'll be fine
    let (count, set_count) = create_signal(cx, 0);
    let inc_count = move |_| set_count.update(|count| *count += 1);
    let dec_count = move |_| set_count.update(|count| *count -= 1);

    let (name, set_name) = create_signal(cx, "user".to_string());

    view! { cx,
        <h1>"Welcome to the Ascea!"</h1>
        <p>"At the moment there is simply a counter button built that can increment. Nothing too wild."</p>
        <button on:click=inc_count>"+"</button>
        <button on:click=dec_count>"-"</button>    
        <h2>"This is a work in progress! Current Count: "{count}</h2>
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <h2>"Username: "{name}</h2>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
