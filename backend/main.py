import requests

res = requests.post("http://127.0.0.1:3000/api/v1/download", json={"site": "youtube", "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"})

print(res.text)