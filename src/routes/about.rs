use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <section class="relative pt-24 pb-36 overflow-hidden">
            <div class="relative z-10 container px-4 mx-auto">
            <h2 class="mb-5 text-6xl md:text-8xl xl:text-10xl text-center font-bold font-heading tracking-px-n leading-none text-slate-900 dark:text-slate-50">{"About"}</h2>
            <p class="mb-20 text-lg text-slate-500 dark:text-slate-400 text-center font-medium leading-normal md:max-w-16  mx-auto">{"I am passionate about becoming a full-time open-source programmer, driven by the desire to contribute to the development of free and open software that can benefit everyone in their day-to-day lives. Programming has always been my true passion, and I find great fulfillment in helping people solve coding challenges, as I believe it not only aids developers but also empowers end users and inspires future learners."}</p>
            <h3 class="mb-5 text-2xl md:text-6xl xl:text-8xl text-center font-bold font-heading tracking-px-n leading-none text-slate-900 dark:text-slate-50">{"Links"}</h3>

            <div class="flex justify-center">
                <a href="https://github.com/a-isaiahharvey" data-mdb-ripple="true" data-mdb-ripple-color="light" class="btn px-6 py-2.5 mb-2 text-white font-medium text-xs leading-normal rounded shadow-md hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out" style="background-color: #333333;">
                <i class="fa-brands fa-github"></i>
                {" GitHub "}
                </a>
            </div>

            <div class="flex justify-center">
                <a href="https://twitter.com/AllisterIsaiah" data-mdb-ripple="true" data-mdb-ripple-color="light" class="btn px-6 py-2.5 mb-2 text-white font-medium text-xs leading-normal rounded shadow-md hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out" style="background-color: #1da1f2;">
                <i class="fa-brands fa-twitter"></i>
                {" Twitter "}
                </a>
            </div>

            <div class="flex justify-center">
                <a href="https://www.instagram.com/isaiah.harvey" data-mdb-ripple="true" data-mdb-ripple-color="light" class="btn px-6 py-2.5 mb-2 text-white font-medium text-xs leading-normal rounded shadow-md hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out" style="background-color: #E4405F;">
                <i class="fa-brands fa-instagram"></i>
                {" Instagram "}
                </a>
            </div>
            <div class="flex justify-center">
                <a href="https://soundcloud.com/ai-harvey" data-mdb-ripple="true" data-mdb-ripple-color="light" class="btn px-6 py-2.5 mb-2 text-white font-medium text-xs leading-normal rounded shadow-md hover:shadow-lg focus:shadow-lg focus:outline-none focus:ring-0 active:shadow-lg transition duration-150 ease-in-out" style="background-color: #FE6D35;">
                <i class="fa-brands fa-soundcloud"></i>
                {" SoundCloud "}
                </a>
            </div>
            </div>
        </section>
    }
}
