import requests

r = requests.get("http://127.0.0.1:8080/api/v1/workflows")
print(r.json())
