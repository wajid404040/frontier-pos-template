// Sidebar toggle for mobile
document.addEventListener('DOMContentLoaded', function() {
  // Mobile menu toggle (if needed)
  const sidebar = document.querySelector('.sidebar');
  const mainContent = document.querySelector('.main-content');
  
  // Highlight active nav item based on current page
  const currentPath = window.location.pathname;
  const navItems = document.querySelectorAll('.nav-item');
  
  navItems.forEach(item => {
    const href = item.getAttribute('href');
    if (currentPath.endsWith(href) || (href === 'index.html' && currentPath.endsWith('/'))) {
      item.classList.add('nav-item--active');
    } else {
      item.classList.remove('nav-item--active');
    }
  });
  
  // Smooth scroll for anchor links
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
      e.preventDefault();
      const target = document.querySelector(this.getAttribute('href'));
      if (target) {
        target.scrollIntoView({
          behavior: 'smooth',
          block: 'start'
        });
      }
    });
  });
});

