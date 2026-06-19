//! Moribund Client OS - Modular Plugin Architecture
//! This establishes the core trait for all native plugins.
//! Disabled plugins incur zero memory or payload overhead.

use crate::render::xml_generator::XmlNode;

/// The strict contract that all modular features must agree to.
/// Using static string references ensures that plugins that are toggled off
/// cost absolutely zero memory overhead during the rendering pipeline.
pub trait MorBloggerPlugin: Send + Sync {
    /// A unique, machine-readable text identifier (e.g., "os_chameleon").
    fn id(&self) -> &'static str;

    /// The human-readable name shown to the user in the Plugin Manager.
    fn display_name(&self) -> &'static str;

    /// The semantic version of the plugin (e.g., "1.0.0").
    fn version(&self) -> &'static str;

    /// Optional: Generates standard CSS to inject into the Blogger layout.
    fn inject_css(&self) -> Option<String> {
        None
    }

    /// Optional: Generates native Blogger XML widgets to insert into the structure.
    fn inject_xml_widgets(&self) -> Option<Vec<XmlNode>> {
        None
    }

    /// Optional: Returns the exact Vanilla JS required for this feature to work.
    fn inject_js(&self) -> Option<&'static str> {
        None
    }

    /// Optional: Analyzes the current layout and returns a plain-English warning
    /// if a missing tag or optimization issue is detected.
    fn run_diagnostics(&self) -> Option<String> {
        None
    }
}

// =========================================================================
// ISOLATED PLUGIN IMPLEMENTATIONS
// =========================================================================

/// Injects the Dark Mode toggle logic and local storage memory.
pub struct OsChameleonPlugin;

impl MorBloggerPlugin for OsChameleonPlugin {
    fn id(&self) -> &'static str {
        "os_chameleon"
    }

    fn display_name(&self) -> &'static str {
        "OS Chameleon (Dark Mode)"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

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
    fn id(&self) -> &'static str {
        "dewey_indexer"
    }

    fn display_name(&self) -> &'static str {
        "Dewey Indexer (Auto-TOC)"
    }

    fn version(&self) -> &'static str {
        "1.1.0"
    }

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
    fn id(&self) -> &'static str {
        "workspace_docks"
    }

    fn display_name(&self) -> &'static str {
        "Workspace Docks (Collapsible Sidebars)"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

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
