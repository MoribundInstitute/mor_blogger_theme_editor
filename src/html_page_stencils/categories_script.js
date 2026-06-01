<script>
(function () {
  const sections = [
    { id: "author-links", keyword: "Author" },
    { id: "musician-links", keyword: "Musician" },
    { id: "painter-links", keyword: "Painter" },
    { id: "actor-links", keyword: "Actor" },
    { id: "anime-links", keyword: "Anime" },
    { id: "kdrama-links", keyword: "Korean Drama" },
    { id: "animal-links", keyword: "Animal" }
  ];

  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");

  sections.forEach(({ id, keyword }) => {
    const container = document.getElementById(id);
    if (!container) return;

    alphabet.forEach(letter => {
      const label = keyword + ": " + letter;
      const link = document.createElement("a");
      link.href = "/search/label/" + encodeURIComponent(label);
      link.textContent = letter;
      link.setAttribute("aria-label", keyword + " labels beginning with " + letter);
      container.appendChild(link);
    });
  });
})();
</script>
