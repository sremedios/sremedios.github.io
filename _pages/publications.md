---
layout: page
permalink: /publications/
title: publications
nav: true
nav_order: 3
description: Selected contributions, full bibliography, and recent scholarly activity.
---

{% include bib_search.liquid %}

<div class="publications">
  <div class="publications-intro">
    <div class="callout-card">
      <h3>How to read this page</h3>
      <p>
        My work spans medical image reconstruction, super-resolution, generative modeling, harmonization, and robust analysis across MRI and related modalities. For a quick overview, start with the selected work below; the complete bibliography follows by category.
      </p>
    </div>
    <div class="download-card callout-card">
      <h3>Key materials</h3>
      <a href="{{ '/assets/pdf/CirriculumVitae.pdf' | relative_url }}">Download CV</a>
      <a href="{{ '/assets/pdf/ResearchStatement.pdf' | relative_url }}">Download research statement</a>
      <a href="{{ '/projects/' | relative_url }}">View project summaries</a>
    </div>
  </div>

  <h2>Selected work</h2>
  {% bibliography --query @*[selected=true] %}

  <h2>Journal articles</h2>
  {% bibliography -f journal %}

  <h2>Accepted conference papers and proceedings</h2>
  {% bibliography -f conf %}

  <h2>Theses</h2>
  {% bibliography -f theses %}

  <h2>Books and chapters</h2>
  {% bibliography -f books %}
  {% bibliography -f bookchapters %}
</div>
