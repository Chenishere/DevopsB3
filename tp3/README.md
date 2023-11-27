# Api DevOps

Api DevOps est une application Node.js qui écoute sur un port spécifique et renvoie les en-têtes HTTP lorsqu'une requête GET est effectuée sur l'URL `/ping`.

## Lancement des contenaires docker

Le dockerfile 1 avec un seul stage :

```bash
docker build -t [my-name] .
docker run -d -p 2000:2000 [my-name]
```

Le dockerfile 2 avec deux stages :

```bash
docker build -f Dockerfile2 -t [my-name] .
docker run -d -p 2000:2000 [my-name]
```


## Lancement docker compose 
    
```bash
docker-compose up -d
```

Accéder à l'api via `http://127.0.0.1:[9090]/ping`