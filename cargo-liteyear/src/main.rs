use std::process::{self, Command};
use std::thread;

use displaydoc::Display;
use eyre::eyre;
use liteyear::{BenchParam, Benchmark};
use thiserror::Error;

type Result<T> = color_eyre::Result<T>;

#[tokio::main]
async fn main() -> crate::Result<()> {
    // check that the directory is a crate.
    let dir = async {
        let dir = std::env::current_dir()?;
        if !dir.join("Cargo.toml").exists() {
            return Err(eyre!("not a crate"));
        }
        Ok(dir)
    };

    let cargo_expand = async {
        let out = Command::new("cargo")
            .arg("expand")
            .output()
            .map_err(|e| eyre!("failed to run cargo expand: {}", e))?;
        if !out.status.success() {
            return Err(eyre!("cargo expand failed"));
        }
        Ok(out.stdout)
    };

    let cargo_expand_worker = thread::spawn(|| {
        Command::new("cargo")
            .arg("expand")
            .output()
            .map(|out| out.stdout)
    });


    // let functions = todo!();
    // let benches = functions.map(|Func { item }| Benchmark {
    //     name: item.sig.ident.to_string(),
    //     fn_path: item.sig.ident.to_string(),
    //     args: item
    //         .sig
    //         .inputs
    //         .iter()
    //         .filter_map(|arg| match arg {
    //             syn::FnArg::Receiver(_) => None,
    //             syn::FnArg::Typed(pat) => {
    //                 let name = match &*pat.pat {
    //                     syn::Pat::Ident(ident) => ident.ident.to_string(),
    //                     _ => todo!(),
    //                 };
    //                 let ty = match &*pat.ty {
    //                     syn::Type::Path(path) => {
    //                         path.path.segments.last().unwrap().ident.to_string()
    //                     }
    //                     _ => todo!(),
    //                 };
    //                 let val = todo!();
    //                 Some(BenchParam { name, ty, val })
    //             }
    //         })
    //         .collect(),
    // });

    // if !out.status.success() {
    //     process::exit(1);
    // }

    // let cargo_expand_worker =
    //     thread::spawn(|| match Command::new("cargo").arg("expand").output() {
    //         Ok(out) if out.status.success() => Ok(out.stdout),
    //     });

    Ok(())
}
