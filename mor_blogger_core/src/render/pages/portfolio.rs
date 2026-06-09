pub fn generate_portfolio_html<T>(_config: &T) -> String {
    r##"
    <style>
        .mor-portfolio-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            color: var(--theme-text, #cccccc);
            font-family: var(--theme-font, system-ui, sans-serif);
            display: flex;
            flex-direction: column;
            gap: 40px;
        }

        .mor-portfolio-header {
            text-align: center;
            padding: 40px 0 20px 0;
            border-bottom: 1px solid var(--theme-border-light, #333);
        }

        .mor-portfolio-title {
            font-size: 3rem;
            margin: 0;
            color: var(--theme-accent, #60cdff);
            letter-spacing: -1px;
        }

        .mor-portfolio-subtitle {
            font-size: 1.2rem;
            opacity: 0.8;
            max-width: 600px;
            margin: 16px auto 0 auto;
            line-height: 1.6;
        }

        .mor-portfolio-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
            gap: 30px;
        }

        .mor-project-card {
            background: var(--theme-bg-panel, #252525);
            border: 1px solid var(--theme-border, #333);
            border-radius: 8px;
            overflow: hidden;
            transition: transform 0.2s ease, box-shadow 0.2s ease;
            display: flex;
            flex-direction: column;
            text-decoration: none;
            color: inherit;
        }

        .mor-project-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 12px 30px rgba(0,0,0,0.5);
            border-color: var(--theme-accent, #60cdff);
        }

        .mor-project-image {
            width: 100%;
            height: 200px;
            background-color: var(--theme-bg-elevated, #1e1e1e);
            border-bottom: 1px solid var(--theme-border, #333);
            background-size: cover;
            background-position: center;
            position: relative;
        }

        .mor-project-image.placeholder::after {
            content: '</>';
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            font-size: 3rem;
            color: var(--theme-border-light, #444);
            font-family: monospace;
            font-weight: bold;
        }

        .mor-project-content {
            padding: 24px;
            display: flex;
            flex-direction: column;
            flex-grow: 1;
        }

        .mor-project-tags {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin-bottom: 12px;
        }

        .mor-tag {
            font-size: 0.7rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            padding: 4px 8px;
            background: rgba(96, 205, 255, 0.1);
            color: var(--theme-accent, #60cdff);
            border-radius: 4px;
        }

        .mor-project-title {
            margin: 0 0 12px 0;
            font-size: 1.5rem;
        }

        .mor-project-desc {
            margin: 0 0 24px 0;
            font-size: 0.95rem;
            line-height: 1.6;
            opacity: 0.8;
            flex-grow: 1;
        }

        .mor-project-links {
            display: flex;
            gap: 16px;
            margin-top: auto;
        }

        .mor-project-link {
            text-decoration: none;
            font-size: 0.9rem;
            font-weight: bold;
            color: var(--theme-text, #fff);
            border-bottom: 2px solid var(--theme-accent, #60cdff);
            padding-bottom: 2px;
            transition: color 0.1s ease;
        }

        .mor-project-card:hover .mor-project-link {
            color: var(--theme-accent, #60cdff);
        }

        @media (max-width: 768px) {
            .mor-portfolio-title { font-size: 2.2rem; }
            .mor-portfolio-grid { grid-template-columns: 1fr; }
        }
    </style>

    <div class="mor-portfolio-container">
        <header class="mor-portfolio-header">
            <h1 class="mor-portfolio-title">Selected Works</h1>
            <p class="mor-portfolio-subtitle">A collection of open-source projects, design explorations, and technical write-ups.</p>
        </header>

        <div class="mor-portfolio-grid">
            
            <a href="#" class="mor-project-card">
                <div class="mor-project-image placeholder"></div>
                <div class="mor-project-content">
                    <div class="mor-project-tags">
                        <span class="mor-tag">Rust</span>
                        <span class="mor-tag">Dioxus</span>
                    </div>
                    <h2 class="mor-project-title">Moribund Architect</h2>
                    <p class="mor-project-desc">A desktop GUI theme engine that compiles raw layout settings into fully functional, modular Blogger XML templates.</p>
                    <div class="mor-project-links">
                        <span class="mor-project-link">View Case Study &rarr;</span>
                    </div>
                </div>
            </a>

            <a href="#" class="mor-project-card">
                <div class="mor-project-image" style="background-image: url('https://images.unsplash.com/photo-1550751827-4bd374c3f58b?auto=format&fit=crop&q=80&w=600');"></div>
                <div class="mor-project-content">
                    <div class="mor-project-tags">
                        <span class="mor-tag">UI/UX</span>
                        <span class="mor-tag">CSS</span>
                    </div>
                    <h2 class="mor-project-title">The Dewey Layout</h2>
                    <p class="mor-project-desc">A retro, high-density dashboard inspired by 90s library indexing systems designed for modern web applications.</p>
                    <div class="mor-project-links">
                        <span class="mor-project-link">Live Demo &rarr;</span>
                    </div>
                </div>
            </a>

            <a href="#" class="mor-project-card">
                <div class="mor-project-image" style="background-image: url('https://images.unsplash.com/photo-1456513080510-7bf3a84b82f8?auto=format&fit=crop&q=80&w=600');"></div>
                <div class="mor-project-content">
                    <div class="mor-project-tags">
                        <span class="mor-tag">Web 1.0</span>
                        <span class="mor-tag">HTML</span>
                    </div>
                    <h2 class="mor-project-title">Project Lexicon</h2>
                    <p class="mor-project-desc">An experimental dictionary format utilizing offline-first web technologies for extreme longevity.</p>
                    <div class="mor-project-links">
                        <span class="mor-project-link">Read Article &rarr;</span>
                    </div>
                </div>
            </a>

        </div>
    </div>
    "##
    .to_string()
}
