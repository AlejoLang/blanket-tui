# blanket-tui

[Blanket](https://github.com/rafaelmardojai/blanket) but in a TUI.

Using [ratatui](https://ratatui.rs/).

## Sounds

See [CREDITS](./resources/CREDITS.md)

## Controls

### General

|     Key     |              Action               |
| :---------: | :-------------------------------: |
|     `n`     |             Add sound             |
|     `d`     |       Delete selected sound       |
|   `Enter`   |       Play/Pause all sounds       |
|     `+`     | Increase general volume by `0.05` |
|     `-`     | Decrease general volume by `0.05` |
| `q` / `Esc` |               Quit                |

### Sound controls

|   Key   |                  Action                  |
| :-----: | :--------------------------------------: |
|  `Up`   |             Focus top sound              |
| `Down`  |            Focus bottom sound            |
| `Left`  | Decrease selected sound volume by `0.05` |
| `Right` | Increase selected sound volume by `0.05` |
| `Space` |              Activate sound              |

### Add sound popup

|   Key   |        Action         |
| :-----: | :-------------------: |
|  `Tab`  | Switch selected input |
|  `Esc`  |      Quit popup       |
| `Enter` |      Save sound       |

**In order to save a sound all input fields must be filled**

