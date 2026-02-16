# cargo-cult 🕯️🦀

> *Add dependencies without understanding why.*

A cargo subcommand that simulates the experience of copying code from Stack Overflow without understanding it.

## What is Cargo Cult Programming?

After World War II, some Pacific island societies that had seen military cargo planes deliver supplies built replica runways, wooden "radios," and straw planes, hoping to summon the cargo back.

**They mimicked the *form* without understanding the *mechanism*.**

In programming, this manifests as:
- Copying code without understanding why it works
- Adding dependencies because "everyone uses it"  
- Replicating patterns from tutorials without context
- "It worked on Stack Overflow"

## Installation

```bash
cargo install cargo-cult
```

## Usage

```bash
# Perform the ritual (simulate adding a random dependency)
cargo cult

# Add multiple mystery dependencies
cargo cult ritual --count 3

# Explain cargo cult programming
cargo cult explain

# List all dependencies in the sacred texts
cargo cult list
```

## Example Output

```
🕯️  Initiating cargo cult ritual...

──────────────────────────────────────────────────

   Adding `left-pad = "0.1.1"` to dependencies...
   📖 Pad strings on the left. Essential.

   💬 "I don't know why this works, but it fixed my issue."
   🤷 Someone on Reddit mentioned it.

──────────────────────────────────────────────────

   ⚠️  DISCLAIMER: No actual dependencies were harmed.
   This is a joke. Your Cargo.toml remains pristine.

   🦀 May the borrow checker have mercy on your soul.
```

## Important Disclaimer

**This crate does not actually modify your project.** It's a harmless joke that prints funny messages. Your `Cargo.toml` will remain untouched, your dependency tree unmolested, and your sanity (relatively) intact.

## Why?

Because the Rust community has a sense of humor, and sometimes we need a reminder that blindly copying code from the internet isn't always the answer.

Also, it's fun.

## See Also

- [cargo-mommy](https://crates.io/crates/cargo-mommy) - Mommy's here to support you when running cargo~ ❤️
- [Wikipedia: Cargo cult programming](https://en.wikipedia.org/wiki/Cargo_cult_programming)

## License

- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

Contributions welcome! Feel free to add more mystery crates, wisdom quotes, or reasons to add dependencies. PRs that _actually_ modify `Cargo.toml` will be rejected on philosophical grounds.

---

*"I copied this README from a template I found online. I don't know what half of it means."*

## Attribution
_I cannot take credit for this idea: in a session with Claude where I was asking about some code I was puzzled about, he suggested it might be "cargo culting"_:

>[`Ritchie:`] OK, I have to ask, what is "cargo culting", a Rust reference?

>[`Claude:`] > Ha! No, predates Rust by decades—though that's a delightful coincidence given your Rust learning journey. Cargo cult programming refers to copying code or practices without understanding why they work, just hoping they'll magically produce the desired result. It comes from the anthropological term "cargo cult"—after WWII, some Pacific island societies that had seen military cargo planes deliver supplies built replica runways, wooden "radios," and straw planes, hoping to summon the cargo back. They mimicked the form without understanding the mechanism. So _no_, not a Rust thing—but now I kind of want `cargo cult` to be a real Rust subcommand that just pretends to copy random dependencies into your `Cargo.toml`.

So here we are ;-), have fun...