use leptos::*;

#[component]
pub fn MusicPage() -> impl IntoView {
    view! {
        <section class="relative pt-24 pb-36 overflow-hidden">
            <div class="relative z-10 container px-4 mx-auto">
                <h2 class="mb-5 text-6xl md:text-8xl xl:text-10xl text-center font-bold font-heading tracking-px-n leading-none text-slate-900 dark:text-slate-50">{"Music"}</h2>
            </div>

            <iframe style="border-radius:12px" src="https://open.spotify.com/embed/artist/6dNUtjVk2PTuchPwmnfZYp?utm_source=generator" width="100%" height="372" frameBorder="0" allowfullscreen="" allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture" loading="lazy"></iframe>

        </section>
    }
}
