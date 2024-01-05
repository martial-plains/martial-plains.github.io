use leptos::*;

#[component]
pub fn MusicPage() -> impl IntoView {
    view! {
        <section class="relative pt-24 pb-36 overflow-hidden">
            <div class="relative z-10 container px-4 mx-auto">
                <h2 class="mb-5 text-6xl md:text-8xl xl:text-10xl text-center font-bold font-heading tracking-px-n leading-none text-slate-900 dark:text-slate-50">{"Music"}</h2>
            </div>

            <iframe width="100%" height="300" scrolling="no" frameborder="no" allow="autoplay" src="https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/1373666239&color=%23ff5500&auto_play=true&hide_related=false&show_comments=true&show_user=true&show_reposts=false&show_teaser=true&visual=true"></iframe><div style="font-size: 10px; color: #cccccc;line-break: anywhere;word-break: normal;overflow: hidden;white-space: nowrap;text-overflow: ellipsis; font-family: Interstate,Lucida Grande,Lucida Sans Unicode,Lucida Sans,Garuda,Verdana,Tahoma,sans-serif;font-weight: 100;"><a href="https://soundcloud.com/ai-harvey" title="Allister Isaiah Harvey" target="_blank" style="color: #cccccc; text-decoration: none;">{"Allister Isaiah Harvey"}</a> {"·"} <a href="https://soundcloud.com/ai-harvey/project-32-1" title="Project 32" target="_blank" style="color: #cccccc; text-decoration: none;">{"Project 32"}</a></div>

        </section>
    }
}
