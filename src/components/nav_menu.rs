use leptos::*;
use leptos_router::*;

pub enum Msg {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy)]
pub struct LinkItem {
    id: usize,
    text: &'static str,
    link: &'static str,
}

pub const LINK_ITEMS: &[LinkItem] = &[
    LinkItem {
        id: 0,
        text: "Donate",
        link: "/donate",
    },
    LinkItem {
        id: 1,
        text: "About",
        link: "/about",
    },
    LinkItem {
        id: 2,
        text: "Contact",
        link: "/contact",
    },
    LinkItem {
        id: 3,
        text: "Blog",
        link: "https://medium.com/@allisterharvey",
    },
];

#[component]
pub fn NavMenuItemComponent(cx: Scope, item: LinkItem) -> impl IntoView {
    view! { cx,
        <li>
                <a href={item.link} class={"block py-2 pr-4 pl-3 text-gray-700 md:text-base rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent".to_string()}>{item.text}</a>
        </li>
    }
}

#[component]
pub fn NavMenu(cx: Scope) -> impl IntoView {
    let (open, set_open) = create_signal(cx, false);

    let glassmorphism = stylist::Style::new(
        r"
            background: rgba(255, 255, 255, 0.07);
            backdrop-filter: blur(5px);
            -webkit-backdrop-filter: blur(5px);
        ",
    )
    .expect("Unable to create Style");

    let onclick_navbar_button = move |_| {
        log::info!("{}", open());
        set_open.update(|open| *open = !*open);
    };

    view! { cx,
        <nav class={format!("{} bg-white border-gray-200 px-2 sm:px-4 py-2.5 dark:bg-gray-900", glassmorphism.get_class_name())}>
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                <A href="/" class="flex items-center md:text-xl font-bold".to_string()>
                    <span class="text-slate-500 dark:text-slate-400">{"Allister I. Harvey"}</span>
                </A>
                <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false" on:click=onclick_navbar_button>
                    <span class="sr-only">{"Open main menu"}</span>
                    <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" data-darkreader-inline-fill="" style="--darkreader-inline-fill: currentColor;"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
                </button>
                {
                    move ||
                        view!{ cx,
                        <div class={format!("{}w-full md:block md:w-auto", if !open() { "hidden "} else { "" })} id="navbar-default">
                            <ul class={format!("{} flex flex-col p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700", glassmorphism.get_class_name())}>

                                <For
                                    each=move || LINK_ITEMS.to_vec()
                                    key=|link| link.id
                                    view=move |item: LinkItem| {
                                        view! { cx,
                                            <NavMenuItemComponent item />
                                        }
                                    }
                                />

                            </ul>
                        </div>
                    }
                }

            </div>
        </nav>
    }
}
