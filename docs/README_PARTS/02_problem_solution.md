# 🌍 Why Do We Exist?

Blogger could literally become a free, highly customizable LMS for teachers worldwide. Imagine if schools, colleges, and educators could build their own free Khan Academies with built-in spaced repetition software (that's the ultimate end goal).

We're experimenting with several LMS options and identity verification methods: Syncthing, Web3, whatnot, OAuth 2.0, Rauthy, you name it. We also take loose inspiration from [rebane2001/xikipedia](https://github.com/rebane2001/xikipedia) because we really want educational content to become decentralized skinner boxes.

🔗 **Experimental vault:** [MoribundInstitute/mor_lms_vault](https://github.com/MoribundInstitute/mor_lms_vault)

Google could also foster a symbiotic relationship with Blogger by generating significant revenue through integrated ads. While the GUI editor includes several advertising options, they are left off by default, as traditional banner ads often degrade the user experience. Ideally, Google would introduce a Patreon-style monetization platform for Blogger, or perhaps an opt-in system for LLM training to support their platform.

![I know there is good in you - Google meme](mor_blogger_dioxus_ui/assets/images/memes/I_Know_Theres_Good_In_You_Google.jpg)

*Star Wars: Return of the Jedi © Lucasfilm Ltd. Google logo ™ Google LLC. Used here as parody/meme. Repo code is MIT licensed third-party image, not covered by MIT.*

## The Problem

Editing a custom Blogger theme means wrestling with a monolithic, 3,000-line `template.xml` file. 

## ✨ The Solution

The MorBlogger GUI Theme Builder replaces the monolith with a component-driven pipeline. You work visually with structured modules in a desktop UI. When you are ready, the Rust engine safely compiles your palettes, typography, and modular CSS into a single bulletproof XML file, matching HTML pages, or a ZIP archive containing the whole lot, ready for upload.

The theming is also hype because we have Rust code that converts GTK4 SVGs into Data URIs. Then, with a bit of behind-the-scenes CSS magic, we can make all sorts of SVG whatnot appear on your blog. I think we could get it to work for these later too: [GNOME-Look SVG Themes](https://www.gnome-look.org/browse?cat=277&ord=rating).