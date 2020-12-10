# kenkda

An experiment in NUX of a language.

## Motivation

- I wanted to be a new user in something outside of the event loop world and document my experience.
- Rust is said to have a long learning curve, I can learn a lot from the way they introduce concepts.
- Rust compiles well to WASM. Playing with that will give me a chance to intuit out how far we are from making large apps outside of JS/TS.

## Notes

I will continue this till I either learn the language or I get bored and drop it.

### Day 1

- Rust book is a good idea.
- I really wish I could keep rustbook open in vs code.
- They dedicated a complete page to hello world!
- They've used the first page after install to introduce
  - langauage structure
  - language syntax
  - tooling
  - compiling and running
  - macros
- Interesting that they explained "compiling is a separate step", are they targetting interpreted language folks?
- Explaining the biggest conceptual difference early is a good idea!
- Second page is entirely about cargo!
- Interesting that they put scaffolding after a manual hello world example. Was that deliberate?
- Am I supposed to gitignore the target folder? (ans: yes) (Probably when they know we're in a git repo, they should have a .gitignore file in the scaffolding? 🤷)
- Compile is slow enough that you need a separate `cargo check`!
- `include` / `import` is `use` now, not sure what the difference is, is there any? They side-stepped import/include so many times with `bring into scope` lol.
- Behold the `prelude`! I'd most likely only use a couple of imports in 1 file, they _really_ don't want you to have imports?
- 🤯 You declare variables which are mutable and also pass them around as mutable explicitly 🤯
- `use` is not like import because you can literally use it without the use! Just do a `std::io::stdin`.
  Now machine can answer what packages this file depends on, but people can't. Do we actually ask that question a lot?
  And yes, they do not want you to ~import~ `use` a lot I guess.
- language convention - lowercase for function, capitalise for type. `std::io::stdin` returns instance of `std::io::Stidin`
- 🤯 `&mut` is exactly what I wanted in life! Most languages copy to gain immutability, (some deepfreeze). This is SO AWESOME.
- Does this mean I can pass `&mut guess` to one thread and do a `&guess` on other threads and be sure I am safe? 😲
- I need to take a break to let this sink in...
- Screw breaks, give me MOAR!
- Subtly introducing good code style, nice! `rustfmt` will be good.
- Is everything a `Result`?
- These enums don't look like the enums we know from other languages!
- I can understand how `.expect` on `io::Result` would know about the enum, but how does the compiler know that `Err` on `Result` is something that needs to trigger a warning?
- The language doesn't give a `rand` in `stdlib`? That's interesting.
- Wow the crates VSCode extension is really good! Helps keep deps updated. Someone should do this for `package.json`
- Central package registry and lock files! :)
- Remember: cargo update keeps you in the minor version. The extension makes so much more sense now!
- They have traits too? Is there something called too many concepts in 1 sitting? No... ofc not.
- Why does autocomplete not trigger for a few things in VSCode. Is there a lot of language magic or does the extension suck?
- And there's this thing called `match` too. This language is heavy on enum and pattern matching, interesting!
- Ok I really need to take a break now.
