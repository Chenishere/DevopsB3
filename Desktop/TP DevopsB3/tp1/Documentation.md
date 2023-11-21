# TP1 - Devops B3
## Documentation de Lancement du Projet

Cette documentation fournit des instructions sur la manière de lancer le projet et d'interagir avec l'API.

## Configuration du Projet

1. **Cloner le Repository:**
   ```bash
   git clone https://github.com/Chenishere/DevopsB3.git
   ```
   ```
   cd DevopsB3
   ```
2. **Installer les Dépendances:**


   Assurez-vous d'avoir Node.js installé, puis exécutez la commande suivante pour installer les dépendances :
   ```
   npm install
   ```
3. **Definir le port:**


Vous pouvez définir le port d'écoute en utilisant la variable d'environnement PING_LISTEN_PORT. Par défaut, le serveur écoutera sur le port 80.
   ```
   export PING_LISTEN_PORT=80
```

4. **Lancer le serveur:**
```
npx ts-node src/app.ts
```
Si le serveur est bien lancé, le terminal affichera :
```
Server is running on port 80
```

5. **Tester l'API:**
Effectuer une Requête GET /ping:


Vous pouvez tester l'API en effectuant une requête GET sur le chemin /ping. Cela renverra les en-têtes de la requête au format JSON.
```
curl http://localhost:80/ping
```
Qui retournera au format JSON les headers de la requête quand il y une requête HTTP GET sur /ping : 
```
{"headers":{"host":"localhost","user-agent":"curl/8.1.2","accept":"*/*"}}%
```
6. **Gestion d'erreur:**



Réponse vide avec code 404 si quoi que ça soit d'autre que GET /ping (dans l'exemple si dessous /test donc erreur 404)
```
curl http://localhost:80/test
```
Qui retourne comme prévu une erreur 404 : 
```
{"error":"404"}%
```



![Texte alternatif](https://customersfirstacademy.com/wp-content/uploads/2021/04/strategies-for-Creating-a-Happy-Customer-1024x576.jpg)



