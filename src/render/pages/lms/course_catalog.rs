pub fn generate_course_catalog_html<T>(_config: &T) -> String {
    r#"
    <style>
        /* Scoped CSS for the Course Catalog */
        .mor-lms-container {
            display: flex;
            flex-direction: column;
            gap: 32px;
            max-width: 1200px;
            margin: 0 auto;
            color: var(--theme-text, #cccccc);
            font-family: var(--theme-font, system-ui, sans-serif);
        }

        .mor-lms-header {
            border-bottom: 2px solid var(--theme-border, #333);
            padding-bottom: 16px;
        }

        .mor-lms-title {
            margin: 0;
            font-size: 2rem;
            color: var(--theme-accent, #60cdff);
            letter-spacing: 1px;
        }

        .mor-lms-subtitle {
            margin: 8px 0 0 0;
            opacity: 0.8;
            font-size: 1rem;
        }

        .mor-lms-layout {
            display: grid;
            grid-template-columns: 280px 1fr;
            gap: 32px;
            align-items: start;
        }

        /* Dewey Sidebar */
        .mor-lms-sidebar {
            background: var(--theme-bg-panel, #252525);
            border: 1px solid var(--theme-border, #333);
            border-radius: 6px;
            padding: 20px;
        }

        .mor-sidebar-title {
            margin: 0 0 16px 0;
            font-size: 0.85rem;
            text-transform: uppercase;
            letter-spacing: 2px;
            color: var(--theme-text-muted, #888);
        }

        .mor-subject-list {
            list-style: none;
            padding: 0;
            margin: 0;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .mor-subject-list a {
            display: block;
            color: var(--theme-text, #ccc);
            text-decoration: none;
            font-size: 0.95rem;
            padding: 6px 8px;
            border-radius: 4px;
            transition: all 0.1s ease;
        }

        .mor-subject-list a:hover {
            background: var(--theme-accent, #60cdff);
            color: #111;
        }

        /* Course Grid */
        .mor-course-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
            gap: 20px;
        }

        .mor-course-card {
            display: flex;
            flex-direction: column;
            background: var(--theme-bg-panel, #252525);
            border: 1px solid var(--theme-border, #333);
            border-radius: 6px;
            padding: 20px;
            transition: transform 0.15s ease, box-shadow 0.15s ease;
        }

        .mor-course-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 24px rgba(0,0,0,0.4);
            border-color: var(--theme-accent, #60cdff);
        }

        .mor-card-meta {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 12px;
            font-size: 0.8rem;
            font-weight: bold;
            font-family: monospace;
        }

        .mor-course-code {
            color: var(--theme-accent, #60cdff);
        }

        .mor-badge {
            background: var(--theme-bg-elevated, #333);
            padding: 4px 8px;
            border-radius: 20px;
            text-transform: uppercase;
            font-size: 0.7rem;
            letter-spacing: 1px;
        }
        
        .mor-badge.active { background: rgba(96, 205, 255, 0.15); color: #60cdff; border: 1px solid #60cdff; }
        .mor-badge.done { background: rgba(80, 250, 123, 0.15); color: #50fa7b; border: 1px solid #50fa7b; }

        .mor-course-title {
            margin: 0 0 8px 0;
            font-size: 1.25rem;
            line-height: 1.3;
        }

        .mor-course-desc {
            margin: 0 0 20px 0;
            font-size: 0.9rem;
            opacity: 0.8;
            line-height: 1.5;
            flex-grow: 1;
        }

        /* Progress Bar */
        .mor-progress-container {
            margin-top: auto;
        }

        .mor-progress-text {
            display: flex;
            justify-content: space-between;
            font-size: 0.8rem;
            margin-bottom: 6px;
            color: var(--theme-text-muted, #888);
        }

        .mor-progress-track {
            height: 6px;
            background: var(--theme-bg-elevated, #1a1a1c);
            border-radius: 3px;
            overflow: hidden;
            border: 1px solid var(--theme-border, #333);
        }

        .mor-progress-fill {
            height: 100%;
            background: var(--theme-accent, #60cdff);
            border-radius: 3px;
        }

        @media (max-width: 900px) {
            .mor-lms-layout { grid-template-columns: 1fr; }
            .mor-lms-sidebar { order: 2; }
            .mor-course-grid { order: 1; }
        }
    </style>

    <div class="mor-lms-container">
        <header class="mor-lms-header">
            <h1 class="mor-lms-title">Course Catalog</h1>
            <p class="mor-lms-subtitle">Access the Moribund Institute's archival learning modules and interactive tracks.</p>
        </header>

        <div class="mor-lms-layout">
            <aside class="mor-lms-sidebar">
                <h3 class="mor-sidebar-title">Disciplines</h3>
                <ul class="mor-subject-list">
                    <li><a href="{{SUBJECT_000_URL}}">{{SUBJECT_000_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_100_URL}}">{{SUBJECT_100_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_200_URL}}">{{SUBJECT_200_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_300_URL}}">{{SUBJECT_300_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_400_URL}}">{{SUBJECT_400_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_500_URL}}">{{SUBJECT_500_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_600_URL}}">{{SUBJECT_600_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_700_URL}}">{{SUBJECT_700_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_800_URL}}">{{SUBJECT_800_LABEL}}</a></li>
                    <li><a href="{{SUBJECT_900_URL}}">{{SUBJECT_900_LABEL}}</a></li>
                </ul>
            </aside>

            <main class="mor-course-grid">
                
                <article class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LEX-101</span>
                        <span class="mor-badge active">In Progress</span>
                    </div>
                    <h2 class="mor-course-title">Introduction to Lexicography</h2>
                    <p class="mor-course-desc">Explore the architecture of language, the evolution of dictionaries, and the anatomy of a definition.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Module 3 of 8</span> <span>37%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 37%;"></div></div>
                    </div>
                </article>

                <article class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SYS-404</span>
                        <span class="mor-badge done">Completed</span>
                    </div>
                    <h2 class="mor-course-title">Digital Aesthetics &amp; Decay</h2>
                    <p class="mor-course-desc">A deep dive into Web 1.0 brutalism, dead links, and the preservation of digital ruins.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Final Assessment</span> <span>100%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 100%; background: var(--mor-success, #50fa7b);"></div></div>
                    </div>
                </article>

                <article class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">COG-205</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Frameworks of Memory</h2>
                    <p class="mor-course-desc">Building cognitive palaces, spaced repetition, and offloading memory to physical ledgers.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Module 1 of 12</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </article>

            </main>
        </div>
    </div>
    "#
    .to_string()
}