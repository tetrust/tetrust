use stylist::css;
use stylist::yew::Global;
use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    let is_login = false;

    let css = css!(
        r##"
        .thumb {
            max-width: 40px;
            max-height: 40px;
        }

        header {
            display: flex;
            justify-content: space-between;
        }

        .corner {
            width: 3em;
            height: 3em;
        }

        .corner a {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 100%;
        }

        .corner img {
            width: 2em;
            height: 2em;
            object-fit: contain;
        }

        nav {
            display: flex;
            justify-content: center;
            --background: rgba(255, 255, 255, 0.7);
        }

        svg {
            width: 2em;
            height: 3em;
            display: block;
        }

        path {
            fill: var(--background);
        }

        ul {
            position: relative;
            padding: 0;
            margin: 0;
            height: 3em;
            display: flex;
            justify-content: center;
            align-items: center;
            list-style: none;
            background: var(--background);
            background-size: contain;
        }

        li {
            position: relative;
            height: 100%;
        }

        li[aria-current='page']::before {
            --size: 6px;
            content: '';
            width: 0;
            height: 0;
            position: absolute;
            top: 0;
            left: calc(50% - var(--size));
            border: var(--size) solid transparent;
            border-top: var(--size) solid var(--color-theme-1);
        }

        nav a {
            display: flex;
            height: 100%;
            align-items: center;
            padding: 0 0.5rem;
            color: var(--color-text);
            font-weight: 700;
            font-size: 0.8rem;
            text-transform: uppercase;
            letter-spacing: 0.1em;
            text-decoration: none;
            transition: color 0.2s linear;
        }

        a:hover {
            color: var(--color-theme-1);
        }
    "##
    );

    html! {
        <>
            <Global css={css} />

            <header>
                <div class="corner">
                    <a href="/">
                        <img src="/resource/svg/rust.svg" alt="SvelteKit" />
                    </a>
                </div>

                <nav>
                    <svg viewBox="0 0 2 3" aria-hidden="true">
                        <path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
                    </svg>
                    <ul>
                        <li>
                            <a href="/">{"AAAAAAAAA"}</a>
                        </li>
                        <li>
                            <a href="/about">{"BBBBBBBBB"}</a>
                        </li>
                    </ul>
                    <svg viewBox="0 0 2 3" aria-hidden="true">
                        <path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
                    </svg>
                </nav>

                <div class="corner">
                    {
                        if is_login {
                            html! {
                                <a href="/mypage">
                                    <img src="/resource/png/statistics-64.png" alt="MyStats" />
                                </a>
                            }
                        }
                        else {
                            html! {
                                <a href="/login">
                                    <img src="/resource/svg/login.svg" alt="Login" />
                                </a>
                            }
                        }
                    }
                </div>
            </header>
        </>
    }
}
