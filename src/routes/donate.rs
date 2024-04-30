use leptos::*;
use leptos_router::*;

#[component]
pub fn DonatePage() -> impl IntoView {
    view! {
        <section class="relative pt-6 pb-36 overflow-hidden">
            <div class="container mx-auto px-6 sm:px-6 max-w-3xl prose prose-lg lg:prose-xl dark:prose-invert dark:prose-headings:text-slate-300 prose-headings:font-heading prose-headings:leading-tighter prose-headings:tracking-tighter prose-headings:font-bold prose-img:rounded-md prose-img:shadow-lg mt-8 prose-a:text-black/75 dark:prose-a:text-white/90 prose-a:underline prose-a:underline-offset-4 prose-a:decoration-primary-500 hover:prose-a:decoration-primary-600 prose-a:decoration-2 hover:prose-a:decoration-4 hover:prose-a:text-black dark:hover:prose-a:text-white break-words tracking-normal prose-h4:tracking-normal prose-h5:tracking-normal prose-h6:tracking-normal">
            <h2 class="mb-5 text-6xl font-bold font-heading tracking-px-n leading-none text-slate-900 dark:text-slate-50">{"Donate"}</h2>
            <p class="text-slate-900 dark:text-slate-50">{"If you or your company are using any of my projects, consider supporting me so I can continue "}
                <a class="underline underline-offset-4 decoration-2 hover:decoration-sky-400 decoration-sky-500" href="https://github.com/a-isaiahharvey">{"my open source work"}</a>
                {"."}
            </p>
            <h3 class="pt-8 mb-5 text-2xl md:text-4xl font-bold font-heading tracking-tight leading-none text-slate-900 dark:text-slate-50">{"Monthly donations"}</h3>
            <div class="flex">
                <ul class="rounded-lg w-96 text-gray-900 text-left list-disc">
                    <li>
                        <A class={"text-slate-900 dark:text-slate-50 text-2xl".to_string()} href="https://github.com/sponsors/martial-plains">{"GitHub Sponsors"}</A>
                    </li>
                    <li>
                        <A class={"text-slate-900 dark:text-slate-50 text-2xl".to_string()} href="https://www.patreon.com/aiharvey">{"Patreon"}</A>
                    </li>
                </ul>
            </div>
            </div>
        </section>
    }
}
