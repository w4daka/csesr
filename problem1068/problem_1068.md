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

# Weird Algorithm

- [Task](/problemset/task/1068/){.current}
- [Statistics](/problemset/stats/1068/)
:::
::::

:::::: content-wrapper
:::: content

- **Time limit:** 1.00 s
- **Memory limit:** 512 MB

::: md
Consider an algorithm that takes as input a positive integer [n]{.math
.math-inline}. If [n]{.math .math-inline} is even, the algorithm divides
it by two, and if [n]{.math .math-inline} is odd, the algorithm
multiplies it by three and adds one. The algorithm repeats this, until
[n]{.math .math-inline} is one. For example, the sequence for
[n=3]{.math .math-inline} is as follows: [ 3 \\rightarrow 10
\\rightarrow 5 \\rightarrow 16 \\rightarrow 8 \\rightarrow 4
\\rightarrow 2 \\rightarrow 1]{.math .math-display} Your task is to
simulate the execution of the algorithm for a given value of [n]{.math
.math-inline}.

# Input

The only input line contains an integer [n]{.math .math-inline}.

# Output

Print a line that contains all values of [n]{.math .math-inline} during
the algorithm.

# Constraints

- [1 \\le n \\le 10\^6]{.math .math-inline}

# Example

Input:

    3

Output:

    3 10 5 16 8 4 2 1
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
