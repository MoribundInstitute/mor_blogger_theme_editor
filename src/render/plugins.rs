//! Moribund Client OS - Vanilla JS Payload
//! This script is injected directly into the exported Blogger XML.
//! It handles client-side interactivity without external dependencies.

pub const CORE_FRAMEWORK_JS: &str = r##"
<script>
document.addEventListener('DOMContentLoaded', () => {
    
    // =========================================================
    // 1. OS CHAMELEON (Dark Mode Toggle & Memory)
    // =========================================================
    const themeToggle = document.getElementById('mor-theme-toggle');
    const body = document.body;
    
    // Check localStorage first, fallback to OS system preference
    const currentTheme = localStorage.getItem('mor-theme') || 
        (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    
    // Apply immediately to prevent flashbangs
    if (currentTheme === 'dark') {
        body.classList.add('dark-mode');
    }
    
    if (themeToggle) {
        themeToggle.addEventListener('click', (e) => {
            e.preventDefault();
            body.classList.toggle('dark-mode');
            const newTheme = body.classList.contains('dark-mode') ? 'dark' : 'light';
            localStorage.setItem('mor-theme', newTheme);
        });
    }

    // =========================================================
    // 2. THE DEWEY INDEXER (Dynamic Table of Contents)
    // =========================================================
    const tocContainer = document.querySelector('.mor-toc-container');
    const postBody = document.querySelector('.mor-post-body'); // The main Blogger post content
    
    if (tocContainer && postBody) {
        // Find all subheadings in the article
        const headings = postBody.querySelectorAll('h2, h3, h4');
        
        if (headings.length > 0) {
            let html = '<ul class="mor-toc-list" style="list-style:none; padding:0; margin:0;">';
            
            headings.forEach((h, i) => {
                // Give the heading an ID if Blogger didn't generate one
                const id = h.id || 'mor-heading-' + i;
                h.id = id;
                
                // Indent h3 and h4 tags automatically
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

    // =========================================================
    // 3. WORKSPACE DOCKS (Mobile/Tablet Sidebars)
    // =========================================================
    const leftToggle = document.getElementById('mor-left-toggle');
    const rightToggle = document.getElementById('mor-right-toggle');
    const workspace = document.querySelector('.mor-workspace');
    
    if (workspace) {
        if (leftToggle) {
            leftToggle.addEventListener('click', (e) => {
                e.preventDefault();
                workspace.classList.toggle('left-open');
                workspace.classList.remove('right-open'); // Close the other one
            });
        }
        if (rightToggle) {
            rightToggle.addEventListener('click', (e) => {
                e.preventDefault();
                workspace.classList.toggle('right-open');
                workspace.classList.remove('left-open'); // Close the other one
            });
        }
    }
});
</script>
"##;
