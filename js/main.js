

document.addEventListener('DOMContentLoaded', () => {
    const downloadButton = document.getElementById('downloadButton');
    if (downloadButton) {
        downloadButton.addEventListener('click', function(e) {
            e.preventDefault(); // Prevent the default button click action

            const videoUrl = document.getElementById('videoUrl').value;
            fetch('http://127.0.0.1:3000/api/v1/download', {
                method: 'POST',
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ site: "youtube", url: videoUrl })
            })
            .then(response => console.log(response.status))
            .catch(error => {
                console.error('Error:', error);
            });
        })
    }
});







document.addEventListener("DOMContentLoaded", function() {
    var input = document.querySelector('.input__hero');
    var icon = document.querySelector('.hero__icon__download');

    input.addEventListener('input', function() {
        if (input.value.trim() !== '') {
            icon.classList.add('shake');
        } else {
            icon.classList.remove('shake');
        }
    });
});