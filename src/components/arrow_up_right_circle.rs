use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, IntoView, component, view};

#[component]
pub fn ArrowUpRightCircle() -> impl IntoView {
    view! {
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <filter
                    id="neon-pink-svg"
                    filterUnits="userSpaceOnUse"
                    x="-50%"
                    y="-50%"
                    width="200%"
                    height="200%"
                >
                    <feGaussianBlur in="SourceGraphic" stdDeviation="5" result="blur5" />
                    <feGaussianBlur in="SourceGraphic" stdDeviation="10" result="blur10" />
                    <feGaussianBlur in="SourceGraphic" stdDeviation="20" result="blur20" />
                    <feGaussianBlur in="SourceGraphic" stdDeviation="30" result="blur30" />
                    <feGaussianBlur in="SourceGraphic" stdDeviation="50" result="blur50" />

                    <feMerge result="blur-merged">
                        <feMergeNode in="blur10" />
                        <feMergeNode in="blur20" />
                        <feMergeNode in="blur30" />
                        <feMergeNode in="blur50" />
                    </feMerge>

                    <feColorMatrix
                        result="red-blur"
                        in="blur-merged"
                        type="matrix"
                        values="0.5 0 0 0 0
                        0 0.06 0 0 0
                        0 0 0.44 0 0
                        0 0 0 1 0"
                    />

                    <feMerge>
                        <feMergeNode in="red-blur" />
                        <feMergeNode in="blur5" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>

            <path
                d="M19.25 12C19.25 16.0041 16.0041 19.25 12 19.25C7.99594 19.25 4.75 16.0041 4.75 12C4.75 7.99594 7.99594 4.75 12 4.75C16.0041 4.75 19.25 7.99594 19.25 12Z"
                stroke="currentColor"
                stroke-width="1.2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M14.25 13.25V9.75H10.75"
                stroke="currentColor"
                stroke-width="1.2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M14 10L9.75 14.25"
                stroke="currentColor"
                stroke-width="1.2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}
