use crate::config::AnalyticsDashboardPageConfig;
use crate::render::pages::escape_html;

fn escape_js(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace("</script>", "<\\/script>")
}

/// Emits the Analytics Dashboard page as a Blogger HTML-page stencil.
///
/// This page intentionally emits layout + data hooks only. It inherits the
/// visual system from the Mor theme skin, especially tokens like:
/// `--bg-base`, `--bg-panel`, `--fg-base`, `--fg-muted`, `--accent`,
/// and `--border-color`.
pub fn generate_analytics_dashboard_html(config: &AnalyticsDashboardPageConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r##"<div class="mor-analytics-dashboard">
  <section class="mor-analytics-intro">
    <div class="mor-analytics-kicker">{kicker}</div>
    <h2 class="mor-analytics-title">{title}</h2>
    <p class="mor-analytics-desc">{desc}</p>
  </section>

  <div class="mor-analytics-notice">
    Public Blogger feeds can populate posts, comments, labels, and publish rhythm. Private dashboard-only metrics such as followers, storage, and pageviews use manual fallback values unless you wire in a private data source.
  </div>

  <section class="mor-analytics-summary" aria-labelledby="mor-analytics-summary-title">
    <h3 id="mor-analytics-summary-title">Blog Summary</h3>
    <ul class="mor-analytics-stat-grid">
      <li><span>Total Posts</span><strong id="mor-analytics-total-posts">Loading...</strong></li>
      <li><span>Total Comments</span><strong id="mor-analytics-total-comments">Loading...</strong></li>
      <li><span>Published Pages</span><strong id="mor-analytics-total-pages">Loading...</strong></li>
      <li><span>Followers</span><strong id="mor-analytics-followers">Manual</strong></li>
      <li><span>Views, Last 30 Days</span><strong id="mor-analytics-views-30">Manual</strong></li>
      <li><span>Total Views</span><strong id="mor-analytics-total-views">Manual</strong></li>
      <li><span>Storage Used</span><strong id="mor-analytics-storage">Manual</strong></li>
    </ul>
  </section>

  <div class="mor-analytics-grid">
    <section class="mor-analytics-card">
      <div class="mor-analytics-card-head">
        <h3>Blog Visitors</h3>
        <p>Manual placeholder because Blogger pageviews are not exposed through public feeds.</p>
      </div>
      <div class="mor-analytics-button-row">
        <button type="button" class="mor-analytics-button" data-mor-analytics-range="daily">Daily</button>
        <button type="button" class="mor-analytics-button" data-mor-analytics-range="15days">15 Days</button>
        <button type="button" class="mor-analytics-button is-active" data-mor-analytics-range="total">Total</button>
      </div>
      <div class="mor-analytics-chart" id="mor-analytics-visitors-chart">
        <div class="mor-analytics-placeholder">Manual visitor trend slot</div>
      </div>
    </section>

    <section class="mor-analytics-card">
      <div class="mor-analytics-card-head">
        <h3>Posts by Category</h3>
        <p>Built from Blogger post feed labels.</p>
      </div>
      <div class="mor-analytics-category-layout">
        <div class="mor-analytics-pie" id="mor-analytics-category-pie" aria-hidden="true"></div>
        <ul class="mor-analytics-legend" id="mor-analytics-category-legend">
          <li>Loading categories...</li>
        </ul>
      </div>
    </section>

    <section class="mor-analytics-card">
      <div class="mor-analytics-card-head">
        <h3>Posts Published Over Time</h3>
        <p>Built from public post publish dates.</p>
      </div>
      <div class="mor-analytics-button-row">
        <button type="button" class="mor-analytics-button" data-mor-analytics-posts-mode="daily">Daily</button>
        <button type="button" class="mor-analytics-button" data-mor-analytics-posts-mode="weekly">Weekly</button>
        <button type="button" class="mor-analytics-button is-active" data-mor-analytics-posts-mode="monthly">Monthly</button>
      </div>
      <div class="mor-analytics-chart" id="mor-analytics-posts-chart">
        <div class="mor-analytics-placeholder">Loading publishing rhythm...</div>
      </div>
    </section>

    <section class="mor-analytics-card">
      <div class="mor-analytics-card-head">
        <h3>Page Views Over Time</h3>
        <p>Manual placeholder unless a private analytics export is injected.</p>
      </div>
      <div class="mor-analytics-button-row">
        <button type="button" class="mor-analytics-button" data-mor-analytics-views-mode="daily">Daily</button>
        <button type="button" class="mor-analytics-button" data-mor-analytics-views-mode="weekly">Weekly</button>
        <button type="button" class="mor-analytics-button is-active" data-mor-analytics-views-mode="cumulative">Cumulative</button>
      </div>
      <div class="mor-analytics-chart" id="mor-analytics-views-chart">
        <div class="mor-analytics-placeholder">Manual pageview trend slot</div>
      </div>
    </section>
  </div>

  <div class="mor-analytics-updated">
    Last updated: <span id="mor-analytics-last-updated">Loading...</span>
  </div>
</div>
"##,
        kicker = escape_html(&config.kicker),
        title = escape_html(&config.title),
        desc = escape_html(&config.description),
    ));

    html.push_str(&format!(
        r#"<script>
window.morAnalyticsDashboardConfig = {{
  maxResults: {max_results},
  manualFollowers: "{followers}",
  manualThirtyDayViews: "{thirty_day_views}",
  manualTotalViews: "{total_views}",
  manualStorageUsed: "{storage_used}"
}};
</script>
"#,
        max_results = config.max_results,
        followers = escape_js(&config.manual_followers),
        thirty_day_views = escape_js(&config.manual_thirty_day_views),
        total_views = escape_js(&config.manual_total_views),
        storage_used = escape_js(&config.manual_storage_used),
    ));

    let js_template = include_str!("../../html_page_stencils/analytics_dashboard_script.js");
    html.push_str(&js_template.replace("{{MAX_RESULTS}}", &config.max_results.to_string()));

    format!("{}{}", crate::render::pages::page_chrome_overrides(&config.layout), html)
}
/// Compatibility alias for earlier patch attempts.
pub fn generate_analytics_html(config: &AnalyticsDashboardPageConfig) -> String {
    generate_analytics_dashboard_html(config)
}
