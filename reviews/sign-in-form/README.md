# Sign-in form review

Reviewed on 2026-07-24 from the copy-installed `sign_in_form` registry source.

## Evidence

- [Desktop light](desktop-light.png)
- [Mobile light](mobile-light.png)
- [Desktop dark](desktop-dark.png)

Coatcheck generated ten cases: default and minimal forms at mobile, tablet, and
desktop widths across the configured light and dark surfaces. All 13 browser
tests passed. Every generated case reported zero Axe violations and zero
browser or page errors.

The behavioral checks cover:

- POST action and field names;
- visible label associations;
- email and password autocomplete;
- required controls and submit ownership;
- hidden-field composition for CSRF;
- optional navigation;
- keyboard focus order;
- Enter-key submission;
- unique IDs for multiple-instance pages.

The retained PNGs are selected review evidence. They are not automatic pixel
baselines.
