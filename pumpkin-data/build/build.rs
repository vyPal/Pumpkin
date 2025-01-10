use std::{env, fs, path::Path, process::Command};

use proc_macro2::{Span, TokenStream};
use syn::Ident;

mod chunk_status;
mod packet;
mod particle;
mod screen;
mod sound;

pub fn main() {
    write_generated_file(packet::build(), "packet.rs");
    write_generated_file(screen::build(), "screen.rs");
    write_generated_file(particle::build(), "particle.rs");
    write_generated_file(sound::build(), "sound.rs");
    write_generated_file(chunk_status::build(), "chunk_status.rs");
}

pub fn write_generated_file(content: TokenStream, out_file: &str) {
    let out_dir = env::var_os("OUT_DIR").expect("failed to get OUT_DIR env var");
    let path = Path::new(&out_dir).join(out_file);
    let code = content.to_string();

    fs::write(&path, code).expect("Faile to write to fs");

    // Try to format the output for debugging purposes.
    // Doesn't matter if rustfmt is unavailable.
    let _ = Command::new("rustfmt").arg(path).output();
}

pub fn ident<I: AsRef<str>>(s: I) -> Ident {
    let s = s.as_ref().trim();

    // Parse the ident from a str. If the string is a Rust keyword, stick an
    // underscore in front.
    syn::parse_str::<Ident>(s)
        .unwrap_or_else(|_| Ident::new(format!("_{s}").as_str(), Span::call_site()))
}
