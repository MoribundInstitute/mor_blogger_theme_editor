//! Native plugins: each contributes CSS, XML widgets, and/or JS to the
//! rendered Blogger layout. Disabled plugins are simply not added to the
//! active list, so they cost nothing during rendering.

use crate::render::xml_generator::XmlNode;

/// The contract every renderable plugin implements. All hooks are optional;
/// a plugin overrides only the ones it needs.
pub trait MorBloggerPlugin: Send + Sync {
    /// Optional: Generates native Blogger XML widgets to insert into the structure.
    fn inject_xml_widgets(&self) -> Option<Vec<XmlNode>> {
        None
    }

    /// Optional: Returns the exact Vanilla JS required for this feature to work.
    fn inject_js(&self) -> Option<&'static str> {
        None
    }
}

// =========================================================================
// ISOLATED PLUGIN IMPLEMENTATIONS
// =========================================================================

/// Injects the Dark Mode toggle logic and local storage memory.
pub struct OsChameleonPlugin;

impl MorBloggerPlugin for OsChameleonPlugin {
    fn inject_js(&self) -> Option<&'static str> {
        Some(
            r##"
            const themeToggle = document.getElementById('mor-theme-toggle');
            const body = document.body;
            const currentTheme = localStorage.getItem('mor-theme') || 
                (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
            
            if (currentTheme === 'dark') body.classList.add('dark-mode');
            
            if (themeToggle) {
                themeToggle.addEventListener('click', (e) => {
                    e.preventDefault();
                    body.classList.toggle('dark-mode');
                    localStorage.setItem('mor-theme', body.classList.contains('dark-mode') ? 'dark' : 'light');
                });
            }
        "##,
        )
    }
}

/// Dynamically generates a Table of Contents based on document headers.
pub struct DeweyIndexerPlugin;

impl MorBloggerPlugin for DeweyIndexerPlugin {
    fn inject_js(&self) -> Option<&'static str> {
        Some(
            r##"
            const tocContainer = document.querySelector('.mor-toc-container');
            const postBody = document.querySelector('.mor-post-body');
            
            if (tocContainer && postBody) {
                const headings = postBody.querySelectorAll('h2, h3, h4');
                
                if (headings.length > 0) {
                    let html = '<ul class="mor-toc-list" style="list-style:none; padding:0; margin:0;">';
                    
                    headings.forEach((h, i) => {
                        const id = h.id || 'mor-heading-' + i;
                        h.id = id;
                        
                        const level = parseInt(h.tagName.substring(1));
                        const indent = (level - 2) * 16; 
                        
                        html += `<li style="margin-left: ${indent}px; margin-bottom: 8px;">
                                    <a href="#${id}" style="text-decoration:none; color:inherit; opacity:0.8; font-size:0.9rem;" 
                                       onmouseover="this.style.opacity='1'; this.style.color='var(--theme-accent, #60cdff)';" 
                                       onmouseout="this.style.opacity='0.8'; this.style.color='inherit';">
                                       ${h.textContent}
                                    </a>
                                 </li>`;
                    });
                    html += '</ul>';
                    tocContainer.innerHTML = html;
                } else {
                    tocContainer.innerHTML = '<p style="font-size:0.85rem; opacity:0.6;">No document anchors found.</p>';
                }
            }
        "##,
        )
    }

    fn inject_xml_widgets(&self) -> Option<Vec<XmlNode>> {
        Some(vec![XmlNode::new(
            "{{PLUGIN_WIDGET_SIDEBAR_RIGHT}}",
            "<div class='mor-toc-container'></div>",
        )])
    }
}
/// Adds collapsible left/right sidebar ("dock") toggles with persisted state.
pub struct WorkspaceDocksPlugin;

impl MorBloggerPlugin for WorkspaceDocksPlugin {
    fn inject_js(&self) -> Option<&'static str> {
        Some(
            r##"
            (function() {
                const body = document.body;
                const leftToggle  = document.getElementById('mor-dock-left-toggle');
                const rightToggle = document.getElementById('mor-dock-right-toggle');

                // Restore previous dock state from localStorage.
                if (localStorage.getItem('mor-dock-left')  === 'collapsed') body.classList.add('mor-dock-left-collapsed');
                if (localStorage.getItem('mor-dock-right') === 'collapsed') body.classList.add('mor-dock-right-collapsed');

                function bindToggle(btn, sideClass, storageKey) {
                    if (!btn) return;
                    btn.addEventListener('click', (e) => {
                        e.preventDefault();
                        const collapsed = body.classList.toggle(sideClass);
                        localStorage.setItem(storageKey, collapsed ? 'collapsed' : 'expanded');
                        btn.setAttribute('aria-expanded', String(!collapsed));
                    });
                    btn.setAttribute('aria-expanded', String(!body.classList.contains(sideClass)));
                }

                bindToggle(leftToggle,  'mor-dock-left-collapsed',  'mor-dock-left');
                bindToggle(rightToggle, 'mor-dock-right-collapsed', 'mor-dock-right');
            })();
        "##,
        )
    }
}

/// "Notification Bell" — a header bell that opens a dropdown showing the newest
/// post (via a native `FeaturedPost` widget). Colors follow the active theme via
/// CSS custom properties (no hardcoded palette). Injects its widget into the
/// `{{PLUGIN_WIDGET_HEADER}}` socket and the toggle JS into the page.
pub struct NotificationBellPlugin;

impl MorBloggerPlugin for NotificationBellPlugin {
    fn inject_xml_widgets(&self) -> Option<Vec<XmlNode>> {
        Some(vec![XmlNode::new(
            "{{PLUGIN_WIDGET_HEADER}}",
            r##"<nav class='mor-notif-container'>
  <span class='mor-notif-bell' role='button' tabindex='0' aria-label='Newest post'>
    <svg viewBox='0 0 24 24' width='22' height='22' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9'/><path d='M13.73 21a2 2 0 0 1-3.46 0'/></svg>
  </span>
  <div class='mor-notif-dropdown'>
    <b:section class='mor-notif-section' id='notification-section' maxwidgets='1' showaddelement='yes'>
      <b:widget id='FeaturedPost99' locked='false' title='Newest Post' type='FeaturedPost' version='1'>
        <b:widget-settings>
          <b:widget-setting name='showSnippet'>true</b:widget-setting>
          <b:widget-setting name='showPostTitle'>true</b:widget-setting>
          <b:widget-setting name='postId'>0</b:widget-setting>
          <b:widget-setting name='showFirstImage'>true</b:widget-setting>
          <b:widget-setting name='useMostRecentPost'>true</b:widget-setting>
        </b:widget-settings>
        <b:includable id='main'>
          <b:include name='content'/>
          <b:include name='quickedit'/>
        </b:includable>
        <b:includable id='content'>
          <a class='mor-notif-link' expr:href='data:postUrl'>
            <b:if cond='data:showPostTitle and data:postTitle != &quot;&quot;'>
              <h3 class='mor-notif-title'><data:postTitle/></h3>
            </b:if>
            <b:if cond='data:showFirstImage and data:postFirstImage != &quot;&quot;'>
              <img class='mor-notif-img' expr:src='data:postFirstImage'/>
            </b:if>
            <b:if cond='data:showSnippet and data:postSummary != &quot;&quot;'>
              <p class='mor-notif-snippet'><data:postSummary/></p>
            </b:if>
          </a>
        </b:includable>
      </b:widget>
    </b:section>
  </div>
</nav>
<style>
.mor-notif-container { position: relative; display: inline-flex; align-items: center; }
.mor-notif-bell { display: inline-flex; cursor: pointer; color: var(--fg-base, #ececeb); transition: color .2s ease, filter .2s ease; }
.mor-notif-bell:hover { color: var(--accent, #fff); filter: drop-shadow(0 0 6px var(--glow, rgba(255,255,255,.5))); }
.mor-notif-dropdown { display: none; position: absolute; top: 130%; right: 0; background: var(--bg-panel, #1c1c1e); color: var(--fg-base, #ececeb); border: 1px solid var(--border-color, rgba(255,255,255,.12)); border-radius: 8px; padding: 10px; min-width: 240px; max-width: 340px; box-shadow: var(--shadow-dropdown, 0 8px 24px rgba(0,0,0,.45)); z-index: var(--dropdown-z, 9999); }
.mor-notif-dropdown.open { display: block; }
.mor-notif-link { display: block; text-decoration: none; color: inherit; }
.mor-notif-title { margin: 0 0 6px; font-size: 1rem; font-weight: 700; font-family: var(--font-heading, inherit); color: var(--fg-base, #ececeb); }
.mor-notif-link:hover .mor-notif-title { color: var(--accent, #fff); text-shadow: 0 0 8px var(--glow, rgba(255,255,255,.4)); }
.mor-notif-snippet { margin: 6px 0 0; font-size: .85rem; line-height: 1.4; color: var(--fg-muted, #b9b9b8); }
.mor-notif-img { width: 100%; height: auto; border-radius: 6px; margin-top: 8px; }
</style>"##,
        )])
    }

    fn inject_js(&self) -> Option<&'static str> {
        Some(
            r##"
            (function () {
                if (window.__morNotifBellInit) return;
                window.__morNotifBellInit = true;
                document.addEventListener('click', function (e) {
                    var menu = document.querySelector('.mor-notif-container .mor-notif-dropdown');
                    if (!menu) return;
                    if (e.target.closest('.mor-notif-bell')) {
                        e.preventDefault();
                        e.stopPropagation();
                        menu.classList.toggle('open');
                        return;
                    }
                    if (!e.target.closest('.mor-notif-dropdown')) {
                        menu.classList.remove('open');
                    }
                });
                document.addEventListener('keydown', function (e) {
                    if (e.key === 'Escape') {
                        var m = document.querySelector('.mor-notif-container .mor-notif-dropdown.open');
                        if (m) m.classList.remove('open');
                    }
                });
            })();
        "##,
        )
    }
}
