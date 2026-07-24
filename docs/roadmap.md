# Block roadmap

The roadmap follows the useful product families visible in
[shadcn/ui Blocks](https://ui.shadcn.com/blocks), translated into
server-rendered Topcoat contracts rather than copied React implementations.

## Sequence

### 1. Authentication

- [x] Sign-in form
- [ ] Sign-up form
- [ ] Password recovery form
- [ ] Verification-code form

Authentication blocks establish form composition, error summaries, pending
states, and provider-neutral routing before more complex shells depend on them.

### 2. Application shell

- [ ] Responsive sidebar shell
- [ ] Account and team navigation
- [ ] Breadcrumb header
- [ ] Mobile navigation drawer

Shell blocks wait for the necessary navigation and overlay primitives to be
stable in Topcoat.

### 3. Dashboard and data

- [ ] Summary-card grid
- [ ] Filter and search toolbar
- [ ] Paginated data table
- [ ] Empty, loading, and error states

Data blocks must accept host-owned rows and actions. They will not choose a
database, charting library, or client-side state framework.

### 4. Product flows

- [ ] Settings form
- [ ] Profile form
- [ ] Invite-members form
- [ ] Destructive confirmation flow

## Selection rule

A block moves into active development when its primitive dependencies exist,
its server-side contract can be stated without application-specific policy,
and there is a real consuming repository ready to exercise it. One accepted
block is more valuable than a large ticket inventory.
