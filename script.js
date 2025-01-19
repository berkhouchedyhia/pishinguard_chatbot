// Sélection des éléments HTML
const form = document.getElementById("chat-form");
const chatLog = document.getElementById("chat-log");
const input = document.getElementById("user-message");

// Fonction pour afficher un message dans le journal
function addMessage(content, isUser) {
    const message = document.createElement("p");
    message.textContent = content;
    message.className = isUser ? "user-message" : "bot-reply";
    chatLog.appendChild(message);
    chatLog.scrollTop = chatLog.scrollHeight; // Faire défiler vers le bas
}

// Événement de soumission du formulaire
form.addEventListener("submit", async (e) => {
    e.preventDefault(); // Empêche le rechargement de la page
    const userMessage = input.value;

    // Ajouter le message de l'utilisateur à l'interface
    addMessage(`Vous: ${userMessage}`, true);

    // Envoyer le message au serveur.
    try {
        const response = await fetch("http://localhost:5501/chat", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ message: userMessage }),
        });
        const data = await response.json();

        // Afficher la réponse du bot
        addMessage(`Bot: ${data.reply}`, false);
    } catch (error) {
        addMessage("Erreur : impossible de joindre le serveur.", false);
    }
    // Réinitialiser le champ de saisie
    input.value = "";
});



