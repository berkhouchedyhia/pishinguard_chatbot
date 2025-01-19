use warp::Filter; // Pour gérer les routes HTTP et les filtres.
use serde::{Deserialize, Serialize}; // Pour la sérialisation/désérialisation JSON.
use std::sync::{Arc, Mutex}; // Pour gérer un état partagé entre plusieurs threads.
use std::collections::HashMap; // Pour gérer les paires question-réponse dynamiquement.

#[derive(Deserialize)] // Permet de convertir des données JSON entrantes en cette structure.
struct ChatRequest {
    message: String, // Le message envoyé par l'utilisateur.
}

#[derive(Serialize)] // Permet de convertir cette structure en JSON pour les réponses.
struct ChatResponse {
    reply: String, // La réponse générée par le bot.
}

#[tokio::main] // Indique que cette fonction principale est asynchrone.
async fn main() {
    // État partagé pour l'historique et les questions-réponses.
    let state = Arc::new(Mutex::new(AppState {
        history: Vec::new(),
        qa_pairs: HashMap::from([
            ("hello".to_string(), "Bonjour ! Comment puis-je vous aider aujourd'hui ?".to_string()),
            ("phishing".to_string(), "Les utilisateurs ayant cliqué sur des liens de phishing sont : Dyhia et Salah.".to_string()),
            ("tu connais rust".to_string(), "un langage de programmation moderne connu pour sa sécurité et ses performances.".to_string()),
            ("combien de contacts".to_string(), "contacts".to_string()),
            ("combien d'apis".to_string(), "apis".to_string()),
            ("combien de clics".to_string(), "clicks".to_string()),
            ("details sur la plateforme".to_string(), "platform".to_string()),
        ]),
    }));

    // Route POST pour /chat
    let chat_route = warp::path("chat") // Correspond à l'URL /chat.
        .and(warp::post()) // Accepte uniquement les requêtes POST.
        .and(warp::body::json()) // Attend un corps de requête au format JSON.
        .and(with_state(state.clone())) // Passe l'état partagé à la fonction de gestion.
        .and_then(handle_chat); // Appelle la fonction handle_chat pour gérer la requête.

    // Route pour ajouter de nouvelles questions-réponses dynamiquement.
    let add_qa_route = warp::path("add_qa") // Correspond à l'URL /add_qa.
        .and(warp::post())
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(add_question_answer);

    // Middleware CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method("POST")
        .allow_header("content-type");

    println!("Le serveur du chatbot est en cours d'exécution sur http://localhost:5501");

    // Lance le serveur avec les routes et CORS activé.
    warp::serve(chat_route.or(add_qa_route).with(cors))
        .run(([127, 0, 0, 1], 5501))
        .await;
}

// Structure pour stocker l'état de l'application.
struct AppState {
    history: Vec<String>,              // Historique des conversations.
    qa_pairs: HashMap<String, String>, // Paires question-réponse.
}

// Fonction pour partager l'état entre plusieurs requêtes.
fn with_state(
    state: Arc<Mutex<AppState>>,
) -> impl Filter<Extract = (Arc<Mutex<AppState>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone()) // Clone l'état pour chaque requête.
}

// Fonction pour gérer les requêtes POST vers /chat.
async fn handle_chat(
    request: ChatRequest, // Le message envoyé par l'utilisateur, extrait du JSON.
    state: Arc<Mutex<AppState>>, // L'état partagé.
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Requête reçue : /chat avec message = {}", request.message);
    let user_message = request.message.to_lowercase(); // Convertit le message en minuscules.

    // Récupérer la réponse associée à la question.
    let mut state = state.lock().unwrap(); // Verrouille l'état pour un accès exclusif.
    let bot_reply = state
        .qa_pairs
        .get(&user_message) // Cherche une réponse correspondante.
        .cloned() // Clone la réponse trouvée.
        .unwrap_or_else(|| "Je ne suis pas sûr de comprendre votre demande. Pouvez-vous clarifier ?".to_string());

    // Sauvegarde l'historique des messages.
    state.history.push(format!("Utilisateur : {}", request.message));
    state.history.push(format!("Bot : {}", bot_reply));

    // Retourne la réponse en JSON.
    Ok(warp::reply::json(&ChatResponse { reply: bot_reply }))
}
async fn query_platform(url: &str) -> Result<impl warp::Reply, warp::Rejection> {
    let client = Client::new();
    let platform_response = client.get(url).send().await;

    match platform_response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: serde_json::Value = resp.json().await.unwrap();
                Ok(warp::reply::json(&json))
            } else {
                Ok(warp::reply::json(&ChatResponse {
                    reply: "Erreur : la plateforme n'a pas pu traiter la requête.".to_string(),
                }))
            }
        }
        Err(_) => Ok(warp::reply::json(&ChatResponse {
            reply: "Erreur : impossible de se connecter à la plateforme.".to_string(),
        })),
    }
}



// Structure pour ajouter de nouvelles questions-réponses.
#[derive(Deserialize)]
struct AddQaRequest {
    question: String,
    answer: String,
}

// Fonction pour ajouter une paire question-réponse.
async fn add_question_answer(
    request: AddQaRequest,
    state: Arc<Mutex<AppState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Requête reçue : /add_qa avec question = {}, réponse = {}", request.question, request.answer);
    let mut state = state.lock().unwrap(); // Verrouille l'état pour un accès exclusif.
    state
        .qa_pairs
        .insert(request.question.to_lowercase(), request.answer);

    Ok(warp::reply::json(&"Question-Réponse ajoutée avec succès."))
}
