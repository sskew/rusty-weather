# Rusty Weather - CI/CD Pipeline z użyciem GitHub Actions
## Zadanie 2
## Laboratorium Technologii Chmurowych

Niniejsze repozytorium zawiera prosty, zautomatyzowany pipeline CI/CD dla aplikacji z zadania pierwszego. Proces buduje obraz kontenera, skanuje go pod kątem luk bezpieczeństwa (CVE) i publikuje w publicznym rejestrze GitHub Container Registry (GHCR). 

## Realizacja wymagań projektu

Łańcuch CI/CD został zaprojektowany z zachowaniem najlepszych praktyk i spełnia następujące warunki brzegowe:

* **Wsparcie dla wielu architektur** Dzięki wykorzystaniu akcji `docker/setup-qemu-action` oraz `docker/setup-buildx-action`, finalny obraz kompilowany jest równolegle na dwie platformy: `linux/amd64` oraz `linux/arm64`.
* **Zewnętrzny system Cache** Proces budowania wykorzystuje zewnętrzne repozytorium na DockerHub jako backend dla pamięci podręcznej (eksporter typu `registry`). Zastosowano tryb `mode=max`.
* **Bramka bezpieczeństwa CVE** Aplikacja skanowana jest za pomocą **Trivy**. Użycie flagi `exit-code: '1'` oraz ograniczenie do podatności `CRITICAL,HIGH` gwarantuje, że wykrycie poważnych luk natychmiast przerywa działanie pipeline'u.

## Schemat tagowania

Zarządzanie tagami obrazów zostało zautomatyzowane przy pomocy oficjalnej akcji `docker/metadata-action`.

### Obrazy produkcyjne (GHCR)
Schemat tagowania oparto na metodologii Wersjonowania Semantycznego (SemVer) opisanej w artykule. Takie podejście stanowi standard ułatwiający zarządzanie cyklem życia aplikacji. W zależności od zdarzenia wyzwalającego w repozytorium, akcja nadaje odpowiednie tagi:
* **Commit na gałąź (Push)** Nadawany jest skrócony hash commitu oraz nazwa gałęzi. Jest to niezbędne dla programistów w procesie debugowania (pozwala łatwo powiązać konkretny obraz z dokładnym miejscem w kodzie).
* **Tagi wydań (Releases)** Kiedy w repozytorium utworzony zostaje tag zgodny z wzorcem SemVer (np. `v1.2.3`), system automatycznie generuje tagi dla konkretnej wersji oraz jej podwersji (np. `1.2.3`, `1.2`, `1`, `latest`). Gwarantuje to użytkownikom końcowym, że pobierając obraz z tagiem `1`, zawsze otrzymają najnowszą, bezpieczną, wstecznie kompatybilną łatkę z danej serii.

#### Źródło: https://medium.com/@jaredhatfield/publishing-semantic-versioned-docker-images-to-github-packages-using-github-actions-ebe88fa74522
