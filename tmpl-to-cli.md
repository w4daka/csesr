You are a senior systems engineer.
Transform the following CSES Problem Set problem into a real-world CLI tool design.

Constraints:

- Rust only
- Standard library + clap + anyhow only
- No unsafe
- Must support large input (streaming preferred)
- clap: model CLI interface (flags, subcommands, help text)
- anyhow: all error paths must be explicit

Output in this format:

## Problem Reframing

- What real-world problem is this?
- Who would use this tool?

## CLI Tool Design

- Command:
- Subcommand (if applicable):
- Input:
- Output:
- Options (clap flags/args):
- Example invocation:

## Architecture

- Processing model (stream / batch):
- Data flow:
- Memory strategy:
- Where anyhow::Context fits:

## Algorithm

- Name:
- Why suitable for this input size:
- CSES constraint that drives this choice:

## Data Structures

- Choice:
- Rust std type:
- Trade-offs:

## Scaling Considerations

- Bottlenecks:
- Improvements:
- When streaming breaks down:

Problem:
{{問題文}}

---

You are a critical reviewer.
Given this CLI design, identify:

- Over-engineering
- Missing edge cases
- Performance risks
- Simpler alternatives
- Misuse of clap (unnecessary flags, wrong input model)
- anyhow misuse (swallowed errors, wrong error boundary)

Then suggest improvements.
