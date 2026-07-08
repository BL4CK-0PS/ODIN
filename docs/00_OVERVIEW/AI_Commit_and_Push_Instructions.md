# AI Commit & Push Instructions (ODIN)

## Objective

Whenever you complete a meaningful unit of work:

1.  Verify the project builds.
2.  Run tests (if available).
3.  Review the diff.
4.  Create a clear commit.
5.  Push to the current feature branch.

------------------------------------------------------------------------

## Before Every Commit

``` bash
git status
git diff
```

Ensure: - No secrets or API keys - No generated artifacts unless
intentionally tracked - No debug code or temporary files - Code is
formatted

------------------------------------------------------------------------

## Commit Message Format

    <type>: <short summary>

    Examples

    feat: add adaptive interview engine
    fix: resolve transcript synchronization bug
    refactor: simplify evaluation pipeline
    docs: update architecture
    test: add evaluator unit tests
    style: format frontend components
    perf: optimize JD parser
    chore: update dependencies

------------------------------------------------------------------------

## Commit Workflow

``` bash
git add .

git status

git commit -m "feat: add adaptive interview orchestration"

git push origin <branch-name>
```

------------------------------------------------------------------------

## Large Features

Break work into small commits.

Example:

    feat: create interview session model

    feat: implement orchestrator

    feat: integrate evaluation engine

    feat: add report generation

    docs: update architecture

------------------------------------------------------------------------

## Never Commit

-   `.env`
-   API keys
-   Tokens
-   Passwords
-   Large datasets
-   Build artifacts
-   Temporary files

------------------------------------------------------------------------

## Before Pushing

``` bash
git status
git log --oneline -5
```

Confirm: - Working tree clean - Commit messages are meaningful - Correct
branch

------------------------------------------------------------------------

## Emergency Rollback

Undo last commit (keep changes):

``` bash
git reset --soft HEAD~1
```

Discard last commit:

``` bash
git reset --hard HEAD~1
```

------------------------------------------------------------------------

## AI Operating Rules

-   Never commit broken code.
-   Never combine unrelated changes.
-   Keep commits atomic.
-   Update documentation when architecture changes.
-   If behavior changes, update tests.
-   Explain *why* in commit bodies for non-trivial changes.

------------------------------------------------------------------------

## Recommended Workflow

``` text
Plan
 ↓
Implement
 ↓
Build
 ↓
Test
 ↓
Review Diff
 ↓
Commit
 ↓
Push
 ↓
Open PR / Continue
```
