use super::{
    button::button,
    card::{card, card_content, card_description, card_footer, card_header, card_title},
    input::input,
    label::label,
};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// A portable sign-in form for server-rendered Topcoat applications.
///
/// The component owns the form structure and browser semantics while leaving
/// authentication to the host application. `action` is the endpoint that
/// receives an `email` and `password` POST. Child nodes are rendered first in
/// the form body, which makes the ordinary child slot useful for CSRF tokens
/// and other hidden fields.
///
/// The root `<form>` forwards `attrs`; use `class` to size or position the
/// block. Set `id_prefix` when more than one sign-in form can appear in the
/// same document so every label continues to point to a unique control.
///
/// ```ignore
/// view! {
///     sign_in_form(
///         action: "/session",
///         forgot_password_href: Some("/forgot-password".to_owned()),
///         sign_up_href: Some("/sign-up".to_owned()),
///         attrs: attributes! { class="w-full max-w-sm" },
///         <input type="hidden" name="csrf_token" value=(csrf_token)>
///     )
/// }
/// ```
#[component]
pub async fn sign_in_form(
    #[into] action: String,
    #[default] forgot_password_href: Option<String>,
    #[default] sign_up_href: Option<String>,
    #[default]
    #[into]
    id_prefix: String,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let id_prefix = if id_prefix.is_empty() {
        "sign-in".to_owned()
    } else {
        id_prefix
    };
    let email_id = format!("{id_prefix}-email");
    let password_id = format!("{id_prefix}-password");

    view! {
        <form
            method="post"
            action=(action)
            class=(class!("w-full", attrs.remove("class")))
            (attrs)
        >
            card(
                card_header(
                    attrs: attributes! { class="text-center" },
                    card_title(
                        attrs: attributes! { class="text-2xl tracking-tight" },
                        "Welcome back"
                    )
                    card_description("Enter your email and password to continue.")
                )
                card_content(
                    <div class="grid gap-5">
                        (child)
                        <div class="grid gap-2">
                            label(
                                attrs: attributes! { for=(email_id.as_str()) },
                                "Email"
                            )
                            input(attrs: attributes! {
                                id=(email_id)
                                name="email"
                                type="email"
                                inputmode="email"
                                autocomplete="email"
                                autocapitalize="none"
                                spellcheck="false"
                                required=""
                                placeholder="you@example.com"
                            })
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between gap-4">
                                label(
                                    attrs: attributes! { for=(password_id.as_str()) },
                                    "Password"
                                )
                                match forgot_password_href {
                                    Some(href) => <a
                                        href=(href)
                                        class="rounded-sm text-sm text-muted-foreground underline-offset-4 hover:text-foreground hover:underline focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                                    >
                                        "Forgot password?"
                                    </a>,
                                    None => {},
                                }
                            </div>
                            input(attrs: attributes! {
                                id=(password_id)
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                required=""
                            })
                        </div>
                        button(
                            attrs: attributes! {
                                type="submit"
                                class="min-h-11 w-full"
                            },
                            "Sign in"
                        )
                    </div>
                )
                match sign_up_href {
                    Some(href) => card_footer(
                        attrs: attributes! {
                            class="justify-center border-t border-border pt-5 text-sm text-muted-foreground"
                        },
                        <p>
                            "New here? "
                            <a
                                href=(href)
                                class="rounded-sm font-medium text-foreground underline-offset-4 hover:underline focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                            >
                                "Create an account"
                            </a>
                        </p>
                    ),
                    None => {},
                }
            )
        </form>
    }
}
