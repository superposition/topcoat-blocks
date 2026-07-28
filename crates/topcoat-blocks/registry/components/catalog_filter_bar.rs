use super::{button::button, input::input, label::label, select::select};
use topcoat::{
    Result,
    view::{Attributes, attributes, class, component, view},
};

/// One option in a catalog category or sort control.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogFilterOption {
    pub value: String,
    pub label: String,
    pub selected: bool,
}

impl CatalogFilterOption {
    #[must_use]
    pub fn new(
        value: impl Into<String>,
        option_label: impl Into<String>,
        selected: bool,
    ) -> Self {
        Self {
            value: value.into(),
            label: option_label.into(),
            selected,
        }
    }
}

/// Native GET form for catalog search, category, and sorting.
///
/// The host owns the query contract and result set. This block only preserves
/// labels, names, and browser submission semantics.
#[component]
pub async fn catalog_filter_bar(
    #[into] action: String,
    #[default]
    #[into]
    query: String,
    categories: Vec<CatalogFilterOption>,
    sorts: Vec<CatalogFilterOption>,
    #[default] mut attrs: Attributes,
) -> Result {
    view! {
        <form
            method="get"
            action=(action)
            data-topcoat-block="catalog-filter-bar"
            class=(class!("grid gap-4 rounded-xl border border-border bg-background p-4 md:grid-cols-[minmax(0,1fr)_auto_auto_auto] md:items-end", attrs.remove("class")))
            (attrs)
        >
            <div class="grid gap-2">
                label(attrs: attributes! { for="catalog-search" }, "Search")
                input(attrs: attributes! {
                    id="catalog-search"
                    name="q"
                    type="search"
                    value=(query)
                    autocomplete="off"
                    placeholder="Search products"
                })
            </div>
            <div class="grid gap-2">
                label(attrs: attributes! { for="catalog-category" }, "Category")
                select(
                    attrs: attributes! { id="catalog-category" name="category" },
                    for option in categories {
                        match option.selected {
                            true => <option value=(option.value) selected="">(option.label)</option>,
                            false => <option value=(option.value)>(option.label)</option>,
                        }
                    }
                )
            </div>
            <div class="grid gap-2">
                label(attrs: attributes! { for="catalog-sort" }, "Sort")
                select(
                    attrs: attributes! { id="catalog-sort" name="sort" },
                    for option in sorts {
                        match option.selected {
                            true => <option value=(option.value) selected="">(option.label)</option>,
                            false => <option value=(option.value)>(option.label)</option>,
                        }
                    }
                )
            </div>
            button(
                attrs: attributes! {
                    type="submit"
                    class="min-h-11"
                },
                "Apply"
            )
        </form>
    }
}
