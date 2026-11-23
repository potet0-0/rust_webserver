
const button = document.getElementById("welcomeBtn");
const message = document.getElementById("message");

let clicks = 0;

button.addEventListener("click", function() {
    clicks++;

    message.textContent = `You clicked the button ${clicks} times!`;

    if (clicks === 1) {
        button.textContent = "Click again!";
    } else if (clicks >= 5) {
        button.textContent = "You're having fun!";
    }
});
