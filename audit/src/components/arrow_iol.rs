use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::StyleAttribute;
use leptos::prelude::component;
use leptos::prelude::view;
#[component]
pub fn ArrowIol() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xml:space="preserve"
            style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5"
            // starting viewbox was 0 0 48 48
            viewBox="5 11 35 31"
        >
            <defs>
                <filter
                    id="neon-pink-filter"
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
                        result="magenta-blur"
                        in="blur-merged"
                        type="matrix"
                        values="0.5 0 0 0 0
                        0 0.06 0 0 0
                        0 0 0.44 0 0
                        0 0 0 1.2 0"
                    />

                    <feMerge>
                        <feMergeNode in="magenta-blur" />
                        <feMergeNode in="blur5" />
                        <feMergeNode in="SourceGraphic" />
                    </feMerge>
                </filter>
            </defs>

            <path
                d="M24 31.25s8.425 0 11.386-2.545c3.685-3.167 1.792-10.317-2.79-11.105"
                style="fill:none;stroke:currentColor;stroke-width:1px"
                transform="translate(-.205)"
            />
            <path
                d="M24 31.25s8.425 0 11.386-2.545c3.685-3.167 1.792-10.317-2.79-11.105"
                style="fill:none;stroke:currentColor;stroke-width:1px"
                transform="matrix(-1 0 0 -1 47.77 48.001)"
            />
            <path
                d="M13 14.25 15.25 12 13 9.75M4.75 12a7.25 7.25 0 1 0 14.5 0 7.25 7.25 0 0 0-14.5 0M15 12H8.8"
                style="fill:none;fill-rule:nonzero;stroke:currentColor;stroke-width:1px;stroke-miterlimit:4"
                transform="rotate(-45 32.485 3.515)"
            />
            <path
                d="m16.75 24 1.326.001"
                style="fill:none;stroke:currentColor;stroke-width:.88px"
                transform="matrix(.8784 -.87963 .71388 .70046 -12.75 26.83)"
            />
            <path
                d="m31.25 24-2.266-.022"
                style="fill:none;stroke:currentColor;stroke-width:1.1px"
                transform="matrix(.56771 -.57039 .70576 .70843 -5.689 19.83)"
            />
        </svg>
    }
}
