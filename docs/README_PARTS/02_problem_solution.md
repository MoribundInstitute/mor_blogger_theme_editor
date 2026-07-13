# 🌍 Why Do We Exist?

We want to make Blogger GUI editing software so easy that even 95-year-old, technologically illiterate grandmothers could use it. I'm thinking we should lean into [skeuomorphism](https://en.wikipedia.org/wiki/Skeuomorph) to maximize the software's intuitiveness. Blogger could be an ideal platform for older people because the content management system is already fairly intuitive, while we could help them make their blogs look good.

Those of us who are more technically inclined should be willing to sacrifice some of our time creating presets, widgets, & other tools (and post them on the various compendiums this GUI software will manage in a similar fashion to [RuneLite plugins](https://github.com/runelite/plugin-hub)) so that people who may be technologically illiterate, but have other interesting hobbies, such as maintaining an orchard of [heritage apples](https://en.wikipedia.org/wiki/Lost_Apple_Project) or translating [Beowulf](https://en.wikipedia.org/wiki/Beowulf) into [Old Scots](https://en.wikipedia.org/wiki/Early_Scots), can more easily manage a blog without worrying about subscription fees and similar obstacles.

Blogger could also become a free, highly customizable learning management system (LMS) for teachers worldwide. Imagine schools, colleges, and independent educators being able to build their own free versions of Khan Academy, complete with built-in spaced repetition software. That's the ultimate end goal.

We're experimenting with several LMS options and identity verification methods — Syncthing, OAuth 2.0, Rauthy, and other approaches — in the experimental vault below. We also take loose inspiration from [rebane2001/xikipedia](https://github.com/rebane2001/xikipedia) because we want educational content to stay portable and community-owned rather than locked inside one platform.

🔗 **Experimental vault:** [MoribundInstitute/mor_lms_vault](https://github.com/MoribundInstitute/mor_lms_vault)

Google could also foster a symbiotic relationship with Blogger by generating significant revenue through integrated ads. While the GUI editor includes several advertising options, they are left off by default, as traditional banner ads often degrade the user experience. Ideally, Google would introduce a Patreon-style monetization platform for Blogger, or perhaps an opt-in system for LLM training to support their platform.

![I know there is good in you - Google meme](mor_blogger_dioxus_ui/assets/images/memes/I_Know_Theres_Good_In_You_Google.jpg)

*Star Wars: Return of the Jedi © Lucasfilm Ltd. Google logo ™ Google LLC. Used here as parody/meme. Repo code is MIT licensed third-party image, not covered by MIT.*

## The Problem

Editing a custom Blogger theme means wrestling with a monolithic, 3,000-line `template.xml` file. 

## ✨ The Solution

The MorBlogger Theme Editor replaces the monolith with a component-driven pipeline. You work visually with structured modules in a desktop UI. When you are ready, the Rust engine safely compiles your palettes, typography, and modular CSS into a single bulletproof XML file, matching HTML pages, or a ZIP archive containing the whole lot, ready for upload.

![Monolith vs modular pipeline](docs/diagrams/monolith_vs_modular.drawio.png)

GTK4 desktop themes can supply more than colors: Rust code converts bundled SVGs into data URIs, and CSS maps them onto Blogger-safe hooks so icons, decorations, and UI chrome can ship inside the theme without extra HTTP requests. GNOME-Look [icon/SVG theme packs](https://www.gnome-look.org/browse?cat=277&ord=rating) are on the roadmap for the same pipeline.