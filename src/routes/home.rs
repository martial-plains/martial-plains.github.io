use leptos::*;

/// Renders the home page of the application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="relative pt-24 pb-36 overflow-hidden">
            <div class="relative z-10 container px-4 mx-auto">
            <img class="w-32 h-32 md:w-48 md:h-auto rounded-full mx-auto" src="/allister-isaiah-harvey.png" alt="" width="384" height="512" />
            <div class="pt-6 md:p-8 text-center space-y-4">
                <figcaption class="font-medium">
                <div class="text-cyan-600 text-4xl font-bold">
                    {"Allister Isaiah Harvey"}
                </div>
                <div class="text-gray-500 text-lg">
                    {"Hobby Open-Sourcerer & Audio Engineer"}
                </div>
                </figcaption>
            </div>
            <div class="flex space-x-2 justify-center">
                <div class="max-w-none px-6 flex flex-nowrap flex-col sm:flex-row sm:justify-center gap-6">
                <div class="flex flex w-full sm:w-auto">
                    <a href="https://github.com/a-isaiahharvey" class="btn inline-block px-6 pt-2.5 pb-2 bg-gray-900 dark:bg-gray-700 dark:hover:bg-gray-600 text-white font-medium text-sm leading-normal uppercase rounded shadow-md hover:bg-gray-700 hover:shadow-lg focus:bg-gray-700 focus:shadow-lg focus:outline-none focus:ring-0 transition duration-150 ease-in-out ">
                    <i class="fa-brands fa-github"></i>
                    {" GitHub "}
                    </a>
                </div>
                <div class="flex flex w-full sm:w-auto">
                    <a href="https://soundcloud.com/ai-harvey" class="btn px-6 pt-2.5 pb-2 bg-amber-500 dark:bg-amber-700 dark:hover:bg-amber-600 text-white font-medium text-sm leading-normal uppercase rounded shadow-md hover:bg-amber-700 hover:shadow-lg focus:bg-amber-700 focus:shadow-lg focus:outline-none focus:ring-0 transition duration-150 ease-in-out">
                    <i class="fa-brands fa-soundcloud"></i>
                    {" SoundCloud "}
                    </a>
                </div>
                </div>
            </div>
            </div>
        </section>
    }
}
