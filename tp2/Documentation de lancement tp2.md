# Déploiement de l'API avec Docker

## Construction de l'image Docker

1. **Créer un Dockerfile :**

    ```Dockerfile

    FROM node:14-alpine

    WORKDIR /usr/src/app

    COPY package*.json ./
    COPY tsconfig.json ./

    RUN npm install

    COPY . .

    RUN npm run build

    EXPOSE 8080

    CMD ["node", "build/index.js"]
    ```


3. **Construire l'image Docker :**

    ```bash
    docker build -t tp2 .
    ```


Ici, tp2 est le nom que j'ai donné a mon image docker

4. **Lancer le conteneur Docker :**

    ```bash
    docker run -p 8080:8080 -d tp2
    ```

## Scanner l'image Docker avec Trivy

1. **Installer Trivy :**

```
brew install trivy
```


2. **Scanner l'image Docker et sauvegarder les résultats dans un fichier texte :**

    - Pour une image locale :

        ```bash
        trivy image tp2 > scan.txt
        ```
Cette commande permet d'éxecuter le scan et stock le resultat du scan en locale sur le fichier scan.txt

3. **Visualiser les résultats :**

    Après avoir exécuté la commande, ouvrez le fichier "scan.txt" pour consulter les résultats du scan Trivy.
