---
layout: page
permalink: /projects/
title: projects
nav: false
nav_order: 2
description: Selected research contributions showing the technical arc of my work in computational medical imaging.
---

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Selected contributions</div>
    <h2>Projects that demonstrate the arc of my research program</h2>
    <p>
      These projects show how my work moves from clinically grounded problem formulation to technical innovation in inverse problems, generative modeling, and robust medical imaging.
    </p>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Case study 1</div>
    <h2>Through-plane super-resolution for anisotropic MRI</h2>
    <p>
      A central theme of my research is how to recover missing spatial detail in clinical MRI without relying on unrealistic training assumptions. This line of work addresses a common clinical setting in which scans have high in-plane resolution but thick slices or slice gaps through plane.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Standard super-resolution methods often assume access to clean paired high-resolution targets or synthetic degradations that do not match how clinical MRI is actually acquired.
      </p>
    </div>
    <div class="mini-card">
      <h3>Contribution</h3>
      <p>
        I developed acquisition-aware, internally trained, and cycle-consistent approaches that explicitly model the MRI imaging operator, allowing reconstruction methods to learn from the scan itself and better reflect the physics of data acquisition.
      </p>
    </div>
    <div class="mini-card">
      <h3>Why it matters</h3>
      <p>
        This work shows how super-resolution can be made more realistic, more robust across settings, and less dependent on idealized supervised pipelines, helping lay the groundwork for clinically credible image enhancement.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Related publications</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Case study 2</div>
    <h2>Zero-shot and internally trained reconstruction</h2>
    <p>
      Across several projects, I study how imaging models can reduce dependence on large external paired datasets by learning directly from structure already present within an image or acquisition.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Supervised medical imaging pipelines often inherit biases from their training distributions and can fail when applied to new scanners, protocols, or clinical populations.
      </p>
    </div>
    <div class="mini-card">
      <h3>Contribution</h3>
      <p>
        I developed zero-shot and scan-specific learning methods that train on internal image structure rather than only on external datasets, enabling flexible reconstruction even when matched training data are unavailable.
      </p>
    </div>
    <div class="mini-card">
      <h3>Why it matters</h3>
      <p>
        This work advances a broader research agenda in which medical imaging systems are designed to adapt to local structure and acquisition context, rather than relying exclusively on fixed training corpora.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Related work</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Case study 3</div>
    <h2>Generative priors for MRI inverse problems</h2>
    <p>
      I am interested in how modern generative models can be used in medical imaging without sacrificing reliability. This motivates my work on diffusion-based and generative formulations for inverse problems.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Generative models can capture rich image structure, but naively using them for reconstruction risks producing outputs that look plausible while departing from the actual acquired measurements.
      </p>
    </div>
    <div class="mini-card">
      <h3>Contribution</h3>
      <p>
        I study data-consistent generative inference, including diffusion-based priors that explicitly enforce agreement with measured observations while still leveraging learned image distributions.
      </p>
    </div>
    <div class="mini-card">
      <h3>Why it matters</h3>
      <p>
        This work addresses one of the central obstacles to deploying generative models in medicine: how to benefit from expressive priors without enabling clinically unsafe hallucinations.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/research/' | relative_url }}">Research vision</a>
    <a href="{{ '/publications/' | relative_url }}">Related publications</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Case study 4</div>
    <h2>Large-scale 3D brain MRI curation for generative modeling</h2>
    <p>
      Building useful generative models for medical imaging requires not only model innovation, but also careful dataset construction and evaluation. I treat this as a research problem in its own right.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Problem</h3>
      <p>
        Medical generative modeling is limited by fragmented datasets, variable preprocessing, and a lack of evaluation standards tied to anatomy and downstream utility.
      </p>
    </div>
    <div class="mini-card">
      <h3>Contribution</h3>
      <p>
        I have worked on curating and analyzing large-scale 3D brain MRI datasets for diffusion-model development, with emphasis on anatomical plausibility, data quality, and usefulness for reconstruction tasks.
      </p>
    </div>
    <div class="mini-card">
      <h3>Why it matters</h3>
      <p>
        This work strengthens the empirical foundation for medical generative imaging and supports a more rigorous pipeline from dataset design to clinically meaningful model evaluation.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Related publications</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Cross-cutting theme</div>
    <h2>Robustness across scanners, sites, and populations</h2>
    <p>
      Across reconstruction, harmonization, and downstream analysis, a recurring goal of my work is to reduce sensitivity to changes in scanner, site, acquisition protocol, and patient population.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Scientific question</h3>
      <p>
        How can we design imaging algorithms that remain reliable when the data distribution changes in ways that are inevitable in real-world clinical deployment?
      </p>
    </div>
    <div class="mini-card">
      <h3>Approach</h3>
      <p>
        I combine acquisition-aware modeling, robust learning, and structured priors to reduce unwanted sensitivity to scanner and protocol variation.
      </p>
    </div>
    <div class="mini-card">
      <h3>Program-level significance</h3>
      <p>
        This theme connects my current work to a broader faculty agenda centered on trustworthy and deployable computational imaging.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/publications/' | relative_url }}">Related publications</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Broader scope</div>
    <h2>Cross-modal collaborations in imaging AI</h2>
    <p>
      While MRI is the main technical center of my work, my collaborations span CT, OCT, pathology, ultrasound, and multimodal imaging. These collaborations broaden the questions I ask and reinforce a larger view of computational imaging as a discipline.
    </p>
  </div>

  <div class="mini-grid">
    <div class="mini-card">
      <h3>Role of collaboration</h3>
      <p>
        Working across modalities helps identify which ideas are MRI-specific and which reflect deeper principles in inverse problems, representation learning, and robust image analysis.
      </p>
    </div>
    <div class="mini-card">
      <h3>Research value</h3>
      <p>
        These collaborations extend the reach of my methods while also informing future directions in multimodal and multi-image computational imaging.
      </p>
    </div>
    <div class="mini-card">
      <h3>Faculty perspective</h3>
      <p>
        They also illustrate the collaborative style of lab I aim to build: technically deep, clinically connected, and open to new imaging domains.
      </p>
    </div>
  </div>

  <div class="links" style="margin-top: 1rem;">
    <a href="{{ '/service/' | relative_url }}">Professional engagement</a>
  </div>
</section>