import { expect, test } from "@playwright/test"

const frame = (story: string, scene = "desktop-light") =>
  `http://127.0.0.1:3201/__coatcheck/frame?story=${story}&scene=${scene}`

test("product card preserves inventory and add-to-cart semantics", async ({
  page,
}) => {
  await page.goto(frame("product-card-available"))

  const subject = page.locator('[data-review-subject="product-card"]')
  const form = subject.locator("form")

  await expect(subject).toHaveAttribute("data-state", "available")
  await expect(
    subject.getByRole("link", { name: "Review field pack", exact: true })
  ).toHaveAttribute("href", "/products/review-field-pack")
  await expect(form).toHaveAttribute("method", "post")
  await expect(form).toHaveAttribute("action", "/cart/items")
  await expect(form.locator('input[name="product_id"]')).toHaveValue(
    "review-product"
  )
  await expect(form.locator('input[name="csrf_token"]')).toHaveValue(
    "review-token"
  )
  await expect(subject.getByRole("button", { name: "Add to cart" })).toHaveAttribute(
    "type",
    "submit"
  )

  await page.goto(frame("product-card-sold-out", "mobile-dark"))
  await expect(page.locator('[data-review-subject="product-card"]')).toHaveAttribute(
    "data-state",
    "sold-out"
  )
  await expect(page.getByRole("button", { name: "Unavailable" })).toBeDisabled()
})

test("catalog filter uses a shareable native GET query", async ({ page }) => {
  await page.goto(frame("catalog-filter-bar", "mobile-light"))

  const form = page.locator('[data-review-subject="catalog-filter-bar"]')
  await expect(form).toHaveAttribute("method", "get")
  await expect(form).toHaveAttribute("action", "/shop")
  await expect(page.getByLabel("Search")).toHaveAttribute("name", "q")
  await expect(page.getByLabel("Category")).toHaveValue("packs")
  await expect(page.getByLabel("Sort")).toHaveValue("featured")
  await expect(page.getByRole("button", { name: "Apply" })).toHaveAttribute(
    "type",
    "submit"
  )
})

test("cart line keeps update and remove in one host-owned POST", async ({
  page,
}) => {
  await page.goto(frame("cart-line-item"))

  const form = page.locator('[data-review-subject="cart-line-item"]')
  const quantity = page.getByLabel("Quantity")

  await expect(form).toHaveAttribute("method", "post")
  await expect(form).toHaveAttribute("action", "/cart/items/review-line")
  await expect(quantity).toHaveAttribute("name", "quantity")
  await expect(quantity).toHaveAttribute("min", "1")
  await expect(quantity).toHaveAttribute("max", "4")
  await expect(page.getByRole("button", { name: "Update" })).toHaveAttribute(
    "value",
    "update"
  )
  await expect(page.getByRole("button", { name: "Remove" })).toHaveAttribute(
    "value",
    "remove"
  )
})

test("shipping form exposes ordinary address names and validation errors", async ({
  page,
}) => {
  await page.goto(frame("shipping-address"))

  const form = page.locator('[data-review-subject="shipping-address-form"]')
  await expect(form).toHaveAttribute("method", "post")
  await expect(form).toHaveAttribute("action", "/checkout/address")
  await expect(page.getByLabel("Full name")).toHaveAttribute("autocomplete", "name")
  await expect(page.getByLabel("Street address")).toHaveAttribute(
    "autocomplete",
    "shipping address-line1"
  )
  await expect(form.locator('input[name="country"]')).toHaveValue("US")
  await expect(form.locator('input[name="csrf_token"]')).toHaveValue(
    "review-token"
  )

  await page.goto(frame("shipping-address-errors", "mobile-dark"))
  await expect(page.getByRole("alert")).toContainText(
    "Check the highlighted address fields."
  )
  await expect(page.getByText("Enter a valid postal code.")).toHaveCount(2)
})

test("payment routes are native radios and unavailable routes stay disabled", async ({
  page,
}) => {
  await page.goto(frame("payment-routes"))

  const form = page.locator('[data-review-subject="payment-route-selector"]')
  await expect(form).toHaveAttribute("method", "post")
  await expect(form).toHaveAttribute("action", "/payments/intents")
  await expect(page.getByRole("radio", { name: /Solana/ })).toBeChecked()
  await expect(page.getByRole("radio", { name: /Unavailable network/ })).toBeDisabled()
  await expect(page.getByRole("button", { name: "Continue to wallet" })).toHaveAttribute(
    "type",
    "submit"
  )
})

test("payment status and receipt remain read-only recovery surfaces", async ({
  page,
}) => {
  await page.goto(frame("payment-confirming"))

  const status = page.locator('[data-review-subject="payment-status"]')
  await expect(status).toHaveAttribute("aria-live", "polite")
  await expect(status).toHaveAttribute("data-state", "confirming")
  await expect(page.getByRole("progressbar")).toHaveAttribute(
    "aria-label",
    "Payment confirmation progress"
  )

  await page.goto(frame("payment-paid", "desktop-dark"))
  await expect(page.locator('[data-review-subject="payment-status"]')).toHaveAttribute(
    "data-state",
    "paid"
  )
  await expect(page.getByRole("link", { name: "View transaction" })).toHaveAttribute(
    "rel",
    "external noreferrer"
  )

  await page.goto(frame("order-receipt"))
  await expect(page.locator('[data-review-subject="order-receipt"]')).toContainText(
    "CF-000184"
  )
  await expect(page.getByRole("link", { name: "Continue shopping" })).toHaveAttribute(
    "href",
    "/shop"
  )
})
