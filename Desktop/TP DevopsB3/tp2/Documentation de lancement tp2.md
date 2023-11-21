# Déploiement de l'API avec Docker

1. **Créer un Dockerfile (partie 2 du tp) :**

```Dockerfile
FROM node:14-alpine


    COPY package*.json ./
    COPY tsconfig.json ./


    RUN npm install

    COPY . .

    RUN npm run build

    EXPOSE 8080

    CMD ["node", "build/index.js"]
```

2. **Construire l'image Docker :**

   ```bash
   docker build -t tp2 .
   ```

Ici, tp2 est le nom que j'ai donné a mon image docker

3. **Lancer le conteneur Docker :**

   ```bash
   docker run -p 8080:8080 -d tp2
   ```

## Scanner l'image Docker avec Trivy

1. **Installer Trivy :**

```
brew install trivy
```

2.  **Scanner l'image Docker et sauvegarder les résultats dans un fichier texte :**

        - Pour une image locale :

            ```bash
            trivy image tp2 > scan.txt
            ```

    Cette commande permet d'éxecuter le scan et stock le resultat du scan en locale sur le fichier scan.txt

3.  **Visualiser les résultats :**

    Après avoir exécuté la commande, ouvrez le fichier "scan.txt" pour consulter les résultats du scan Trivy.

4.  **Créer un Dockerfile (partie 2 du tp) :**

```Dockerfile

  # Stage 1: Build stage
FROM node:14-alpine AS build

WORKDIR /usr/src/app

COPY package*.json ./
COPY tsconfig.json ./

RUN npm install

COPY . .

RUN npm run build

# Stage 2: Execution stage
FROM node:14-alpine AS execution

WORKDIR /usr/src/app

# Créer un utilisateur non-root pour l'exécution du serveur web
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

# Copier uniquement les fichiers nécessaires pour l'exécution (build/ et node_modules/)
COPY --from=build /usr/src/app/build ./build
COPY --from=build /usr/src/app/node_modules ./node_modules

EXPOSE 8080

CMD ["node", "build/index.js"]
```

# Conclusion

L'image Docker est configurée pour exécuter le serveur web avec un utilisateur spécifique. De plus, une seconde image Docker optimisée a été créée, avec un stage de build et un stage d'exécution distincts, ce dernier excluant les sources du projet pour des raisons de sécurité et d'efficacité.
