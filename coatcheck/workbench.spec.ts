import { expect, test } from "@playwright/test"

test("workbench exposes the isolated review frame", async ({ page }) => {
  await page.goto("http://127.0.0.1:3201")

  await expect(
    page.getByRole("heading", { name: "Topcoat Blocks" })
  ).toBeVisible()
  await expect(
    page.getByRole("link", { name: "Open isolated frame" })
  ).toHaveAttribute(
    "href",
    "/__coatcheck/frame?story=workbench-overview&scene=desktop-light"
  )
})

