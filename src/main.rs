use axum::{
    extract::Query,
    response::Html,
    routing::get,
    Router,
};
use chrono::Local;
use serde::Deserialize;
use std::net::SocketAddr;

// Struktura do deserializacji parametrów zapytania GET
#[derive(Deserialize)]
struct WeatherQuery {
    city: Option<String>,
    country: Option<String>,
}

// Głónwa funkcja wykorzystująca Tokio do asynchronicznego uruchomienia serwera HTTP
#[tokio::main]
async fn main() {
    let author = "Wojciech Cioczek";
    let port = 8080;
    // Generowanie logów przy uruchomieniu kontenera
    println!("Kontener uruchomiony o: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Autor programu: {}", author);
    println!("Aplikacja nasłuchuje na porcie TCP: {}", port);

    // Definiowanie ścieżek do obsługi żadań HTTP
    let app = Router::new()
        .route("/", get(handle_index))
        .route("/weather", get(handle_weather));

    // Definiowanie socketu na którym aplikacja będzie nasłuchiwać i uruchomienie serwera
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Funkcja generująca kod HTML fomularza
// Zapis z apostrofem oznacza, że zwracany string jest statyczny i nie będzie modyfikowany w czasie działania programu
// Zamiast listy, można sprawdzić pogodę dla dowolnego miasta, jednocześnie podające odpowiedni kod kraju
async fn handle_index() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head><title>Pogodynka Rust</title></head>
        <body>
            <h1>Wybierz lokalizację</h1>
            <form action="/weather" method="get">
                Miasto: <input type="text" name="city" required>
                <br><br>
                Kod kraju (np. PL): <input type="text" name="country" required>
                <button type="submit">Sprawdź pogodę</button>
            </form>
        </body>
        </html>
    "#)
}

// Funkcja obsługuje zapytanie GET do api OpenWeatherMap
// Jeśli użytkownik nie poda miasta lub kraju, domyślnie sprawdzana jest pogoda dla Warszawy
async fn handle_weather(Query(params): Query<WeatherQuery>) -> Html<String> {
    let api_key = "7a04a4cda6c61f02409dad1f6f7c4c9b";
    let city = params.city.unwrap_or_else(|| "Warsaw".to_string());
    let country = params.country.unwrap_or_else(|| "PL".to_string());

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric&lang=pl",
        city, country, api_key
    );

    match reqwest::get(&url).await {
        // Jeśli odpowiedź jest poprawna, parsowany jest JSON i generowany HTML z informacjami o pogodzie
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                let temp = json["main"]["temp"].as_f64().unwrap_or(0.0);
                let desc = json["weather"][0]["description"].as_str().unwrap_or("brak danych");
                
                Html(format!(
                    "<h1>Pogoda dla: {}, {}</h1><p>Temperatura: {}°C</p><p>Opis: {}</p><a href='/'>Powrót</a>",
                    city, country, temp, desc
                ))
            } else {
                Html("<h1>Błąd parsowania danych API</h1>".into())
            }
        }
        // Jeśli wystąpi błąd podczas połączenia z API, zwracany jest komunikat o błędzie
        Err(_) => Html("<h1>Nie udało się połączyć z OpenWeatherMap</h1>".into()),
    }
}