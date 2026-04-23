# Zadanie 1 - część dodatkowa
# Laboratorium Technologii chmurowych
## Wykonał Wojciech Cioczek

## Kod źródłowy

Zadanie zostało zrealizowane w języku Rust aby wygenerować obraz o jak najmniejszym rozmiarze.
Kod źródłowy programu zawarty jest w plku `src/main.rs` a wymagane przez aplikację zależności zdefiniowane zostały w pliku `Cargo.toml`.
Student aby zapoznać się z językiem Rust i wymaganymi przez aplikację zależnościami, wspierał się modelem LLM Gemini 🙂.
Wszystkie obrazy znajdują się w repozytorium https://hub.docker.com/r/sskew/rusty-weather/tags.

## Dockerfile

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
  CMD wget --quiet --tries=1 --spider http://127.0.0.1:8080/ || exit 1

# Nasłuchiwanie na porcie 8080
EXPOSE 8080

# Uruchomienie aplikacji
ENTRYPOINT ["/app"]
```

## Zadanie 3

1. Wykorzystanie buildera opartego na sterowniku docker-container
```bash
❯ docker buildx create --name mybuilder --driver=docker-container --use --bootstrap
[+] Building 3.4s (1/1) FINISHED                                                                                                              
 => [internal] booting buildkit                                                                                                          3.4s
 => => pulling image moby/buildkit:buildx-stable-1                                                                                       2.2s
 => => creating container buildx_buildkit_mybuilder0                                                                                     1.2s
mybuilder
❯ docker buildx ls
NAME/NODE           DRIVER/ENDPOINT     STATUS    BUILDKIT         PLATFORMS
mybuilder*          docker-container                               
 \_ mybuilder0       \_ desktop-linux   running   v0.29.0          linux/amd64 (+3), linux/arm64, linux/arm (+2), linux/ppc64le, (3 more)
default             docker                                         
 \_ default          \_ default         running   v0.0.0+unknown   linux/amd64 (+3), linux/386
desktop-linux       docker                                         
 \_ desktop-linux    \_ desktop-linux   running   v0.29.0          linux/amd64 (+3), linux/arm64, linux/arm (+2), linux/ppc64le, (2 more)
```
2. Dodanie osobistego klucza prywatnego do ssh-agent
```bash
❯ eval $(ssh-agent -s)
Agent pid 31002
❯ ssh-add ~/.ssh/gh6_labpl_25519
Identity added: /home/sskew/.ssh/gh6_labpl_25519 (sskew@proton.me)
```
3. Budowanie obrazu z wykorzystaniem danych cache
```bash
❯ docker buildx build \
--platform linux/amd64,linux/arm64 \
> -t sskew/rusty-weather:dod \
> --cache-to type=registry,ref=sskew/rusty-weather:cache,mode=max \
> --cache-from type=registry,ref=sskew/rusty-weather:cache \
> --ssh sskew=$HOME/.ssh/gh6_labpl_25519 \        
> -f Dockerfile_dod \      
> --progress=plain --push .
```
4. Potwierdzenie manifestu - obraz istnieje w wersji dla dwóch architektur
```bash
❯ docker buildx imagetools inspect sskew/rusty-weather:dod
Name:      docker.io/sskew/rusty-weather:dod
MediaType: application/vnd.oci.image.index.v1+json
Digest:    sha256:298b1cf17d6a8ab3b4d4a29f1b46b5eac3da96a67dbb2d6493d626109d7584e3
           
Manifests: 
  Name:        docker.io/sskew/rusty-weather:dod@sha256:0dbf9e5553d8dba9cd0037b7b22e36b3c92192e99b244909e11a7fdf21207025
  MediaType:   application/vnd.oci.image.manifest.v1+json
  Platform:    linux/amd64
               
  Name:        docker.io/sskew/rusty-weather:dod@sha256:e84ed62b2ca51e64e5acdd78a67d8cfe3e6ad2651dc17b5e057fc8e0a2e5e708
  MediaType:   application/vnd.oci.image.manifest.v1+json
  Platform:    linux/arm64
               
  Name:        docker.io/sskew/rusty-weather:dod@sha256:43fff747a2c67dbdec661548298c9c8dccdecc0e60aa6c4d73bea2d89a1cef48
  MediaType:   application/vnd.oci.image.manifest.v1+json
  Platform:    unknown/unknown
  Annotations: 
    vnd.docker.reference.digest: sha256:0dbf9e5553d8dba9cd0037b7b22e36b3c92192e99b244909e11a7fdf21207025
    vnd.docker.reference.type:   attestation-manifest
               
  Name:        docker.io/sskew/rusty-weather:dod@sha256:0e82dc337f838e8fe037aa93546c71b8afa65260710913b70b54d48eb633abb0
  MediaType:   application/vnd.oci.image.manifest.v1+json
  Platform:    unknown/unknown
  Annotations: 
    vnd.docker.reference.digest: sha256:e84ed62b2ca51e64e5acdd78a67d8cfe3e6ad2651dc17b5e057fc8e0a2e5e708
    vnd.docker.reference.type:   attestation-manifest
```
5. W repozytorium znaleźć też można cache wygenerowany przy tworzeniu obrazu
```bash
❯ docker buildx imagetools inspect sskew/rusty-weather:cache
Name:      docker.io/sskew/rusty-weather:cache
MediaType: application/vnd.oci.image.manifest.v1+json
Digest:    sha256:3bb4764482c53212e7a617896330d5a6fc44dc1ac4c082da330e1cf24488dfbc
```
5. Sprawdzenie podatności za pomocą Docker Scout. Obraz zawiera pojedynczą podatność medium która wynika z wersji pakietu wget. Obecnie nie ma załatanej wersji oprogramowania
```bash
❯ docker scout cves sskew/rusty-weather:dod
    ✓ SBOM of image already cached, 20 packages indexed
    ✓ Provenance obtained from attestation
    ✗ Detected 1 vulnerable package with 1 vulnerability


## Overview

                   │       Analyzed Image        
───────────────────┼─────────────────────────────
 Target            │  sskew/rusty-weather:dod    
   digest          │  298b1cf17d6a               
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
    View base image update recommendations → docker scout recommendations sskew/rusty-weather:dod
```