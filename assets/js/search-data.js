// get the ninja-keys element
const ninja = document.querySelector('ninja-keys');

// add the home and posts menu items
ninja.data = [{
    id: "nav-samuel-w-remedios",
    title: "Samuel W. Remedios",
    section: "Navigation",
    handler: () => {
      window.location.href = "/";
    },
  },{id: "nav-research",
          title: "Research",
          description: "How I think about generative modeling and inverse problems, what I&#39;ve built, and where the work is heading.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/research/";
          },
        },{id: "nav-projects",
          title: "Projects",
          description: "Selected projects in generative modeling, inverse problems, and self-supervised learning.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/projects/";
          },
        },{id: "nav-publications",
          title: "Publications",
          description: "Selected publications, full bibliography, and a record of work in medical imaging, inverse problems, and generative methods.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/publications/";
          },
        },{id: "nav-cv",
          title: "CV",
          description: "Full curriculum vitae, including publications, talks, teaching, service, and awards.",
          section: "Navigation",
          handler: () => {
            window.location.href = "/cv/";
          },
        },{id: "news-received-the-best-poster-award-at-ipmi-2025-for-work-on-cycle-consistent-zero-shot-through-plane-super-resolution-of-anisotropic-head-mri",
          title: 'Received the Best Poster Award at IPMI 2025 for work on cycle-consistent zero-shot...',
          description: "",
          section: "News",},{id: "news-gave-an-invited-talk-at-the-university-of-tokyo-on-through-plane-super-resolution-of-anisotropic-multi-slice-mri",
          title: 'Gave an invited talk at The University of Tokyo on through-plane super-resolution of...',
          description: "",
          section: "News",},{id: "news-presented-an-invited-talk-at-the-nara-institute-of-science-and-technology-on-through-plane-super-resolution-of-anisotropic-multi-slice-mri",
          title: 'Presented an invited talk at The Nara Institute of Science and Technology on...',
          description: "",
          section: "News",},{
        id: 'social-cv',
        title: 'CV',
        section: 'Socials',
        handler: () => {
          window.open("/assets/pdf/CurriculumVitae.pdf", "_blank");
        },
      },{
        id: 'social-email',
        title: 'email',
        section: 'Socials',
        handler: () => {
          window.open("mailto:%73%61%6D%75%65%6C.%72%65%6D%65%64%69%6F%73@%6A%68%75.%65%64%75", "_blank");
        },
      },{
        id: 'social-github',
        title: 'GitHub',
        section: 'Socials',
        handler: () => {
          window.open("https://github.com/sremedios", "_blank");
        },
      },{
        id: 'social-linkedin',
        title: 'LinkedIn',
        section: 'Socials',
        handler: () => {
          window.open("https://www.linkedin.com/in/s-remedios", "_blank");
        },
      },{
        id: 'social-scholar',
        title: 'Google Scholar',
        section: 'Socials',
        handler: () => {
          window.open("https://scholar.google.com/citations?user=H6TdLnoAAAAJ", "_blank");
        },
      },{
        id: 'social-x',
        title: 'X',
        section: 'Socials',
        handler: () => {
          window.open("https://twitter.com/sam_remedios", "_blank");
        },
      },];
