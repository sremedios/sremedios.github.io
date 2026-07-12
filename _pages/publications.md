---
layout: page
permalink: /publications/
title: publications
nav: true
nav_order: 3
description: Selected publications, full bibliography, and a record of work in medical imaging, inverse problems, and generative methods.
---

{% include bib_search.liquid %}

<div class="publications">
  <div class="publications-intro">
    <div class="callout-card">
      <h3>How to read this page</h3>
      <p>
        My publication record centers on medical imaging methods shaped by image formation, inverse problems, and learned priors, with recurring themes in reconstruction, super-resolution, generative modeling, and robustness across acquisition settings and populations. For a quick sense of the technical arc of my work, start with the selected publications below; the full bibliography follows by category.
      </p>
    </div>
    <div class="download-card callout-card">
      <h3>Key materials</h3>
      <a href="{{ '/assets/pdf/CurriculumVitae.pdf' | relative_url }}">Download curriculum vitae</a>
      <a href="{{ '/projects/' | relative_url }}">See selected projects</a>
    </div>
  </div>

  <h2>Selected publications</h2>
  <p class="section-intro">
    These papers represent the main technical contributions that define the arc of my work and connect most directly to the future directions described elsewhere on this site.
  </p>
  {% bibliography -f journal --query @*[keywords~=selected] --group_by none %}
  {% bibliography -f conf --query @*[keywords~=selected] --group_by none %}

  <h2>Full bibliography</h2>
  <p class="section-intro">
    The full record below is organized by publication type for quick reference.
  </p>

  <h3>Journal articles</h3>
  {% bibliography -f journal %}

  <h3>Conference papers and proceedings</h3>
  {% bibliography -f conf %}

  <h3>Theses</h3>
  {% bibliography -f theses %}

  <h3>Books and chapters</h3>
  {% bibliography -f books %}
  {% bibliography -f bookchapters %}
</div>
