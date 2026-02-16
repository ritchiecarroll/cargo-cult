//! # cargo-cult
//!
//! A cargo subcommand that simulates the experience of copying code from
//! Stack Overflow without understanding it.
//!
//! Named after the anthropological phenomenon where Pacific island societies
//! built replica runways and wooden radios after WWII, hoping to summon
//! cargo planes—mimicking the form without understanding the mechanism.
//!
//! NOTE: This is a joke crate. It does not actually modify your project.

use clap::{Parser, Subcommand};
use colored::*;
use rand::seq::SliceRandom;

/// Mysterious crates that someone on the internet said you need
const MYSTERY_CRATES: &[(&str, &str, &str)] = &[
    ("left-pad", "0.1.1", "Pad strings on the left. Essential."),
    ("is-odd", "0.1.0", "Check if a number is odd. Don't ask."),
    (
        "is-even",
        "0.1.0",
        "Check if a number is even. Pairs well with is-odd.",
    ),
    (
        "true",
        "0.1.0",
        "Returns true. For when you need certainty.",
    ),
    ("false", "0.1.0", "Returns false. The yin to true's yang."),
    ("noop", "1.0.0", "Does nothing. But does it well."),
    ("zalgo", "0.2.0", "H̷e̷ ̷c̷o̷m̷e̷s̷."),
    (
        "guid",
        "0.1.1",
        "Generate GUIDs. Every project needs GUIDs.",
    ),
    ("lolcat", "0.1.1", "Rainbow output. Makes errors prettier."),
    ("ferris-says", "0.3.1", "Let the crab speak your errors."),
    ("thiserror", "1.0.0", "For errors. Copied from a tutorial."),
    (
        "anyhow",
        "1.0.0",
        "For other errors. The tutorial used both.",
    ),
    (
        "tokio",
        "1.0.0",
        "Async runtime. The blog post said I need this.",
    ),
    (
        "serde",
        "1.0.0",
        "Serialization. Every Rust project has this, right?",
    ),
    ("regex", "1.0.0", "Now you have two problems."),
    (
        "lazy_static",
        "1.4.0",
        "For statics that are lazy. Like me.",
    ),
    (
        "once_cell",
        "1.0.0",
        "Like lazy_static but newer. Progress!",
    ),
    (
        "chrono",
        "0.4.0",
        "Time is hard. Let someone else handle it.",
    ),
    (
        "rand",
        "0.8.0",
        "Random numbers. For when determinism is too predictable.",
    ),
    (
        "itertools",
        "0.12.0",
        "More ways to iterate. You can never have enough.",
    ),
];

/// Wisdom from the ancients (Stack Overflow, circa 2019)
const WISDOM: &[&str] = &[
    "I don't know why this works, but it fixed my issue.",
    "Try adding this crate. It worked for me™.",
    "Have you tried adding more dependencies?",
    "This is the way.",
    "Works on my machine.",
    "The compiler suggested this, so it must be right.",
    "I copied this from production code at my last job.",
    "My tech lead said to always include this.",
    "The tutorial I followed 3 years ago used this.",
    "Trust the process.",
    "If it compiles, ship it.",
    "Cargo knows best.",
    "Don't question the dependency graph.",
    "A wise mass-downloader once said...",
    "The crates.io download count doesn't lie.",
];

/// Reasons to add dependencies
const REASONS: &[&str] = &[
    "Someone on Reddit mentioned it.",
    "It has a lot of GitHub stars.",
    "The name sounds professional.",
    "My coworker uses it.",
    "It was in the top 10 on crates.io.",
    "The logo looked cool.",
    "I saw it in a conference talk.",
    "ChatGPT suggested it.",
    "It's what the cool kids use.",
    "FOMO.",
    "Job security through obscurity.",
    "It might be useful someday.",
    "Better safe than sorry.",
    "The documentation had nice formatting.",
    "Pure vibes.",
];

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    Cult(CultArgs),
}

#[derive(Parser)]
#[command(author, version, about = "Add dependencies without understanding why")]
struct CultArgs {
    #[command(subcommand)]
    command: Option<CultCommand>,
}

#[derive(Subcommand)]
enum CultCommand {
    /// Explain cargo cult programming
    Explain,
    /// List all mystery crates in the sacred texts
    List,
    /// Perform the ritual (simulate adding a random dependency)
    Ritual {
        /// Number of dependencies to invoke
        #[arg(short, long, default_value = "1")]
        count: usize,
    },
}

fn main() {
    let Cargo::Cult(args) = Cargo::parse();

    match args.command {
        Some(CultCommand::Explain) => explain(),
        Some(CultCommand::List) => list_crates(),
        Some(CultCommand::Ritual { count }) => perform_ritual(count),
        None => perform_ritual(1),
    }
}

fn explain() {
    println!("{}", "═".repeat(70).cyan());
    println!(
        "{}",
        "                     CARGO CULT PROGRAMMING"
            .bold()
            .yellow()
    );
    println!("{}", "═".repeat(70).cyan());
    println!();
    println!(
        "{}",
        "After World War II, some Pacific island societies that had seen".white()
    );
    println!(
        "{}",
        "military cargo planes deliver supplies built replica runways,".white()
    );
    println!(
        "{}",
        "wooden 'radios,' and straw planes, hoping to summon the cargo back.".white()
    );
    println!();
    println!(
        "{}",
        "They mimicked the FORM without understanding the MECHANISM."
            .bold()
            .white()
    );
    println!();
    println!("{}", "In programming:".cyan());
    println!(
        "{}",
        "  • Copying code without understanding why it works".white()
    );
    println!(
        "{}",
        "  • Adding dependencies because 'everyone uses it'".white()
    );
    println!(
        "{}",
        "  • Replicating patterns from tutorials without context".white()
    );
    println!("{}", "  • 'It worked on Stack Overflow'".white());
    println!();
    println!("{}", "Classic symptoms:".red());
    println!(
        "{}",
        "  • try/catch blocks copied everywhere 'just in case'".white()
    );
    println!("{}", "  • Thread.Sleep() to 'fix' race conditions".white());
    println!("{}", "  • CALL on every line of a batch file".white());
    println!("{}", "  • 500 dependencies for a hello world".white());
    println!();
    println!("{}", "Learn more:".green());
    println!(
        "{}",
        "  https://en.wikipedia.org/wiki/Cargo_cult_programming"
            .underline()
            .blue()
    );
    println!();
    println!("{}", "═".repeat(70).cyan());
}

fn list_crates() {
    println!("{}", "═".repeat(70).cyan());
    println!(
        "{}",
        "              THE SACRED DEPENDENCY TEXTS".bold().yellow()
    );
    println!("{}", "═".repeat(70).cyan());
    println!();

    for (name, version, desc) in MYSTERY_CRATES {
        println!(
            "  {} {} - {}",
            name.green().bold(),
            format!("v{}", version).dimmed(),
            desc.white()
        );
    }

    println!();
    println!("{}", "═".repeat(70).cyan());
    println!(
        "{}",
        "   May your dependency tree be ever in your favor."
            .italic()
            .yellow()
    );
    println!("{}", "═".repeat(70).cyan());
}

fn perform_ritual(count: usize) {
    let mut rng = rand::thread_rng();
    let count = count.min(MYSTERY_CRATES.len());

    println!();
    println!("{}", "🕯️  Initiating cargo cult ritual...".yellow().bold());
    println!();

    // Add some dramatic pauses via output
    let selected: Vec<_> = MYSTERY_CRATES.choose_multiple(&mut rng, count).collect();

    for (name, version, desc) in selected {
        let wisdom = WISDOM.choose(&mut rng).unwrap();
        let reason = REASONS.choose(&mut rng).unwrap();

        println!("{}", "─".repeat(50).dimmed());
        println!();
        println!(
            "   {} `{}` to dependencies...",
            "Adding".green().bold(),
            format!("{} = \"{}\"", name, version).cyan()
        );
        println!("   {} {}", "📖".to_string(), desc.italic().white());
        println!();
        println!("   {} \"{}\"", "💬".to_string(), wisdom.italic().yellow());
        println!("   {} {}", "🤷".to_string(), reason.dimmed());
        println!();
    }

    println!("{}", "─".repeat(50).dimmed());
    println!();

    // The important disclaimer
    println!(
        "{}",
        "   ⚠️  DISCLAIMER: No actual dependencies were harmed."
            .red()
            .bold()
    );
    println!(
        "{}",
        "   This is a joke. Your Cargo.toml remains pristine.".dimmed()
    );
    println!();

    // Random closing wisdom
    let final_wisdom = [
        "May the borrow checker have mercy on your soul.",
        "Go forth and cargo build.",
        "Remember: if it compiles, it works.*\n   *Terms and conditions apply.",
        "The ritual is complete. The linker spirits are appeased.",
        "Your dependency graph grows stronger.",
        "Trust in the crate. The crate knows.",
    ];

    println!(
        "   {} {}",
        "🦀".to_string(),
        final_wisdom.choose(&mut rng).unwrap().italic().cyan()
    );
    println!();
}
