## Architecture: Bring Your Own Frontend (BYOF)

MorBlogger splits the software into a reusable **Socket** (window chrome, docks, activity bar) and a swappable **Plug** (Blogger-specific preview, export, and site data). The theme engine stays in `mor_blogger_core`; both the Dioxus UI and `mbt` CLI call into it.

![BYOF crate boundaries](docs/diagrams/byof_crates.drawio.png)

### The Three Main Pieces

* **1. The Engine (`mor_blogger_core`):** Headless logic — reads settings, stitches CSS, validates XML. No buttons or windows.
* **2. The Visual Workspace (`mor_blogger_dioxus_ui`):** The desktop UI you click on. Socket-and-plug layout:
  * **Socket (`MorLayoutChrome`):** Main window, dock zones, activity bar, floating panels.
  * **Plug (`BloggerWorkspace`):** Blogger preview, export, and workbench views. Forking for another host (e.g. Neocities) means swapping this plug, not rebuilding the shell.
* **3. The Command Line (`mor_blogger_cli` / `mbt`):** Terminal workflow for init, check, render, and bundle.

![Socket and plug UI pattern](docs/diagrams/socket_plug_ui.drawio.png)

### Socket & Plug in Plain English

| Piece | What it is | Key types |
| --- | --- | --- |
| **Socket** | Generic editor shell — tabs, docks, popups | `MorLayoutChrome`, `DockZone`, `ActivityBar` |
| **Plug** | Domain brains — what you are actually editing | `BloggerWorkspace` today; `NeocitiesWorkspace` is the fork example in the diagram |
| **Promise** | Swap the plug, keep the socket | Fixes in Blogger logic don't require rewriting dock chrome |

Deep dive: [Architecture Overview](docs/ARCHITECTURE.md)