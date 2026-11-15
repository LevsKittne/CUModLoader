#![windows_subsystem = "windows"]

use installer::i18n::{fill, text};
use installer::*;

const TITLE: &str = "CU ModLoader Setup";

macro_rules! payload {
    ($($rel:literal),* $(,)?) => {
        &[$(($rel, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../dist/", $rel)) as &[u8])),*]
    };
}

const FILES: &[(&str, &[u8])] = payload! {
    "version.dll",
    "ModLoader/core/ModLoader.dll",
    "ModLoader/core/0Harmony.dll",
    "ModLoader/core/Mono.Cecil.dll",
    "ModLoader/core/Mono.Cecil.Mdb.dll",
    "ModLoader/core/Mono.Cecil.Pdb.dll",
    "ModLoader/core/Mono.Cecil.Rocks.dll",
    "ModLoader/core/MonoMod.RuntimeDetour.dll",
    "ModLoader/core/MonoMod.Utils.dll",
};

fn main() {
    let t = text();
    let Some((game, local)) = find_game() else {
        error(TITLE, t.not_found);
        return;
    };
    let shown = game.display().to_string();
    if !local && !ask(TITLE, &fill(t.ask_loader, &[&shown])) {
        return;
    }
    let update = game.join("ModLoader").join("core").join("ModLoader.dll").is_file();
    let result = write_all(&game, FILES).and_then(|_| std::fs::create_dir_all(game.join("ModLoader").join("mods")).map_err(|e| e.to_string()));
    if let Err(e) = result {
        error(TITLE, &fill(t.write_failed, &[&e]));
        return;
    }
    info(TITLE, &fill(t.loader_done, &[if update { t.updated } else { t.installed }, &shown]));
    self_delete();
}
