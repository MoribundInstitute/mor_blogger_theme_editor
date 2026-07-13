use crate::config::pages::LmsConfig;
use crate::render::util::escape_attr as escape_html;

/// Emits the Syllabus page as pure layout. Colors resolve from inherited theme
/// tokens; the `<style>` block carries structure only.
pub fn generate_syllabus_html(config: &LmsConfig) -> String {
    let mut html = String::new();

    html.push_str(
        r##"<style>
.mor-syllabus-section {
  max-width: 800px;
  margin: 0 auto;
  font-family: inherit;
  color: var(--fg-base, inherit);
}
.mor-course-header {
  background: var(--bg-panel, rgba(128, 128, 128, 0.08));
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  padding: 40px;
  border-radius: 8px;
  margin-bottom: 40px;
}
.mor-course-header h1 {
  margin: 0 0 16px 0;
  color: var(--accent, #3b82f6);
  font-size: 2.2rem;
}
.mor-course-header p {
  font-size: 1.1rem;
  line-height: 1.6;
  color: var(--fg-dim, #888);
  margin: 0;
}
.mor-lesson-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.mor-lesson-item {
  display: flex;
  align-items: flex-start;
  gap: 20px;
  padding: 24px;
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  border-radius: 6px;
  transition: border-color 0.2s ease;
}
.mor-lesson-item:hover {
  border-color: var(--accent, #3b82f6);
}
.mor-lesson-marker {
  background: var(--bg-panel, rgba(128, 128, 128, 0.08));
  color: var(--fg-dim, #888);
  border: 1px solid var(--border-color, rgba(128, 128, 128, 0.3));
  border-radius: 50%;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  flex-shrink: 0;
}
.mor-lesson-content {
  flex-grow: 1;
}
.mor-lesson-content h3 {
  margin: 0 0 8px 0;
  font-size: 1.3rem;
}
.mor-lesson-content h3 a {
  color: var(--fg-base, inherit);
  text-decoration: none;
}
.mor-lesson-content h3 a:hover {
  color: var(--accent, #3b82f6);
  text-decoration: underline;
}
.mor-lesson-content p {
  margin: 0 0 12px 0;
  color: var(--fg-dim, #888);
  line-height: 1.5;
}
.mor-lesson-meta {
  font-size: 0.85rem;
  color: var(--accent, #3b82f6);
  font-family: monospace;
  text-transform: uppercase;
}
</style>
"##,
    );

    let mut lessons_html = String::new();
    for (i, lesson) in config.lessons.iter().enumerate() {
        let meta_tag = if !lesson.duration_label.trim().is_empty() {
            format!(
                r##"<div class="mor-lesson-meta">[ {} ]</div>"##,
                escape_html(&lesson.duration_label)
            )
        } else {
            String::new()
        };

        lessons_html.push_str(&format!(
            r##"  <div class="mor-lesson-item" data-mor-type="lesson" data-mor-id="{}">
    <div class="mor-lesson-marker">{}</div>
    <div class="mor-lesson-content">
      <h3><a href="{}">{}</a></h3>
      <p>{}</p>
      {}
    </div>
  </div>
"##,
            escape_html(&lesson.url), // The unique ID for the tracker
            i + 1,
            escape_html(&lesson.url),
            escape_html(&lesson.title),
            escape_html(&lesson.description),
            meta_tag
        ));
    }

    html.push_str(&format!(
        r##"<div class="mor-syllabus-section">
  <header class="mor-course-header">
    <h1>{course_title}</h1>
    <p>{course_desc}</p>
  </header>

  <div class="mor-lesson-list">
{lessons}
  </div>
</div>
"##,
        course_title = escape_html(&config.course_title),
        course_desc = escape_html(&config.course_description),
        lessons = lessons_html
    ));

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
