import requests;

headers = {
    "name": "5-23",
    "age": "17",
    "tell": "010-1234-5678"
}

res = requests.get("http://127.0.0.1:8080/headers", headers = headers)