<script>
        document.addEventListener('DOMContentLoaded', () => {
            const themeToggleButton = document.getElementById('theme-toggle');
            const htmlElement = document.documentElement;

            // Load saved theme from localStorage
            const savedTheme = localStorage.getItem('theme');
            if (savedTheme) {
                htmlElement.classList.add(savedTheme);
            } else {
                htmlElement.classList.add('theme-dark'); // Default theme
            }

            themeToggleButton.addEventListener('click', () => {
                if (htmlElement.classList.contains('theme-dark')) {
                    htmlElement.classList.remove('theme-dark');
                    htmlElement.classList.add('theme-light');
                    localStorage.setItem('theme', 'theme-light');
                } else {
                    htmlElement.classList.remove('theme-light');
                    htmlElement.classList.add('theme-dark');
                    localStorage.setItem('theme', 'theme-dark');
                }
            });
        });
</script>
