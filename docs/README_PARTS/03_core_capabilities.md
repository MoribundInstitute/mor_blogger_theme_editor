## 🚀 Core Capabilities

### 🧩 Building with Blocks (Modular XML Assembly)
* **No More Giant Files:** Instead of wrestling with one massive 3,000-line template file, you build your theme using small, easy-to-manage blocks like a header, a sidebar, and a footer. 
* **Clean Code Assembly:** Behind the scenes, the engine takes all your individual style rules and stitches them together perfectly. It keeps the final code lean and prevents messy errors.
* **Automatic Wiring:** The software handles the boring stuff for you. It automatically connects search engine (SEO) settings, text sizes, and widget spaces without you having to write the code.

![Theme compile and export pipeline](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/theme_compile_pipeline.drawio.png)

![CSS assembly pipeline](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/css_assembly_pipeline.drawio.png)

See also: [CSS Assembly Pipeline](docs/CSS_PIPELINE.md)

### 🎨 Stealing Colors from Linux (GTK Desktop Integration)
* **Match Your Desktop:** If you use Linux, you can import color schemes and visual styles directly from popular desktop themes like Adwaita, Nord, or WhiteSur.
* **Built-in Graphics:** When you add icons or vector graphics (SVGs), the software converts them into text and embeds them directly into your theme code. This makes your blog load much faster because it doesn't have to ask another website to provide the images.

### 🖥️ The Visual Workspace (Fluid UI)
* **Lightweight Interface:** The app's interface is built on a fast, modern toolkit called Dioxus 0.7.
* **Flexible Window Borders:** The app's window borders magically adapt to look perfect whether you are on Windows, a Mac, or using a specialized, keyboard-only Linux setup.
* **Smart Code Locator:** When you click on a visual part of your theme (like the header), the code editor instantly scrolls to and highlights the exact line of code you need to change. 
* **Instant Updates:** The program runs on Rust, meaning your live previews and edits update instantaneously without any annoying loading screens.

![Workspace UI layout](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/workspace_ui_layout.drawio.png)

![Dioxus state, docks, and ThemeSignals](https://raw.githubusercontent.com/MoribundMurdoch/mor_blogger_theme_editor_atlas/main/diagrams/shared/dioxus_app_architecture.drawio.png)