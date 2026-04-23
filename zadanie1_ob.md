# Zadanie 1 - część obowiązkowa
# Laboratorium Technologii chmurowych
## Wykonał Wojciech Cioczek

## Zadanie 1

Zadanie zostało zrealizowane w języku Rust aby wygenerować obraz o jak najmniejszym rozmiarze.
Kod źródłowy programu zawarty jest w plku `src/main.rs` a wymagane przez aplikację zależności zdefiniowane zostały w pliku `Cargo.toml`.
Student aby zapoznać się z językiem Rust i wymaganymi przez aplikację zależnościami, wspierał się modelem LLM Gemini 🙂.
Wszystkie obrazy znajdują się w repozytorium https://hub.docker.com/r/sskew/rusty-weather/tags.

## Zadanie 2

Zawartość Dockerfile wraz komentarzami znajduje się poniżej oraz w pliku źródłowym.

```dockerfile
# syntax=docker/dockerfile:1

# ETAP 1
# Wykorzystuję obraz Rust z Alpine oraz dwa etapy budowania, aby zmniejszyć rozmiar obrazu
# Pobieram obraz z najnowszą wersją Rust
FROM rust:alpine AS builder

# Instalacja narzędzi do kompilacji statycznej Rust
# Biblioteka musl-dev jest najmniejszą biblioteką C, która pozwala na kompilację statyczną i zmniejszenie rozmiaru pliku binarnego
RUN apk add --no-cache musl-dev

# Ustalenie katalogu roboczego i kopiowanie kodu źródłowego
WORKDIR /app
COPY . .

# Kompilacja programu z flagą --release, która włącza optymalizacje i jeszcze zmniejsza rozmiar pliku binarnego
# Dodatkowo target kompilacji każe kompilować program pod odpowiednią architekturę linkuksową i nakazuje wykorzystać bibliotkę musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# ETAP 2
# Aplikacja jest w stanie działać na obrazie scratch, ale obraz ten posiada zbyt małą funkcjonalność aby uruchomić healthcheck w tej postaci lub utworzyć użytkownika
FROM alpine:latest

# Metadane zgodne z OCI
LABEL org.opencontainers.image.title="rusty-weather"
LABEL org.opencontainers.image.authors="Wojciech Cioczek"
LABEL org.opencontainers.image.description="Prosta aplikacja pogodowa stworzona w Rust"

# Kod skopiowany z generycznego pliku Dockerfile, dzięki niemu kontener nie używa konta root i podniesione jest bezpieczeństwo kontenera
# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Kopiowanie już skompilowanego w poprzednim etapie pliku binarnego
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rusty-weather /app

# Healthcheck sprawdzający, czy aplikacja nasłuchuje na porcie 8080
HEALTHCHECK --interval=30s --timeout=3s \
  CMD wget --quiet --tries=1 --spider http://localhost:8080/ || exit 1

# Nasłuchiwanie na porcie 8080
EXPOSE 8080

# Uruchomienie aplikacji
ENTRYPOINT ["/app"]
```

## Zadanie 3

1. Budowanie obrazu
```bash
docker build -f Dockerfile_ob -t rusty-weather:ob .
```
2. Uruchomienie kontenera
```bash
❯ docker run -d -p 8080:8080 --name rusty-weather-container rusty-weather:ob
0942fd4cd5ebf33d36bb4c95824b0d4533fa977a9ac9fb7acb1dcfa91c5ed5d8
```
3. Podejrzenie logów
```bash
❯ docker logs rusty-weather-container
Kontener uruchomiony o: 2026-04-22 18:48:35
Autor programu: Wojciech Cioczek
Aplikacja nasłuchuje na porcie TCP: 8080
```
5. Obraz składa się z 12 warstw i ma łączny rozmiar 16.9MB.
```bash
❯ docker history rusty-weather:ob
IMAGE          CREATED          CREATED BY                                      SIZE      COMMENT
7818ff2f9a58   46 minutes ago   ENTRYPOINT ["/app"]                             0B        buildkit.dockerfile.v0
<missing>      46 minutes ago   EXPOSE [8080/tcp]                               0B        buildkit.dockerfile.v0
<missing>      46 minutes ago   HEALTHCHECK &{["CMD-SHELL" "wget --quiet --t…   0B        buildkit.dockerfile.v0
<missing>      46 minutes ago   COPY /app/target/x86_64-unknown-linux-musl/r…   2.59MB    buildkit.dockerfile.v0
<missing>      6 hours ago      USER appuser                                    0B        buildkit.dockerfile.v0
<missing>      6 hours ago      RUN |1 UID=10001 /bin/sh -c adduser     --di…   32.8kB    buildkit.dockerfile.v0
<missing>      6 hours ago      ARG UID=10001                                   0B        buildkit.dockerfile.v0
<missing>      6 hours ago      LABEL org.opencontainers.image.description=P…   0B        buildkit.dockerfile.v0
<missing>      6 hours ago      LABEL org.opencontainers.image.authors=Wojci…   0B        buildkit.dockerfile.v0
<missing>      6 hours ago      LABEL org.opencontainers.image.title=rusty-w…   0B        buildkit.dockerfile.v0
<missing>      6 days ago       CMD ["/bin/sh"]                                 0B        buildkit.dockerfile.v0
<missing>      6 days ago       ADD alpine-minirootfs-3.23.4-x86_64.tar.gz /…   9.11MB    buildkit.dockerfile.v0
❯ docker images rusty-weather:ob
                                                                                                                          i Info →   U  In Use
IMAGE              ID             DISK USAGE   CONTENT SIZE   EXTRA
rusty-weather:ob   7818ff2f9a58       16.9MB          5.2MB    U  
```
6. Sprawdzenie podatności za pomocą Docker Scout. Obraz zawiera pojedynczą podatność medium która wynika z wersji pakietu wget. Obecnie nie ma załatanej wersji oprogramowania
```
❯ docker scout cves sskew/rusty-weather:ob
    ✓ Image stored for indexing
    ✓ Indexed 20 packages
    ✓ Provenance obtained from attestation
    ✗ Detected 1 vulnerable package with 1 vulnerability


## Overview

                   │       Analyzed Image        
───────────────────┼─────────────────────────────
 Target            │  sskew/rusty-weather:ob     
   digest          │  7818ff2f9a58               
   platform        │ linux/amd64                 
   vulnerabilities │    0C     0H     1M     0L  
   size            │ 5.2 MB                      
   packages        │ 20                          


## Packages and Vulnerabilities

   0C     0H     1M     0L  busybox 1.37.0-r30
pkg:apk/alpine/busybox@1.37.0-r30?os_name=alpine&os_version=3.23

    ✗ MEDIUM CVE-2025-60876
      https://scout.docker.com/v/CVE-2025-60876
      Affected range : <=1.37.0-r30 
      Fixed version  : not fixed    
    


1 vulnerability found in 1 package
  CRITICAL  0 
  HIGH      0 
  MEDIUM    1 
  LOW       0 


What's next:
    View base image update recommendations → docker scout recommendations sskew/rusty-weather:ob

```