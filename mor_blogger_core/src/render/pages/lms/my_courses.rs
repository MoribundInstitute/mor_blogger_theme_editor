use crate::config::pages::LmsConfig;

/// Emits the "My Courses" dashboard.
/// Designed for Local-First hydration (LocalStorage or Browser Extension).
pub fn generate_my_courses_html(config: &LmsConfig) -> String {
    let mut html = String::new();

    // 1. Dashboard CSS
    html.push_str(
        r##"<style>
.mor-dashboard-section {
  max-width: 1000px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base, inherit);
}

/* User Stats Header (Streak, Level, etc.) */
.mor-dash-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-panel, #161b22);
  border: 1px solid var(--border-color, #30363d);
  padding: 24px 32px;
  border-radius: 8px;
  margin-bottom: 40px;
}
.mor-user-profile {
  display: flex;
  align-items: center;
  gap: 16px;
}
.mor-avatar {
  width: 64px;
  height: 64px;
  background: var(--bg-elevated, #21262d);
  border: 2px solid var(--border-color, #30363d);
  border-radius: 50%;
}
.mor-user-info h2 { margin: 0 0 4px 0; font-size: 1.5rem; }
.mor-user-info p { margin: 0; color: var(--fg-dim, #8b949e); font-size: 0.9rem; font-family: monospace; }

/* Stats Bar */
.mor-stats-row {
  display: flex;
  gap: 24px;
  text-align: center;
}
.mor-stat-block span { display: block; }
.mor-stat-val { font-size: 1.5rem; font-weight: bold; color: var(--accent, #58a6ff); }
.mor-stat-label { font-size: 0.75rem; color: var(--fg-dim, #8b949e); text-transform: uppercase; letter-spacing: 1px; margin-top: 4px; }

/* Course Grid */
.mor-dash-title {
  font-size: 2rem;
  margin-bottom: 24px;
  border-bottom: 1px dashed var(--border-color, #30363d);
  padding-bottom: 12px;
}
.mor-active-courses {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

/* Active Course Card */
.mor-dash-card {
  background: var(--bg-panel, #161b22);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 6px;
  padding: 20px;
  display: flex;
  flex-direction: column;
}
.mor-dash-card-header {
  display: flex;
  gap: 16px;
  margin-bottom: 16px;
}
.mor-course-icon {
  width: 48px;
  height: 48px;
  background: var(--bg-elevated, #21262d);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}
.mor-dash-card-header h3 { margin: 0 0 6px 0; font-size: 1.1rem; line-height: 1.3; }
.mor-dash-card-header p { margin: 0; font-size: 0.85rem; color: var(--fg-dim, #8b949e); }

/* Progress Bar Engine */
.mor-progress-container {
  margin-top: auto;
}
.mor-progress-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.8rem;
  color: var(--fg-dim, #8b949e);
  margin-bottom: 8px;
  font-family: monospace;
}
.mor-progress-track {
  width: 100%;
  height: 8px;
  background: var(--bg-elevated, #21262d);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 16px;
}
.mor-progress-fill {
  height: 100%;
  background: var(--accent, #58a6ff);
  width: 0%; /* Hydrated by JS */
  transition: width 0.5s ease-out;
}

.mor-dash-action {
  width: 100%;
  padding: 10px;
  background: transparent;
  color: var(--accent, #58a6ff);
  border: 1px solid var(--border-color, #30363d);
  border-radius: 4px;
  text-align: center;
  text-decoration: none;
  font-weight: bold;
  cursor: pointer;
}
.mor-dash-action:hover {
  background: var(--bg-elevated, #21262d);
}
</style>
"##,
    );

    // 2. HTML Structure
    html.push_str(
        r##"<div class="mor-dashboard-section">
  
  <header class="mor-dash-header">
    <div class="mor-user-profile">
      <div class="mor-avatar"></div>
      <div class="mor-user-info">
        <h2 id="mor-local-username">Local Scholar</h2>
        <p>Privacy-Respecting Node</p>
      </div>
    </div>
    <div class="mor-stats-row">
      <div class="mor-stat-block">
        <span class="mor-stat-val" id="mor-local-streak">0</span>
        <span class="mor-stat-label">Day Streak</span>
      </div>
      <div class="mor-stat-block">
        <span class="mor-stat-val" id="mor-local-points">0</span>
        <span class="mor-stat-label">XP Points</span>
      </div>
    </div>
  </header>

  <h1 class="mor-dash-title">My Courses</h1>

  <div class="mor-active-courses" id="mor-course-grid">
    <div class="mor-dash-card" data-course-id="macroeconomics_101">
      <div class="mor-dash-card-header">
        <div class="mor-course-icon">📈</div>
        <div>
          <h3>Macroeconomics</h3>
          <p>National income and price determination</p>
        </div>
      </div>
      <div class="mor-progress-container">
        <div class="mor-progress-meta">
          <span class="progress-label">In Progress</span>
          <span class="progress-pct">0%</span>
        </div>
        <div class="mor-progress-track">
          <div class="mor-progress-fill"></div>
        </div>
        <a href="/p/macroeconomics-syllabus.html" class="mor-dash-action">Resume</a>
      </div>
    </div>

    <div class="mor-dash-card" data-course-id="finitude_fighting">
      <div class="mor-dash-card-header">
        <div class="mor-course-icon">⚔️</div>
        <div>
          <h3>Finitude Fighting</h3>
          <p>Philosophical defense mechanisms</p>
        </div>
      </div>
      <div class="mor-progress-container">
        <div class="mor-progress-meta">
          <span class="progress-label">Not Started</span>
          <span class="progress-pct">0%</span>
        </div>
        <div class="mor-progress-track">
          <div class="mor-progress-fill"></div>
        </div>
        <a href="/p/finitude-syllabus.html" class="mor-dash-action">Start Course</a>
      </div>
    </div>

  </div>
</div>
"##,
    );

    // 3. Hydration Script (The "Brain")
    // This script checks LocalStorage and updates the UI.
    // If you build an extension later, the extension simply writes to LocalStorage.
    html.push_str(
        r##"<script>
document.addEventListener("DOMContentLoaded", function() {
  // 1. Simulate pulling data from a Privacy-Respecting Local DB
  // In production, this comes from localStorage.getItem('mor_lms_state')
  const localLmsState = {
    user: { name: "Moribund Murdoch", streak: 4, xp: 18723 },
    progress: {
      "macroeconomics_101": 65, // 65% complete
      "finitude_fighting": 0
    }
  };

  // 2. Hydrate Header Stats
  document.getElementById('mor-local-username').innerText = localLmsState.user.name;
  document.getElementById('mor-local-streak').innerText = localLmsState.user.streak;
  document.getElementById('mor-local-points').innerText = localLmsState.user.xp.toLocaleString();

  // 3. Hydrate Course Progress Bars
  const courseCards = document.querySelectorAll('.mor-dash-card');
  courseCards.forEach(card => {
    const courseId = card.getAttribute('data-course-id');
    const completionPct = localLmsState.progress[courseId] || 0;
    
    // Animate the bar
    const fillBar = card.querySelector('.mor-progress-fill');
    const pctText = card.querySelector('.progress-pct');
    const actionBtn = card.querySelector('.mor-dash-action');

    // Slight delay so the user sees the animation trigger on load
    setTimeout(() => {
      fillBar.style.width = completionPct + '%';
      pctText.innerText = completionPct + '%';

      if (completionPct > 0 && completionPct < 100) {
        actionBtn.innerText = "Resume";
      } else if (completionPct === 100) {
        actionBtn.innerText = "Review";
        card.style.borderColor = "var(--accent)"; // Highlight completed courses
      }
    }, 100);
  });
});
</script>
"##,
    );

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
