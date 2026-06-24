(() => {
  document.addEventListener('DOMContentLoaded', () => {
    const feed = document.querySelector('.mor-magazine-feed');
    if (!feed) {
      return;
    }

    const gridPosts = feed.querySelectorAll('.grid-post');
    if (!gridPosts.length) {
      return;
    }

    const revealPost = (post) => {
      post.classList.add('mor-grid-visible');
      post.classList.remove('mor-grid-pending');
    };

    gridPosts.forEach((post) => {
      post.classList.add('mor-grid-pending');
    });

    if (!('IntersectionObserver' in window)) {
      gridPosts.forEach(revealPost);
      return;
    }

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            revealPost(entry.target);
            observer.unobserve(entry.target);
          }
        });
      },
      {
        root: null,
        rootMargin: '0px 0px -40px 0px',
        threshold: 0.12,
      }
    );

    gridPosts.forEach((post) => observer.observe(post));
  });
})();