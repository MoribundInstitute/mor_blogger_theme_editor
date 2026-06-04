pub fn generate_ap_catalog_html<T>(_config: &T) -> String {
    r#"
    <style>
        /* Scoped CSS for the AP Course Catalog */
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

        /* Discipline Sidebar */
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
            text-decoration: none;
            color: inherit;
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
            <h1 class="mor-lms-title">Advanced Placement (AP) Curriculum</h1>
            <p class="mor-lms-subtitle">Comprehensive prep modules, practice exams, and course notes for all 40 AP subjects.</p>
        </header>

        <div class="mor-lms-layout">
            <aside class="mor-lms-sidebar">
                <h3 class="mor-sidebar-title">Disciplines</h3>
                <ul class="mor-subject-list">
                    <li><a href="/search/label/AP%20Capstone">AP Capstone Diploma</a></li>
                    <li><a href="/search/label/AP%20Arts">Arts</a></li>
                    <li><a href="/search/label/AP%20English">English</a></li>
                    <li><a href="/search/label/AP%20History">History &amp; Social Sciences</a></li>
                    <li><a href="/search/label/AP%20Math">Math &amp; Computer Science</a></li>
                    <li><a href="/search/label/AP%20Science">Sciences</a></li>
                    <li><a href="/search/label/AP%20Languages">World Languages &amp; Cultures</a></li>
                </ul>
            </aside>

            <main class="mor-course-grid">
                
                <a href="/search/label/AP%20English%20Language" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ENG-LANG</span>
                        <span class="mor-badge active">In Progress</span>
                    </div>
                    <h2 class="mor-course-title">English Language and Composition</h2>
                    <p class="mor-course-desc">Rhetorical analysis, synthesis, and evidence-based argumentation based on non-fiction texts.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 3: Claims and Evidence</span> <span>42%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 42%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20US%20History" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">HIST-USH</span>
                        <span class="mor-badge done">Completed</span>
                    </div>
                    <h2 class="mor-course-title">United States History</h2>
                    <p class="mor-course-desc">Cultural, economic, and political history from 1491 to present. Master the DBQ and LEQ writing formats.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Exam Prep Mode</span> <span>100%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 100%; background: var(--mor-success, #50fa7b);"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20English%20Literature" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ENG-LIT</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">English Literature and Composition</h2>
                    <p class="mor-course-desc">Close reading and critical analysis of imaginative literature to deepen understanding of structure and theme.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Short Fiction I</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20World%20History" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">HIST-WHAP</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">World History: Modern</h2>
                    <p class="mor-course-desc">The development of global processes and contacts from 1200 CE to the present.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: The Global Tapestry</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20US%20Government" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">GOV-US</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Gov. and Politics - United States</h2>
                    <p class="mor-course-desc">Foundations, institutions, and political behaviors of the American system and Supreme Court.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Foundations of Democracy</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Psychology" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SOC-PSY</span>
                        <span class="mor-badge active">In Progress</span>
                    </div>
                    <h2 class="mor-course-title">Psychology</h2>
                    <p class="mor-course-desc">Behavior, cognition, and the scientific study of the human mind. High-yield vocabulary flashcards included.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 2: Biological Bases of Behavior</span> <span>18%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 18%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Calculus%20AB" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">MATH-AB</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Calculus AB</h2>
                    <p class="mor-course-desc">Limits, derivatives, integrals, and the Fundamental Theorem of Calculus.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Limits and Continuity</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Human%20Geography" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SOC-HUG</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Human Geography</h2>
                    <p class="mor-course-desc">Patterns and processes that have shaped human understanding, use, and alteration of Earth's surface.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Thinking Geographically</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Biology" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-BIO</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Biology</h2>
                    <p class="mor-course-desc">Evolution, cellular processes, genetics, and ecology. Includes interactive lab simulations.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Chemistry of Life</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Statistics" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">MATH-STAT</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Statistics</h2>
                    <p class="mor-course-desc">Collecting, analyzing, and drawing conclusions from data using probability and statistical inference.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Exploring One-Variable Data</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Environmental%20Science" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-ENV</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Environmental Science</h2>
                    <p class="mor-course-desc">Interrelationships of the natural world, identifying environmental problems and analyzing risks.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: The Living World: Ecosystems</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Precalculus" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">MATH-PRE</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Precalculus</h2>
                    <p class="mor-course-desc">Functions, modeling, and trigonometry to prepare for higher mathematics and calculus.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Polynomial and Rational Functions</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Spanish%20Language" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-SPA</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Spanish Language and Culture</h2>
                    <p class="mor-course-desc">Interpersonal, interpretive, and presentational communication in Spanish.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Computer%20Science%20Principles" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">COMP-CSP</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Computer Science Principles</h2>
                    <p class="mor-course-desc">The internet, data analysis, and the foundational concepts of programming and algorithms.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Creative Development</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Physics%201" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-PHY1</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Physics 1</h2>
                    <p class="mor-course-desc">Newtonian mechanics, work, energy, and power in an algebra-based format.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Kinematics</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Macroeconomics" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ECON-MAC</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Macroeconomics</h2>
                    <p class="mor-course-desc">Principles that apply to an economic system as a whole, including GDP, inflation, and monetary policy.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Basic Economic Concepts</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Chemistry" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-CHEM</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Chemistry</h2>
                    <p class="mor-course-desc">Atomic structure, intermolecular forces, thermodynamics, and equilibrium.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Atomic Structure and Properties</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Calculus%20BC" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">MATH-BC</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Calculus BC</h2>
                    <p class="mor-course-desc">Extends AB concepts to include polar curves, parametric equations, and series.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 9: Parametric Equations &amp; Polar Coordinates</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Microeconomics" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ECON-MIC</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Microeconomics</h2>
                    <p class="mor-course-desc">Principles that apply to the functions of individual decision-makers, supply and demand, and market structures.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Basic Economic Concepts</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Computer%20Science%20A" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">COMP-CSA</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Computer Science A</h2>
                    <p class="mor-course-desc">Object-oriented programming methodology using Java, data structures, and algorithms.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Primitive Types</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Seminar" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">CAP-SEM</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Seminar</h2>
                    <p class="mor-course-desc">Investigate real-world topics, gather and analyze information from multiple sources, and develop arguments.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Performance Task 1</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20European%20History" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">HIST-EURO</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">European History</h2>
                    <p class="mor-course-desc">Cultural, economic, and political history of Europe from 1450 to the present.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Renaissance and Exploration</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Physics%20C%20Mechanics" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-PHCM</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Physics C: Mechanics</h2>
                    <p class="mor-course-desc">Calculus-based study of kinematics, Newton's laws, energy, and rotational motion.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Kinematics</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%202D%20Design" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ART-2D</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Art and Design: 2-D Design</h2>
                    <p class="mor-course-desc">Application of 2-D design principles across various media including graphic design and photography.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Portfolio Building</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Research" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">CAP-RES</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Research</h2>
                    <p class="mor-course-desc">Design, plan, and conduct a year-long research-based investigation on a topic of personal interest.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Academic Paper Draft</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Physics%20C%20EM" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-PHCE</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Physics C: Electricity &amp; Magnetism</h2>
                    <p class="mor-course-desc">Calculus-based study of electrostatics, conductors, capacitors, and electromagnetism.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Electrostatics</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Art%20History" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ART-HIST</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Art History</h2>
                    <p class="mor-course-desc">Global art traditions, materials, and cultural contexts from prehistory to the contemporary world.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Global Prehistory</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Spanish%20Literature" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-SPL</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Spanish Literature</h2>
                    <p class="mor-course-desc">Peninsular and Latin American literature spanning from the Middle Ages to the modern era.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: La Época Medieval</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Comparative%20Government" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">GOV-COMP</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Gov. and Politics - Comparative</h2>
                    <p class="mor-course-desc">Comparison of political concepts and systems across six selected countries.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Political Systems and Regimes</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Drawing" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ART-DRAW</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Art and Design: Drawing</h2>
                    <p class="mor-course-desc">Mark-making, line, surface, space, and light/shade techniques to build a visual portfolio.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Sustained Investigation</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Physics%202" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SCI-PHY2</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Physics 2</h2>
                    <p class="mor-course-desc">Algebra-based study of fluids, thermodynamics, electricity, magnetism, optics, and modern physics.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Fluids</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20French" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-FRE</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">French Language and Culture</h2>
                    <p class="mor-course-desc">Communication and cultural understanding in the Francophone world.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Music%20Theory" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">MUS-THE</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Music Theory</h2>
                    <p class="mor-course-desc">Analysis of pitch, rhythm, form, and musical design. Includes sight-singing and aural skills.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Fundamentals</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Chinese" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-CHI</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Chinese Language and Culture</h2>
                    <p class="mor-course-desc">Communication and cultural appreciation of the Mandarin-speaking world.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20African%20American%20Studies" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">SOC-AAS</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">African American Studies</h2>
                    <p class="mor-course-desc">The history, politics, arts, and contributions of people of African descent.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Origins of the Diaspora</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%203D%20Design" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">ART-3D</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Art and Design: 3-D Design</h2>
                    <p class="mor-course-desc">Spatial relationships and application of 3-D design principles in sculptural formats.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Portfolio Building</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Latin" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-LAT</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Latin</h2>
                    <p class="mor-course-desc">Translation, comprehension, and analysis of classic Latin poetry and prose.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Vergil's Aeneid</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20German" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-GER</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">German Language and Culture</h2>
                    <p class="mor-course-desc">Interpersonal, interpretive, and presentational communication in German.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Japanese" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-JAP</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Japanese Language and Culture</h2>
                    <p class="mor-course-desc">Communication and understanding cultural nuances in written and spoken Japanese.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

                <a href="/search/label/AP%20Italian" class="mor-course-card">
                    <div class="mor-card-meta">
                        <span class="mor-course-code">LANG-ITA</span>
                        <span class="mor-badge">Not Started</span>
                    </div>
                    <h2 class="mor-course-title">Italian Language and Culture</h2>
                    <p class="mor-course-desc">Fluency and cultural context of the Italian language and global Italian communities.</p>
                    <div class="mor-progress-container">
                        <div class="mor-progress-text"><span>Unit 1: Families in Different Societies</span> <span>0%</span></div>
                        <div class="mor-progress-track"><div class="mor-progress-fill" style="width: 0%;"></div></div>
                    </div>
                </a>

            </main>
        </div>
    </div>
    "#
    .to_string()
}