# Inaba

Inaba is a shell built in Amane.

This is my own personal shell, if you wanna add something new then fork it yourself :)

# Included

- Bar
- App launcher
- Notification daemon
- Lock screen
- Haha funny Activate Linux widget

This is my first shell, so it's not amazing, but hey I made it :)

# Installation

Clone the repo:
`git clone https://github.com/imACanOfDrPepper/inaba ~/.config/amane`

Build the shell and launch it:

```
amane compile
amane run
```

`amane run` only starts the compiled shell, so run `amane compile` again after pulling changes. While editing the config, `amane dev` rebuilds and restarts it on every save.

Or start it with your compositor (Hyprland here):

```
hl.on("hyprland.start", function() hl.exec_cmd("amane run") end)
```

# Dependencies

- amane
- A compositor of your choice, Hyprland for me
- JetBrains Mono Font

# IPC

Everything is driven through `amane ipc call`, so it is easy to bind:

`hl.bind(mainMod .. " + " .. "SPACE", hl.dsp.exec_cmd("amane ipc call toggle-launcher"))`

<details>
<summary>All IPC targets</summary>

```
amane ipc call toggle-launcher
amane ipc call lock-screen
amane ipc call clear-notifs
```

</details>

# Notes

This is my config that I daily-drive. Expect random features that are there because I wanted them and occasional breakage.
Also modify this to your heart's content :)
