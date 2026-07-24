import { expect, test } from "@playwright/test"

const defaultFrame =
  "http://127.0.0.1:3201/__coatcheck/frame?story=sign-in-default&scene=desktop-light"
const minimalFrame =
  "http://127.0.0.1:3201/__coatcheck/frame?story=sign-in-minimal&scene=mobile-dark"

test("sign-in form keeps the server POST and browser semantics intact", async ({
  page,
}) => {
  await page.goto(defaultFrame)

  const form = page.locator('form[data-review-subject="sign-in-form"]')
  const email = page.getByLabel("Email")
  const password = page.getByLabel("Password")
  const submit = page.getByRole("button", { name: "Sign in" })

  await expect(form).toHaveAttribute("method", "post")
  await expect(form).toHaveAttribute("action", "/session")
  await expect(form.locator('input[name="csrf_token"]')).toHaveValue(
    "review-token"
  )
  await expect(email).toHaveAttribute("name", "email")
  await expect(email).toHaveAttribute("type", "email")
  await expect(email).toHaveAttribute("autocomplete", "email")
  await expect(email).toHaveAttribute("required", "")
  await expect(password).toHaveAttribute("name", "password")
  await expect(password).toHaveAttribute("autocomplete", "current-password")
  await expect(password).toHaveAttribute("required", "")
  await expect(submit).toHaveAttribute("type", "submit")
  expect(
    await submit.evaluate(
      (node) => node.closest("form")?.getAttribute("data-review-subject")
    )
  ).toBe("sign-in-form")
  await expect(
    page.getByRole("link", { name: "Forgot password?" })
  ).toHaveAttribute("href", "/forgot-password")
  await expect(
    page.getByRole("link", { name: "Create an account" })
  ).toHaveAttribute("href", "/sign-up")

  await email.focus()
  await page.keyboard.press("Tab")
  await expect(
    page.getByRole("link", { name: "Forgot password?" })
  ).toBeFocused()
  await page.keyboard.press("Tab")
  await expect(password).toBeFocused()
  await page.keyboard.press("Tab")
  await expect(submit).toBeFocused()

  await email.fill("person@example.com")
  await password.fill("correct horse battery staple")
  await form.evaluate((node) => {
    node.addEventListener("submit", (event) => {
      event.preventDefault()
      node.setAttribute("data-submitted", "true")
    })
  })
  await password.press("Enter")
  await expect(form).toHaveAttribute("data-submitted", "true")
})

test("minimal sign-in form omits optional navigation and keeps unique ids", async ({
  page,
}) => {
  await page.goto(minimalFrame)

  await expect(page.getByRole("link")).toHaveCount(0)
  await expect(page.getByLabel("Email")).toHaveAttribute("id", "minimal-email")
  await expect(page.getByLabel("Password")).toHaveAttribute(
    "id",
    "minimal-password"
  )
})
