pub mod components;

use coatcheck::{Catalog, Story, StoryKind};
use components::sign_in_form::sign_in_form;
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{Json, Router, Slot, layout, page, query_params, route},
    tailwind,
    view::view,
};

const STORIES: &[Story] = &[
    Story {
        id: "sign-in-default",
        title: "Sign-in form",
        tier: "Authentication",
        kind: StoryKind::Playground,
        description: "Portable POST form with recovery and account-creation paths.",
        source: "crates/topcoat-blocks/registry/components/sign_in_form.rs",
        args: &[],
    },
    Story {
        id: "sign-in-minimal",
        title: "Sign-in form / minimal",
        tier: "Authentication",
        kind: StoryKind::State,
        description: "The same form contract without optional navigation.",
        source: "crates/topcoat-blocks/registry/components/sign_in_form.rs",
        args: &[],
    },
];
const CATALOG: Catalog = Catalog::new("Topcoat Blocks", STORIES);

#[tokio::main]
async fn main() {
    CATALOG
        .validate()
        .expect("the Topcoat Blocks story catalog is valid");

    let router = Router::builder()
        .layout(root_layout)
        .page(home)
        .page(frame)
        .route(health)
        .route(coatcheck_manifest)
        .assets(AssetBundle::load().expect("workbench assets load"))
        .build();

    topcoat::start(router)
        .await
        .expect("the Topcoat Blocks workbench starts");
}

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta
                    name="description"
                    content="Visual development and semantic review for Topcoat Blocks."
                >
                <title>"Topcoat Blocks / Coatcheck"</title>
                topcoat::dev::script()
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>
            <body>(slot.await?)</body>
        </html>
    }
}

#[page("/")]
async fn home() -> Result {
    let registry_name = topcoat_blocks::REGISTRY_NAME;

    view! {
        <main class="min-h-svh bg-background px-5 py-8 text-foreground sm:px-8 lg:px-12">
            <div class="mx-auto max-w-6xl">
                <header class="border-b border-border pb-8">
                    <p class="text-xs font-semibold tracking-[0.18em] text-muted-foreground uppercase">
                        "Coatcheck review host"
                    </p>
                    <div class="mt-3 flex flex-col justify-between gap-5 md:flex-row md:items-end">
                        <div class="max-w-3xl">
                            <h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">
                                "Topcoat Blocks"
                            </h1>
                            <p class="mt-4 max-w-2xl text-base leading-7 text-muted-foreground">
                                "Registry source enters here, survives responsive and semantic \
                                 review, then ships as copy-to-own Rust."
                            </p>
                        </div>
                        <code class="w-fit rounded-md border border-border bg-muted px-3 py-2 text-sm">
                            (registry_name)
                        </code>
                    </div>
                </header>

                <section class="grid gap-4 py-8 md:grid-cols-3" aria-label="Review contract">
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"1 / Build"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Develop one canonical registry source file."
                        </p>
                    </article>
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"2 / Inspect"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Review real installs across viewports and surfaces."
                        </p>
                    </article>
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"3 / Release"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Publish only after semantic and visual approval."
                        </p>
                    </article>
                </section>

                <section class="rounded-2xl border border-border bg-muted/30 p-4 sm:p-6">
                    <div class="flex flex-col gap-4">
                        <div>
                            <h2 class="text-lg font-semibold">"Authentication"</h2>
                            <p class="mt-1 text-sm text-muted-foreground">
                                "The workbench renders the installed registry source, never a \
                                 parallel demo implementation."
                            </p>
                        </div>
                        <div class="grid gap-3 md:grid-cols-2">
                            <a
                                href="/__coatcheck/frame?story=sign-in-default&scene=desktop-light"
                                class="group rounded-xl border border-border bg-background p-4 shadow-xs transition-colors hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                            >
                                <span class="block text-sm font-medium">"Sign-in form"</span>
                                <span class="mt-1 block text-sm leading-6 text-muted-foreground">
                                    "Recovery and account-creation paths."
                                </span>
                            </a>
                            <a
                                href="/__coatcheck/frame?story=sign-in-minimal&scene=desktop-light"
                                class="group rounded-xl border border-border bg-background p-4 shadow-xs transition-colors hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                            >
                                <span class="block text-sm font-medium">"Sign-in form / minimal"</span>
                                <span class="mt-1 block text-sm leading-6 text-muted-foreground">
                                    "Only the required authentication controls."
                                </span>
                            </a>
                        </div>
                    </div>
                </section>
            </div>
        </main>
    }
}

#[query_params(error = bad_request)]
struct FrameQuery {
    story: String,
    #[serde(default)]
    scene: Option<String>,
}

#[page("/__coatcheck/frame")]
async fn frame(cx: &Cx) -> Result {
    let query = query_params::<FrameQuery>(cx)?;
    let scene = query.scene.as_deref().unwrap_or("desktop-light");
    let dark = scene.ends_with("-dark");

    if CATALOG.story(&query.story).is_none() {
        return view! {
            <main
                data-coatcheck-frame=""
                data-coatcheck-error=""
                role="alert"
                class="min-h-svh bg-background p-8 text-foreground"
            >
                <h1 class="text-xl font-semibold">"Unknown story"</h1>
                <p class="mt-2 text-muted-foreground">(query.story.as_str())</p>
            </main>
        };
    }

    let class_name = if dark {
        "dark min-h-svh bg-background text-foreground"
    } else {
        "min-h-svh bg-background text-foreground"
    };

    view! {
        <main
            data-coatcheck-frame=""
            data-story-id=(query.story.as_str())
            data-scene-id=(scene)
            class=(class_name)
        >
            <div class="flex min-h-svh items-center justify-center bg-muted/40 p-4 sm:p-8">
                <h1 class="sr-only">"Sign-in form review"</h1>
                <h2 class="sr-only">"Authentication block"</h2>
                match query.story.as_str() {
                    "sign-in-default" => sign_in_form(
                        action: "/session",
                        forgot_password_href: Some("/forgot-password".to_owned()),
                        sign_up_href: Some("/sign-up".to_owned()),
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="sign-in-form"
                        },
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "sign-in-minimal" => sign_in_form(
                        action: "/session",
                        id_prefix: "minimal",
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="sign-in-form"
                        }
                    ),
                    _ => {},
                }
            </div>
            <span data-coatcheck-ready="" hidden="">"ready"</span>
        </main>
    }
}

fn build_manifest() -> coatcheck::contract::ManifestV1 {
    use coatcheck::contract::{FrameContractV1, ManifestV1};

    let mut manifest = ManifestV1::from_catalog(CATALOG, FrameContractV1::default());
    manifest.scenes = vec![
        scene("mobile-light", "Mobile light", 393, 852, "light"),
        scene("tablet-light", "Tablet light", 768, 1024, "light"),
        scene("desktop-light", "Desktop light", 1280, 900, "light"),
        scene("mobile-dark", "Mobile dark", 393, 852, "dark"),
        scene("desktop-dark", "Desktop dark", 1280, 900, "dark"),
    ];
    manifest
}

fn scene(
    id: &str,
    label: &str,
    width: u16,
    height: u16,
    surface: &str,
) -> coatcheck::contract::SceneV1 {
    coatcheck::contract::SceneV1 {
        id: id.to_owned(),
        label: label.to_owned(),
        viewport: coatcheck::contract::ViewportV1 { width, height },
        surface: surface.to_owned(),
    }
}

#[route(GET "/__coatcheck/manifest.json")]
async fn coatcheck_manifest() -> Result<Json<coatcheck::contract::ManifestV1>> {
    let manifest = build_manifest();
    manifest
        .validate()
        .expect("the served Coatcheck manifest is valid");
    Ok(Json(manifest))
}

#[route(GET "/health")]
async fn health() -> Result<&'static str> {
    Ok("topcoat-blocks:ok")
}

#[cfg(test)]
mod tests {
    use super::{CATALOG, build_manifest};

    #[test]
    fn catalog_and_manifest_are_valid() {
        assert_eq!(CATALOG.validate(), Ok(()));
        assert_eq!(build_manifest().validate(), Ok(()));
        assert_eq!(build_manifest().cases().len(), 10);
    }
}
