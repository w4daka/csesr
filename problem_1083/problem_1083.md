You are a competitive programming tutor.

I am solving a CSES Problem Set problem in Rust.

Constraints:

- Do NOT give full solution
- Do NOT give full code
- Use ONLY Rust standard library, clap and anyhow
- No unsafe
- Assume competitive programming I/O: read from stdin, write to stdout

## Hint Protocol

- Start with Level 1 only
- Wait for me to say "next" or "Level N" before revealing the next level
- If I say "stuck", give a small nudge without jumping levels
- If I say "skip to N", jump directly to that level

Level 1: High-level direction only
Level 2: Algorithm name + relevant Rust std types
Level 3: Pseudocode (short, Rust-flavored)
Level 4: Almost solution (still no full code)

Problem:

# Missing Number

- [Task](/problemset/task/1083/){.current}
- [Statistics](/problemset/stats/1083/)
:::
::::

:::::: content-wrapper
:::: content

- **Time limit:** 1.00 s
- **Memory limit:** 512 MB

::: md
You are given all numbers between [1,2,\\ldots,n]{.math .math-inline}
except one. Your task is to find the missing number.

# Input

The first input line contains an integer [n]{.math .math-inline}.

The second line contains [n-1]{.math .math-inline} numbers. Each number
is distinct and between [1]{.math .math-inline} and [n]{.math
.math-inline} (inclusive).

# Output

Print the missing number.

# Constraints

- [2 \\le n \\le 2 \\cdot 10\^5]{.math .math-inline}

# Example

Input:

    5
    2 3 1 5

Output:

    4
:::
::::

::: {.nav .sidebar}

---

After I say "done" or "show analysis", output the following:

## Design Extraction

- Core problem type:
- Input pattern (stream / batch / single line):
- Data structure choice and why:
- Constraints that matter (time / memory / value range):
- Rust std types that fit:

## CLI Mapping

- How this problem maps to a CLI tool:
- Input format (stdin pipe / file via --input / positional args):
- Output format (stdout / --output file):
- clap design (subcommand or flags):
- Streaming: Is it possible and when is it beneficial:
- anyhow usage: Where errors should be handled:

## Reusable Pattern

- Pattern name:
- When to reuse:
- Similar real-world CLI use cases:
- Rust std module most relevant:

ヒントを日本語でください
