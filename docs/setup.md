# Spuštění

## Vytvoření docker sítě

```bash
docker network create -d bridge traefik_proxy
```

## Traefik

```bash
docker compose -f docker/traefik/docker-compose.yml up
```

Traefik běží na portu 80

## Testy

### Přihlášení

Použijte `docker login` se svými přihlašovacími údaji

```bash
docker login
```

S GitHub tokenem

```bash
export CR_PAT=TOKEN_TU
echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin
```

### Spuštění

Nakonfigurujte šablonu: `docker/portfolio/docker-compose.yml` a spusťte ji

```bash
docker compose -f docker/portfolio/docker-compose.yml up
```

 - Migrace se spustí automaticky