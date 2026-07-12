---
layout: home
title: Samuel W. Remedios
permalink: /
description: Physics-grounded AI for reliable medical imaging.
nav: false
---

<section class="home-hero">
  <div class="eyebrow">Machine learning researcher · Generative modeling &amp; inverse problems</div>
  <div class="hero-grid">
    <div>
      <h1>Generative modeling and inverse problems for ill-posed, data-scarce problems</h1>
      <p class="hero-lede">
        I build generative models that stay consistent with known physics and work when paired data is scarce. Ph.D. in Computer Science from Johns Hopkins; 60+ peer-reviewed publications across diffusion and generative models, image reconstruction and super-resolution, and self-supervised and zero-shot learning. My methods are developed and validated on medical imaging, but built on problem structure — ill-posed inverse problems, scarce paired data, physical forward operators — that transfers across scientific domains.
      </p>
      <div class="hero-meta">
        <span class="meta-pill">Diffusion &amp; generative models</span>
        <span class="meta-pill">Inverse problems</span>
        <span class="meta-pill">Self-supervised / zero-shot</span>
        <span class="meta-pill">PyTorch</span>
        <span class="meta-pill">Distributed HPC</span>
      </div>
      <div class="hero-actions">
        <a class="btn-hero" href="{{ '/projects/' | relative_url }}">Projects</a>
        <a class="btn-hero-secondary" href="{{ '/publications/' | relative_url }}">Publications</a>
        <a class="btn-hero-secondary" href="{{ '/assets/pdf/CurriculumVitae.pdf' | relative_url }}">Download CV</a>
        <a class="btn-hero-secondary" href="https://www.linkedin.com/in/s-remedios/">LinkedIn</a>
        <a class="btn-hero-secondary" href="https://github.com/sremedios">GitHub</a>
      </div>
    </div>

    <aside class="hero-card">
      <img src="/assets/img/headshot.jpg" alt="Samuel W. Remedios" class="headshot">
      <h3>At a glance</h3>
      <ul>
        <li>Builds generative and inverse-problem methods that hold up under real physical constraints</li>
        <li>Works in data-scarce regimes: self-supervised, zero-shot, and physics-informed methods</li>
        <li>Builds the high-performance training and tooling around the models — not just prototypes</li>
        <li>Ph.D. in Computer Science, Johns Hopkins; Postdoctoral Researcher (Prince lab)</li>
      </ul>
      <h3>Selected honors</h3>
      <div class="inline-stat-list">
        <span class="inline-stat">NSF GRFP</span>
        <span class="inline-stat">IPMI 2025 Best Poster</span>
        <span class="inline-stat">SASHIMI 2023 Best Paper</span>
      </div>
    </aside>
  </div>
</section>

<section class="home-section">
  <div class="section-kicker">What I work on</div>
  <h2>Generative modeling and inverse problems, built to transfer</h2>
  <p class="section-intro">
    I work at the intersection of machine learning, optimization, and signal processing. The through-line is generation under constraints: pairing data-driven generative priors with hard consistency to known physics, and making methods work when paired data is scarce. I develop and validate this on medical imaging, but the structure — ill-posed inverse problems, scarce paired data, physical forward operators — recurs across scientific domains.
  </p>

  <div class="glance-grid">
    <div class="glance-card">
      <div class="glance-year">Focus 1</div>
      <h3>Generative models and priors</h3>
      <p>Diffusion, flow, and other generative models used as learned priors — expressive enough to capture real structure, constrained enough to stay trustworthy.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Focus 2</div>
      <h3>Inverse problems and physics-informed inference</h3>
      <p>Reconstruction methods that embed the forward operator and enforce agreement with measured data, rather than relying on paired training sets alone.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Focus 3</div>
      <h3>Self-supervised and zero-shot learning</h3>
      <p>Methods that learn from structure within the data itself, so they hold up in the data-scarce regimes where matched training data does not exist.</p>
    </div>
    <div class="glance-card">
      <div class="glance-year">Domain</div>
      <h3>Medical and neuro imaging</h3>
      <p>Where the methods are proven: MRI reconstruction, super-resolution, and generative modeling under the real constraints of clinical acquisition.</p>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Where this is heading</div>
    <h2>Directions I'm pushing on</h2>
    <p>
      My work centers on combining physical measurement models with modern generative modeling to create methods that are not only accurate, but also structured, controllable, and reliable in deployment.
    </p>
  </div>
  <div class="thrust-grid">
    <div class="thrust-card">
      <div class="tag">Direction 1</div>
      <h3>Multi-image and multi-modal computational imaging</h3>
      <p>
        Develop structured generative frameworks that integrate scans across visits, contrasts, sites, and modalities for reconstruction, longitudinal analysis, and downstream clinical tasks.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Direction 2</div>
      <h3>Co-design of acquisition and reconstruction</h3>
      <p>
        Couple acquisition design with reconstruction through differentiable physics and learned priors to reduce scan time while improving image quality and access.
      </p>
    </div>
    <div class="thrust-card">
      <div class="tag">Direction 3</div>
      <h3>Generative priors for medicine</h3>
      <p>
        Build generative models that are anatomically plausible, controllable, pathology-aware, privacy-preserving, and aligned with the requirements of clinical deployment.
      </p>
    </div>
  </div>
  <div class="button-row">
    <a class="btn-hero-secondary" href="{{ '/research/' | relative_url }}">More on my research</a>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Selected contributions</div>
    <h2>Representative projects and technical contributions</h2>
    <p>
      These projects show how my research combines physical modeling, generative methods, and clinically motivated problem design.
    </p>
  </div>
  <div class="card-grid">
    <div class="feature-card">
      <div class="tag">MRI super-resolution</div>
      <h3>Zero-shot and operator-aware through-plane super-resolution</h3>
      <p>
        Developed internally trained and cycle-consistent methods for anisotropic MRI super-resolution that use acquisition operators directly, reducing dependence on paired high-resolution training data.
      </p>
      <!-- <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
    <div class="feature-card">
      <div class="tag">Inverse problems</div>
      <h3>Generative priors with data-consistent inference</h3>
      <p>
        Developed diffusion-based formulations for MRI inverse problems that use generative models as priors while explicitly enforcing consistency with measured observations.
      </p>
      <!-- <div class="links">
        <a href="{{ '/research/' | relative_url }}">Research context</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
    <div class="feature-card">
      <div class="tag">3D brain MRI</div>
      <h3>Large-scale data curation for generative imaging</h3>
      <p>
        Curated and analyzed large-scale 3D brain MRI datasets for diffusion-model development, with emphasis on anatomical plausibility and downstream utility for reconstruction tasks.
      </p>
      <!-- <div class="links">
        <a href="{{ '/projects/' | relative_url }}">Project overview</a>
        <a href="{{ '/publications/' | relative_url }}">Related papers</a>
      </div> -->
    </div>
  </div>
</section>

<section class="section-block">
  <div class="section-heading">
    <div class="kicker">Recognition and professional activity</div>
    <h2>Recent honors, invited talks, and service</h2>
  </div>
  <div class="timeline-list">
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>IPMI 2025 Best Poster Award</h3>
        <p>Awarded for <em>Cycle-Consistent Zero-Shot Through-Plane Super-Resolution for Anisotropic Head MRI</em>.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2025</div>
      <div>
        <h3>Invited talks in Japan</h3>
        <p>Presented invited talks on through-plane MRI super-resolution at the University of Tokyo and the Nara Institute of Science and Technology.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2024–26</div>
      <div>
        <h3>SASHIMI organizing committee</h3>
        <p>Serving on the organizing committee for SASHIMI at MICCAI across three consecutive years.</p>
      </div>
    </div>
        <div class="timeline-card">
      <div class="date">2026</div>
      <div>
        <h3>SynthOCT organizing committee</h3>
        <p>Serving on the organizing committee for the SynthOCT 2026 challenge at MICCAI.</p>
      </div>
    </div>
    <div class="timeline-card">
      <div class="date">2020–25</div>
      <div>
        <h3>NSF Graduate Research Fellowship</h3>
        <p>Supported independent research in AI for medical imaging through the National Science Foundation Graduate Research Fellowship Program.</p>
      </div>
    </div>
  </div>
</section>

<section class="section-block">
  <div class="mini-grid">
    <div class="mini-card">
      <h3>Community and peer review</h3>
      <p>
        I review for leading machine learning and imaging venues — including ICLR, CVPR, MICCAI, <em>IEEE Transactions on Image Processing</em>, and <em>Proceedings of the IEEE</em> — and help organize the SASHIMI workshop at MICCAI (2024–2026) and the SynthOCT 2026 challenge.
      </p>
    </div>
    <div class="mini-card">
      <h3>Get in touch</h3>
      <p>
        I'm always happy to talk about generative modeling, inverse problems, and hard real-world ML.
      </p>
      <div class="links">
        <a href="https://www.linkedin.com/in/s-remedios/">LinkedIn</a>
        <a href="mailto:samuel.remedios@jhu.edu">Email</a>
      </div>
    </div>
  </div>
</section>
