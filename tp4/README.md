# TP-04

# Etape 1 -> Build et test du pod yaml:

```bash
$ kubectl apply -f step1_pod.yaml
$ kubectl port-forward -n my-namespace [pod name] 9090:8080
```

Go on http://localhost:9090/ping

# Etape 2 -> Build et test du replica yaml:

```bash
$ kubectl apply -f step2_replica.yaml
$ kubectl port-forward -n my-namespace [replica name] 9090:8080
```

Go on http://localhost:9090/ping

# Etape 3 -> Build et test du deployment yaml:

```bash
$ kubectl apply -f step3_deployment.yaml
$ kubectl port-forward -n my-namespace [deployment name] 9090:8080
```

# Etape 4 -> Build et test du service yaml:

```bash
$ kubectl apply -f step4_service.yaml
$ kubectl port-forward -n my-namespace [service name] 8080:80
```

# Etape 5 -> Build et test du ingress yaml:

```bash
$ minikube addons enable ingress
$ kubectl apply -f step3_deployment.yaml
$ kubectl apply -f service.yaml
$ kubectl apply -f ingress.yaml
$ minikube ip
```

Add IP of minikube with your domain name in your /etc/hosts file:

```bash
$ sudo nano /etc/hosts
$ [minikube ip] myapp-domain.local
```

Go on http://myapp-domain.local/ping
