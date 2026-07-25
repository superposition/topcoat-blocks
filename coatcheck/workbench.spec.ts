import { expect, test } from "@playwright/test"

test("workbench exposes the isolated review frame", async ({ page }) => {
  await page.goto("http://127.0.0.1:3201")

  await expect(
    page.getByRole("heading", { name: "Topcoat Blocks" })
  ).toBeVisible()
  const defaultStory = page.locator(
    'a[href="/__coatcheck/frame?story=sign-in-default&scene=desktop-light"]'
  )
  await expect(defaultStory).toContainText("Sign-in form")
})
