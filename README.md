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

### Day 2

- I was tired by the end of day 1 so a little recap might be in order.
- They have `cargo doc --open` for exactly the things you use? Neat!
- Surprise, surprise! It [doesn't work with WSL](https://github.com/rust-lang/cargo/issues/7557).
- They purposefully led you to an error to talk about errors, what to expect and how to read them!
- Errors and their handling are a feature.
- Not a big fan of shortening type names, but I guess I'll get used to it.
- They let you shadow variable names of different types. That's simultaneously super neat and super scary.
- There seems to be a tendency to fail at the call itself with `expect`, would be interesting to see how they'd teach cleaner error handling later.
- I keep feeling this trend of "show them a bad easy way to do it, later teach them a cleaner way", I suspect some people would google stuff, land here, read the bad examples in isolation and do that because "the book told them to".
- I didn't make a separate directory for the guessing game last time, I'll do that now and just remake the game.
- First try 😎
  ```
  Guessing game!!
  Guess the number:
  10
  Got em!
  ```
- I had to play again to confirm I hadn't written bad code. 😅
- So we we `loop`, but isn't this infinite? How do we break out of the loop though?
- Ha! with `break`;
- Error handling is also through the pattern `match`, aaaand match is an expression!
- `_` is to catch all error values, does this mean I can match multiple types of errors based on value/type? 😲
- Gotta go work!

## Day 3

- The flow they've gone with so far:
  - unavoidable install step
  - basic example for first easy win
  - syntax and structure early
  - tooling next
  - break down an involved example into incremental wins and buy in.
  - use each increment to intro more concepts.
  - teach through mistakes.
  - errors are a feature/tool to teach.
  - Then start with the actual basic concepts.
- The first thing they go back to is variables, immutability and constants.
- The book feels like it is written for people who know how to code and might get tripped up by some concepts, so they keep going back to those to help! :)
- The example of shadowing also introduces the idea of how you can use shadowing to do more ops on a variable but still get an immutable result.
- I miss real static typing 😭
- Overflow panic in dev, not in release. I wonder why they'd do that?
  - They call out behaviour that stands out of the ordinary frequently.
- They frequently reference the specific standard they conform to, it helps set the right expectations and build a mental model around the language.
- single quote for `char`, double quote for `string`
- 4 byte chars!!! `'😻'`
- Both tuples and arrays. They went with a very broad set of language constructs.
  - Rust caters very squarely to the "power user".
- Time to freshen up for work!

## Day 4

- Are rust vectors like slices from go?
- `snake_case` functions, throwback to [python days](https://www.python.org/dev/peps/pep-0008/#function-and-variable-names)!
- Everything is not an expression, interesting!
- Block statements can be used as expressions! 😲
- Last line of a block statement without a semi-colon acts like the return of a function expression? Uggh.
  - They introduced this before they introduced the `return` expression.
- `let x = if condition {5} else {6};`
- `loop`, `while` and `for`, why?
  - break out of a loop with a `break` and add an expression after it to act as a return for the block expressions. 😲
  - while is when you want conditions with your loop, but there's break for it too.
  - for is when you want to iterate over a collection.
- Collections have methods on them like `.iter` and `.rev`
- Off to work!

## Day 5

- Long break since the last one!
- Tried to do a recap from before, found a fun gotcha. Guess what happens when you run this?
  ```rust
  let x = 5;
  let y = loop {
    let x = x + 1;
    if x > 10 {
      break x * 2
    }
  };
  println!("x = {}, y = {}", x, y);
  ```
  [See for yourself!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e13a270788e76005aed27a4680a7f64f)
  This is because the x in the third line is still the same x from outside. Shadowing can get confusing, beware :)
- Their description of stack and heap is one of the best ones I've seen in a while.
- Single owner principle might just be the biggest thing Rust has made mainstream.
- And dropping values when owner goes out of scope. _chef's kiss_
- string on the stack, String on the heap.
- Difference of behaviour of same operation between types on the stack and types that point to the heap is an implementation detail leak. 👎
  - Would it be better to force the same behaviour?
  - I don't understand why they couldn't use a ref counter instead of ref counting?
  - Why copy and invalidate when you can pretty much rename?
  - They must have had some good reason. Todo: find out later
- Borrow and give back is a nice physical world thing to add here :)
- Good first day after a long hiatus from this. Off to read sth else!
