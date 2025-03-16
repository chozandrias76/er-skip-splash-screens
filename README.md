<p align="center" style="margin-bottom: 5rem">
  <a href="./readme.md">
    <img alt="Temporary AI generated logo" src="./logo.webp" alt="ER Skip Startup Cutscenes Logo" width="250" style="border-radius: 10px; border: 2px solid black;">
    </br>
    <sub>Temporary AI generated logo</sub>
  </a>
  
</p>

# ER Skip Startup Cutscene Skipper

## Description
Simple DLL to replace runtime execution of function body which shows startup cutscenes with an early return.

## Users

### Using
1. This DLL requires that you have your own form for injecting the DLL into Elden Ring. Popular solutions include [Lazy Loader](https://www.nexusmods.com/darksouls3/mods/677) for souls games generally and [Mod Loader](https://github.com/techiew/EldenRingModLoader). I suggest using Lazy Loader since this DLL does not require the await that ER ML provides.

1. Injecting this DLL while connected to official FromSoft servers is **NOT RECOMMENDED**.

1. The latest and safe-to-use DLL will always be found on the [Releases](https://github.com/chozandrias76/er-skip-startup-cutscenes/releases) page, and this page should be linked/back-linked to [my corresponding Nexus Mod author page](https://next.nexusmods.com/profile/xenos76/mods).

1. If your game crashes unexpectedly during startup when injecting the DLL, or if it otherwise isn't working as expected, check the log file that you will find within the directory that contains your ELDENRING.exe.

### Contributing
1. Think you found a new issue? Check [here](https://github.com/chozandrias76/er-skip-startup-cutscenes/issues?q=sort%3Aupdated-desc) 
1. Add an issue if one isn't closed for your problem, or if one isn't already open related to your problem.
1. It should be noted that this feature is considered complete, and may only receive updates for security reasons or in the event the game is updated and it breaks this implementation.

## Developers

### Building
1. Clone [this repository](/)
1. [Install Rust](https://rust-lang.github.io/rustup/installation/index.html)
1. Navigate to the directory and run: `$ cargo build --manifest-path er_skip_startup_cutscenes/Cargo.toml`
1. Your User ready DLL will be found within `.\er_skip_startup_cutscenes\target\debug\er_skip_startup_cutscenes.dll`

### Contributing
1. Pull requests will be reviewed from forks attempting to merge if they correspond to an issue.
1. Pull requests should not be expected to be merged or reviewed on regular basis